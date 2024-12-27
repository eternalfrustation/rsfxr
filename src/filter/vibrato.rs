use std::io::{Seek, SeekFrom};

use crate::{Frequency, Sample};

#[derive(Clone, Copy)]
pub struct Vibrato<T> {
    pub generator: T,
    pub vibrato_speed: f64,
    pub vibrato_depth: f64,
    pub sample_rate: u64,
}

impl<T: Iterator<Item = Sample<Frequency>>> Iterator for Vibrato<T> {
    type Item = Sample<Frequency>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|sample| {
            let vibrato = (self.vibrato_depth
                * (self.vibrato_speed * sample.phase as f64 / self.sample_rate as f64
                    * std::f64::consts::TAU)
                    .sin()) as f64;
            Sample {
                data: Frequency(*sample + vibrato),
                ..sample
            }
        })
    }
}

impl<T: Seek> Seek for Vibrato<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}
