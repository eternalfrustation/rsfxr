use std::io::{Seek, SeekFrom};

use crate::{Frequency, Sample};

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
