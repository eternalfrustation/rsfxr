use std::io::{Seek, SeekFrom};

use crate::{Amplitude, Sample};

#[derive(Clone)]
pub struct Highpass<T> {
    pub generator: T,
    pub cutoff: f64,
    pub smoothing_factor: f64,
    prev_sample: Sample<Amplitude>,
    prev_output_sample: Sample<Amplitude>,
}

impl<T: Iterator<Item = Sample<Amplitude>>> Iterator for Highpass<T> {
    type Item = Sample<Amplitude>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|sample| {
            self.prev_output_sample = Sample {
                data: Amplitude(
                    *self.prev_output_sample * self.smoothing_factor
                        + self.smoothing_factor * (*sample - *self.prev_sample),
                ),
                ..sample
            };
            self.prev_output_sample
        })
    }
}

impl<T: Seek> Seek for Highpass<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}

impl<T: Iterator<Item = Sample<Amplitude>>> Highpass<T> {
    pub fn new(generator: T, cutoff: f64, sample_rate: u64) -> Self {
        Highpass {
            cutoff,
            smoothing_factor: sample_rate as f64
                / (sample_rate as f64 + 2.0 * std::f64::consts::PI * cutoff),
            prev_sample: Sample {
                data: Amplitude(0.0),
                phase: 0,
            },
            prev_output_sample: Sample {
                data: Amplitude(0.0),
                phase: 0,
            },
            generator,
        }
    }
}
