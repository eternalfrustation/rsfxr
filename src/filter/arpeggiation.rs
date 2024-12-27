use std::io::{Seek, SeekFrom};

use crate::{Frequency, Sample};

#[derive(Clone, Copy)]
pub struct Arpeggiation<T> {
    pub generator: T,
    pub freq_mult: f64,
    pub delay: f64,
    pub sample_rate: u64,
}

impl<T: Iterator<Item = Sample<Frequency>>> Iterator for Arpeggiation<T> {
    type Item = Sample<Frequency>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|v| {
            if self.sample_rate as f64 * self.delay > v.phase as f64 {
                v
            } else {
                Sample {
                    data: Frequency(*v * self.freq_mult),
                    ..v
                }
            }
        })
    }
}

impl<T: Seek> Seek for Arpeggiation<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}
