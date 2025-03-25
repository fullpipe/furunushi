use fundsp::hacker::*;

#[inline]
pub fn midi_hz_tuning<T: Real>(x: T, tuning: i64) -> T {
    T::new(tuning) * exp2((x - T::new(69)) / T::new(12))
}

pub fn sine_minor_chorus(
    midi: u32,
    tuning: f32,
    volume: &Shared,
    sample_rate: f64,
) -> Box<dyn AudioUnit> {
    let c = sine_hz(midi_hz_tuning(midi as f32, tuning as i64))
        & 0.5 * sine_hz(midi_hz_tuning((midi + 3) as f32, tuning as i64))
        & 0.25 * sine_hz(midi_hz_tuning((midi + 7) as f32, tuning as i64));

    let c = c * var(&volume);
    let c = c >> pan(0.0);
    let mut c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    c.set_sample_rate(sample_rate);
    c.allocate();

    Box::new(c)
}

pub fn sine_minor(midi: u32, tuning: f32, volume: &Shared, sample_rate: f64) -> Box<dyn AudioUnit> {
    let c = sine_hz(midi_hz_tuning(midi as f32, tuning as i64))
        & 0.5 * sine_hz(midi_hz_tuning((midi + 3) as f32, tuning as i64))
        & 0.25 * sine_hz(midi_hz_tuning((midi + 7) as f32, tuning as i64));

    let c = c * var(&volume);
    let mut c = c >> pan(0.0);

    c.set_sample_rate(sample_rate);
    c.allocate();

    Box::new(c)
}

pub fn sine_major_chorus(
    midi: u32,
    tuning: f32,
    volume: &Shared,
    sample_rate: f64,
) -> Box<dyn AudioUnit> {
    let c = sine_hz(midi_hz_tuning(midi as f32, tuning as i64))
        + 0.5 * sine_hz(midi_hz_tuning((midi + 4) as f32, tuning as i64))
        + 0.25 * sine_hz(midi_hz_tuning((midi + 7) as f32, tuning as i64));

    let c = c * var(&volume);
    let c = c >> pan(0.0);
    let mut c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    c.set_sample_rate(sample_rate);
    c.allocate();

    Box::new(c)
}

pub fn sine_major(midi: u32, tuning: f32, volume: &Shared, sample_rate: f64) -> Box<dyn AudioUnit> {
    let c = sine_hz(midi_hz_tuning(midi as f32, tuning as i64))
        + 0.5 * sine_hz(midi_hz_tuning((midi + 4) as f32, tuning as i64))
        + 0.25 * sine_hz(midi_hz_tuning((midi + 7) as f32, tuning as i64));

    let c = c * var(&volume);
    let mut c = c >> pan(0.0);

    c.set_sample_rate(sample_rate);
    c.allocate();

    Box::new(c)
}

pub fn sine_pure_chorus(
    midi: u32,
    tuning: f32,
    volume: &Shared,
    sample_rate: f64,
) -> Box<dyn AudioUnit> {
    let c = sine_hz(midi_hz_tuning(midi as f32, tuning as i64));

    let c = c * var(&volume);
    let c = c >> pan(0.0);
    let mut c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    c.set_sample_rate(sample_rate);
    c.allocate();

    Box::new(c)
}

pub fn sine_pure(midi: u32, tuning: f32, volume: &Shared, sample_rate: f64) -> Box<dyn AudioUnit> {
    let c = sine_hz(midi_hz_tuning(midi as f32, tuning as i64));

    let c = c * var(&volume);
    let mut c = c >> pan(0.0);

    c.set_sample_rate(sample_rate);
    c.allocate();

    Box::new(c)
}
