use std::io::{Seek, SeekFrom};

use crate::{Frequency, Sample};

#[derive(Clone, Copy)]
pub struct MinCutoff<T> {
    pub generator: T,
    pub min_cutoff: f64,
}

impl<T: Seek> Seek for MinCutoff<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}

impl<T: Iterator<Item = Sample<Frequency>>> Iterator for MinCutoff<T> {
    type Item = Sample<Frequency>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.generator.next() {
            if *v < self.min_cutoff {
                None
            } else {
                Some(v)
            }
        } else {
            None
        }
    }
}
