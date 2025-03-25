//! Make some noise via cpal.
#![allow(clippy::precedence)]

use anyhow::{bail, Result};
use assert_no_alloc::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample, Stream};
use crossbeam::channel::Receiver;
use fundsp::hacker::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod commands;
mod organ;
mod sine;
pub mod state;

pub enum Control {
    Play(Drone),
    Pause,
    Stop,
    Volume(f32),
}

pub fn init(controls: Receiver<Control>) -> Result<()> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("Failed to find a default output device");

    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f64;
    let channels = config.channels() as usize;
    let volume = shared(1.0);

    let (mut slot, mut sb) = Slot::new(Box::new(zero() >> pan(0.0)));
    let mut next_value = move || assert_no_alloc(|| sb.get_stereo());
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream: Stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_output_stream(
            &config.into(),
            move |data, _| write_data::<i8>(data, channels, &mut next_value),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config.into(),
            move |data, _| write_data::<i16>(data, channels, &mut next_value),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_output_stream(
            &config.into(),
            move |data, _| write_data::<i32>(data, channels, &mut next_value),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data, _| write_data::<f32>(data, channels, &mut next_value),
            err_fn,
            None,
        )?,
        sample_format => bail!("Unsupported sample format '{sample_format}'"),
    };

    stream.pause();

    while let Ok(control) = controls.recv() {
        match control {
            Control::Play(d) => {
                stream.play();
                slot.set(Fade::Smooth, 1.0, build_drone(d, &volume, sample_rate));
            }
            Control::Pause => {
                slot.set(Fade::Smooth, 1.0, Box::new(zero() >> pan(0.0)));
                std::thread::sleep(std::time::Duration::from_millis(1000));
                stream.pause();
            }
            Control::Volume(f) => {
                volume.set(f);
            }
            Control::Stop => break,
        }
    }

    slot.set(Fade::Smooth, 0.4, Box::new(zero() >> pan(0.0)));
    std::thread::sleep(std::time::Duration::from_millis(400));

    stream.pause()?;
    drop(stream);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/app/bindings/")]
pub struct Drone {
    pub midi: u32,
    pub tuning: f32,
    pub instrument: Instrument,
    pub chord: Chord,
    pub chorus: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/app/bindings/")]
pub enum Chord {
    Pure,
    Minor,
    Major,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/app/bindings/")]
pub enum Instrument {
    Organ,
    Sine,
}

fn build_drone(drone: Drone, volume: &Shared, sample_rate: f64) -> Box<dyn AudioUnit> {
    match drone.instrument {
        Instrument::Organ => match drone.chord {
            Chord::Minor => match drone.chorus {
                true => organ::organ_minor_chorus(drone.midi, drone.tuning, volume, sample_rate),
                false => organ::organ_minor(drone.midi, drone.tuning, volume, sample_rate),
            },
            Chord::Major => match drone.chorus {
                true => organ::organ_major_chorus(drone.midi, drone.tuning, volume, sample_rate),
                false => organ::organ_major(drone.midi, drone.tuning, volume, sample_rate),
            },
            Chord::Pure => match drone.chorus {
                true => organ::organ_pure_chorus(drone.midi, drone.tuning, volume, sample_rate),
                false => organ::organ_pure(drone.midi, drone.tuning, volume, sample_rate),
            },
        },
        Instrument::Sine => match drone.chord {
            Chord::Minor => match drone.chorus {
                true => sine::sine_minor_chorus(drone.midi, drone.tuning, volume, sample_rate),
                false => sine::sine_minor(drone.midi, drone.tuning, volume, sample_rate),
            },
            Chord::Major => match drone.chorus {
                true => sine::sine_major_chorus(drone.midi, drone.tuning, volume, sample_rate),
                false => sine::sine_major(drone.midi, drone.tuning, volume, sample_rate),
            },
            Chord::Pure => match drone.chorus {
                true => sine::sine_pure_chorus(drone.midi, drone.tuning, volume, sample_rate),
                false => sine::sine_pure(drone.midi, drone.tuning, volume, sample_rate),
            },
        },
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<()>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    //let c = mls();
    // let c = (mls() | dc(400.0) | dc(50.0)) >> resonator();
    // let c = pink();

    // FM synthesis.
    //let f = 110.0;
    //let m = 5.0;
    //let c = oversample(sine_hz(f as f64) * f * m + f >> sine());

    // Pulse wave.
    // let c = lfo(|t| {
    //     let pitch = 220.0;
    //     let width = lerp11(0.01, 0.99, sin_hz(0.05, t));
    //     (pitch, width)
    // }) >> pulse();

    // let c = zero() >> pluck(220.0, 0.8, 0.8);
    // let c = dc(110.0) >> dsf_saw_r(0.99);
    //let c = dc(110.0) >> triangle();
    //let c = dc(110.0) >> soft_saw();
    //let c = lfo(|t| xerp11(20.0, 2000.0, sin_hz(0.1, t))) >> dsf_square_r(0.99) >> lowpole_hz(1000.0);
    //let c = dc(110.0) >> square();

    // let c =
    // organ_hz(midi_hz(69.0)) + 0.5 * organ_hz(midi_hz(73.0)) + 0.25 * organ_hz(midi_hz(76.0));
    // let c = hammond_hz(midi_hz(69.0))
    //     + 0.5 * hammond_hz(midi_hz(73.0))
    //     + 0.25 * hammond_hz(midi_hz(76.0));

    let c = sine_hz(midi_hz(69.0)) & 0.5 * sine_hz(midi_hz(73.0)) & 0.2 * sine_hz(midi_hz(76.0));

    // let c = triangle_hz(midi_hz(69.0))
    //     & 0.5 * triangle_hz(midi_hz(73.0))
    //     & 0.2 * triangle_hz(midi_hz(76.0));

    //let c = dc(440.0) >> rossler();
    //let c = dc(110.0) >> lorenz();
    //let c = organ_hz(110.1) + organ_hz(54.9);
    //let c = pink() >> hold_hz(440.0, 0.0);

    // Filtered noise tone.
    //let c = (noise() | dc((440.0, 50.0))) >> !resonator() >> resonator();

    // Test ease_noise.
    // let c = lfo(|t| xerp11(50.0, 5000.0, ease_noise(smooth9, 0, t))) >> triangle();

    // Bandpass filtering.
    //let c = c >> (pass() | envelope(|t| xerp11(500.0, 5000.0, sin_hz(0.05, t)))) >> bandpass_q(5.0);
    //let c = c >> (pass() | envelope(|t| (xerp11(500.0, 5000.0, sin_hz(0.05, t)), 0.9))) >> !bandrez() >> bandrez();

    // Waveshaper.
    // let c = c >> shape(Crush(20.0));

    // Add feedback delay.
    //let c = c >> (pass() & feedback(butterpass_hz(1000.0) >> delay(1.0) * 0.5));

    // Apply Moog filter.
    // let c = (c | lfo(|t| (xerp11(110.0, 11000.0, sin_hz(0.1, t)), 0.6))) >> moog();

    let c = c >> pan(0.0);

    //let c = fundsp::sound::risset_glissando(false);

    // Add chorus.
    let c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    // Add flanger.
    //let c = c
    //    >> (flanger(0.6, 0.005, 0.01, |t| lerp11(0.005, 0.01, sin_hz(0.1, t)))
    //        | flanger(0.6, 0.005, 0.01, |t| lerp11(0.005, 0.01, cos_hz(0.1, t))));

    // Add phaser.
    //let c = c
    //    >> (phaser(0.5, |t| sin_hz(0.1, t) * 0.5 + 0.5)
    //        | phaser(0.5, |t| cos_hz(0.1, t) * 0.5 + 0.5));

    let mut c = c
        >> (declick() | declick())
        >> (dcblock() | dcblock())
        //>> (multipass() & 0.2 * reverb_stereo(10.0, 3.0, 1.0))
        >> limiter_stereo(1.0, 5.0);
    //let mut c = c * 0.1;

    c.set_sample_rate(sample_rate);
    c.allocate();

    let (mut slot, mut sb) = Slot::new(Box::new(c));

    let mut next_value = move || assert_no_alloc(|| sb.get_stereo());

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(2000));

    slot.set(Fade::Power, 1.0, Box::new(pink() >> pan(0.0)));

    std::thread::sleep(std::time::Duration::from_millis(2000));

    slot.set(Fade::Power, 1.0, Box::new(zero() >> pan(0.0)));
    std::thread::sleep(std::time::Duration::from_millis(2000));

    slot.set(Fade::Power, 1.0, Box::new(pink() >> pan(0.0)));
    std::thread::sleep(std::time::Duration::from_millis(2000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f32, f32))
where
    T: SizedSample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}
