use std::io::{Seek, SeekFrom};

use crate::{Amplitude, Sample};

#[derive(Clone)]
pub struct Lowpass<T> {
    pub generator: T,
    pub cutoff: f64,
    pub smoothing_factor: f64,
    prev_sample: Sample<Amplitude>,
}

impl<T: Iterator<Item = Sample<Amplitude>>> Iterator for Lowpass<T> {
    type Item = Sample<Amplitude>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|sample| {
            let output = Sample {
                data: Amplitude(
                    *sample * self.smoothing_factor
                        + *self.prev_sample * (1.0 - self.smoothing_factor),
                ),
                ..sample
            };
            self.prev_sample = sample;
            output
        })
    }
}

impl<T: Seek> Seek for Lowpass<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}

impl<T: Iterator<Item = Sample<Amplitude>>> Lowpass<T> {
    pub fn new(generator: T, cutoff: f64, sample_rate: u64) -> Self {
        Lowpass {
            cutoff,
            smoothing_factor: 1.0
                / (1.0 + 2.0 * std::f64::consts::PI * cutoff * sample_rate as f64),
            prev_sample: Sample {
                data: Amplitude(0.0),
                phase: 0,
            },
            generator,
        }
    }
}
