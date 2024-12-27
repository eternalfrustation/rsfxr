use std::io::{Seek, SeekFrom};

use crate::{Amplitude, Frequency, Sample};

/// Produces a constant frequency, phase increments on every call to `next()`.
#[derive(Clone, Copy)]
pub struct ConstantFrequencyGenerator {
    pub frequency: f64,
    pub phase: u64,
}

impl ConstantFrequencyGenerator {
    pub fn new(frequency: f64) -> Self {
        return Self {
            frequency,
            phase: 0,
        };
    }
}

impl Iterator for ConstantFrequencyGenerator {
    type Item = Sample<Frequency>;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += 1;
        Some(Sample {
            data: Frequency(self.frequency),
            phase: self.phase,
        })
    }
}

impl Seek for ConstantFrequencyGenerator {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        match pos {
            SeekFrom::Start(pos) => {
                self.phase = pos;
            }
            SeekFrom::End(_) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
            SeekFrom::Current(pos) => {
                self.phase = (self.phase as i64 + pos) as u64;
            }
        }
        Ok(self.phase)
    }
}

/// Generates white noise.
#[derive(Clone, Copy)]
pub struct WhiteNoiseGenerator(u64);

impl WhiteNoiseGenerator {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Iterator for WhiteNoiseGenerator {
    type Item = Sample<Amplitude>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(Sample {
            data: Amplitude(rand::random()),
            phase: self.0 - 1,
        })
    }
}

impl Seek for WhiteNoiseGenerator {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        match pos {
            SeekFrom::Start(pos) => {
                self.0 = pos;
            }
            SeekFrom::End(_) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
            SeekFrom::Current(pos) => {
                self.0 = (self.0 as i64 + pos) as u64;
            }
        }
        Ok(self.0)
    }
}
