use std::ops::Deref;

pub mod filter;
pub mod generator;
pub mod wave;

/// A packet of frequency in Hz.
#[derive(Clone, Copy, Debug)]
pub struct Frequency(pub f64);

impl Deref for Frequency {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Frequency {
    fn from(value: f64) -> Self {
        Frequency(value)
    }
}

/// A packet of amplitude.
#[derive(Clone, Copy, Debug)]
pub struct Amplitude(pub f64);

impl From<f64> for Amplitude {
    fn from(value: f64) -> Self {
        Amplitude(value)
    }
}

impl Deref for Amplitude {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Sample can contain either frequency or amplitude and the current state of the stream as phase.
#[derive(Clone, Copy, Debug)]
pub struct Sample<T> {
    pub data: T,
    pub phase: u64,
}

impl<T> Deref for Sample<T>
where
    T: Deref<Target = f64>,
{
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &(*self.data)
    }
}
