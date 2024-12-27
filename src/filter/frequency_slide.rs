use std::io::{Seek, SeekFrom};

use crate::{Frequency, Sample};

#[derive(Clone, Copy)]
pub struct FrequencySlide<T> {
    pub generator: T,
    pub frequency_slide: f64,
    pub frequency_slide_delta: f64,
}

impl<T: Seek> Seek for FrequencySlide<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}

impl<T: Iterator<Item = Sample<Frequency>>> Iterator for FrequencySlide<T> {
    type Item = Sample<Frequency>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|sample| {
            let frequency_shift =
                self.frequency_slide + self.frequency_slide_delta * sample.phase as f64;
            Sample {
                data: Frequency(*sample.data + frequency_shift * sample.phase as f64),
                ..sample
            }
        })
    }
}
