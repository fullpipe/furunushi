use std::sync::{Arc, Mutex};
use std::usize;

use anyhow::{bail, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{self, FromSample, Sample, Stream};
use log::{error, info, trace};
use pitch_detection::detector::autocorrelation::AutocorrelationDetector;
use pitch_detection::detector::yin::YINDetector;
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, RingBuffer};

use crossbeam::channel::{bounded, Receiver, Sender};
use serde::Serialize;
use ts_rs::TS;

pub mod commands;

const POWER_THRESHOLD: f32 = 2.0;
const CLARITY_THRESHOLD: f32 = 0.6;
const DEFAULT_BASE: f32 = 440.0;

#[derive(Debug, Clone, Copy, Serialize, TS)]
#[ts(export, export_to = "../../src/app/bindings/")]
pub struct Note {
    name: NoteName,
    octave: u32,
    frequency: f32,
    midi: u32,
    deviation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../src/app/bindings/")]
enum NoteName {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

const NOTE_BY_IDX: [NoteName; 12] = [
    NoteName::C,
    NoteName::Cs,
    NoteName::D,
    NoteName::Ds,
    NoteName::E,
    NoteName::F,
    NoteName::Fs,
    NoteName::G,
    NoteName::Gs,
    NoteName::A,
    NoteName::As,
    NoteName::B,
];

fn note(rf: f32, f: f32) -> Result<Note> {
    let k = 12f32 * (f / rf).log2();
    let m = k.round() as i32;
    let n = 69 + m;
    let octave = n / 12 - 1;
    let frequency = rf * 2.0f32.powf((n as f32 - 69.0) / 12.0);
    let deviation = (1200f32 * (f / frequency).log2()).round() as i32;
    let idx: usize = (n % 12).try_into()?;

    Ok(Note {
        name: NOTE_BY_IDX[idx],
        octave: octave.try_into()?,
        frequency,
        midi: n.try_into()?,
        deviation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_base_note() {
        let n = note(440.0, 440.0).unwrap();

        assert_eq!(n.name, NoteName::A);
        assert_eq!(n.frequency, 440.0);
        assert_eq!(n.octave, 4u32);
        assert_eq!(n.midi, 69u32);
        assert_eq!(n.deviation, 0);

        let n = note(440.0, 442.0).unwrap();
        assert_eq!(n.deviation, 8);

        let n = note(442.0, 440.0).unwrap();
        assert_eq!(n.deviation, -8);
    }

    #[test]
    fn it_detects_notes() {
        let n = note(440.0, 523.0).unwrap();

        assert_eq!(n.name, NoteName::C);
        assert_eq!(n.frequency, 523.2511);
        assert_eq!(n.octave, 5u32);
        assert_eq!(n.midi, 72u32);
        assert_eq!(n.deviation, -1);
    }
}

pub struct DetectorState {
    pub controls_sender: Sender<Control>,
    pub controls_receiver: Receiver<Control>,
    pub data_sender: Sender<Note>,
    pub data_receiver: Receiver<Note>,
}

impl DetectorState {
    pub fn new() -> Self {
        let (controls_sender, controls_receiver) = bounded(1);
        let (data_sender, data_receiver) = bounded(1);

        return Self {
            controls_sender,
            controls_receiver,
            data_sender,
            data_receiver,
        };
    }
}

pub enum Control {
    Pause,
    Start,
    Stop,
    Base(f32),
}

pub async fn init(controls: Receiver<Control>, sender: Sender<Note>) -> Result<()> {
    let host: cpal::Host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("[Microphone] No output devices available");

    info!("{}", device.name().unwrap());

    let config = device
        .default_input_config()
        .expect("Failed to get default config");

    let err_fn = move |err| {
        error!("an error occurred on stream: {}", err);
    };

    let sample_rate: usize = config.sample_rate().0.try_into()?;
    let bb: WavWriterHandle = Mutex::new(AllocRingBuffer::<f32>::new(sample_rate / 8));
    let base = Arc::new(Mutex::new(DEFAULT_BASE));
    let dbase = Arc::clone(&base);

    let stream: Stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, &bb, sample_rate, &dbase, &sender),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| {
                write_input_data::<i16, i16>(data, &bb, sample_rate, &dbase, &sender)
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| {
                write_input_data::<i32, i32>(data, &bb, sample_rate, &dbase, &sender)
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| {
                write_input_data::<f32, f32>(data, &bb, sample_rate, &dbase, &sender)
            },
            err_fn,
            None,
        )?,
        sample_format => bail!("Unsupported sample format '{sample_format}'"),
    };

    stream.pause();

    while let Ok(control) = controls.recv() {
        match control {
            Control::Start => {
                stream.play();
            }
            Control::Pause => {
                stream.pause();
            }
            Control::Base(f) => {
                *base.lock().unwrap() = f;
            }
            Control::Stop => break,
        }
    }

    stream.pause()?;
    drop(stream);

    Ok(())
}

type WavWriterHandle = Mutex<AllocRingBuffer<f32>>;

fn write_input_data<T, U>(
    input: &[T],
    bb: &WavWriterHandle,
    sample_rate: usize,
    base: &Arc<Mutex<f32>>,
    s: &Sender<Note>,
) where
    T: Sample,
    U: Sample + FromSample<T>,
    f32: FromSample<T>,
{
    use pitch_detection::detector::mcleod::McLeodDetector;
    use pitch_detection::detector::PitchDetector;

    let signal: Vec<f32> = input.iter().map(|x| f32::from_sample(*x)).collect();
    let mut data = bb.lock().unwrap();
    data.extend(signal);

    if !data.is_full() {
        return;
    }

    // let mut detector = McLeodDetector::new(data.len(), data.len() / 2);
    // let mut detector = YINDetector::new(data.len(), data.len() / 2);
    let mut detector = AutocorrelationDetector::new(data.len(), data.len() / 2);

    match detector.get_pitch(
        data.to_vec().as_slice(),
        sample_rate,
        POWER_THRESHOLD,
        CLARITY_THRESHOLD,
    ) {
        None => trace!("no pitch"),
        Some(p) => match note(*base.lock().unwrap(), p.frequency) {
            Err(e) => error!("{}", e),
            Ok(n) => {
                // TODO: handle disconect
                s.try_send(n);
            }
        },
    }

    data.clear();
}
