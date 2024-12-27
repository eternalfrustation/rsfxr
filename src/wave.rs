use std::io::{Seek, SeekFrom};

use crate::{Amplitude, Frequency, Sample};

#[derive(Clone, Copy)]
pub struct SquareWaveGenerator<T> {
    pub generator: T,
    pub duty_cycle: f64,
    pub sample_rate: u64,
}

#[derive(Clone, Copy)]
pub struct SineWaveGenerator<T> {
    pub generator: T,
    pub duty_cycle: f64,
    pub sample_rate: u64,
}

#[derive(Clone, Copy)]
pub struct SawtoothWaveGenerator<T> {
    pub generator: T,
    pub duty_cycle: f64,
    pub sample_rate: u64,
}

#[derive(Clone, Copy)]
pub struct WhiteNoiseGenerator(u64);

impl WhiteNoiseGenerator {
    pub fn new() -> Self {
        Self(0)
    }
}

impl<T: Iterator<Item = Sample<Frequency>>> Iterator for SquareWaveGenerator<T> {
    type Item = Sample<Amplitude>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|sample| Sample {
            data: Amplitude(
                (*sample * sample.phase as f64 / self.sample_rate as f64 - self.duty_cycle)
                    .fract()
                    .round(),
            ),
            phase: sample.phase,
        })
    }
}

impl<T: Iterator<Item = Sample<Frequency>>> Iterator for SineWaveGenerator<T> {
    type Item = Sample<Amplitude>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|sample| Sample {
            data: Amplitude(
                if (*sample * sample.phase as f64 / self.sample_rate as f64).fract()
                    < self.duty_cycle
                {
                    (*sample * core::f64::consts::TAU * sample.phase as f64
                        / (self.sample_rate as f64 * self.duty_cycle))
                        .sin()
                } else {
                    0.0
                },
            ),
            phase: sample.phase,
        })
    }
}

impl<T: Iterator<Item = Sample<Frequency>>> Iterator for SawtoothWaveGenerator<T> {
    type Item = Sample<Amplitude>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|sample| Sample {
            data: Amplitude(
                if (*sample * sample.phase as f64 / self.sample_rate as f64).fract()
                    < self.duty_cycle
                {
                    (*sample * sample.phase as f64 / (self.sample_rate as f64 * self.duty_cycle))
                        .fract()
                } else {
                    0.0
                },
            ),
            phase: sample.phase,
        })
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

impl<T: Seek> Seek for SquareWaveGenerator<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}

impl<T: Seek> Seek for SineWaveGenerator<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}

impl<T: Seek> Seek for SawtoothWaveGenerator<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
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
