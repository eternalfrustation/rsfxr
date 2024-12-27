use std::{io::{Seek, SeekFrom}, ops::Deref};

use crate::Sample;

#[derive(Clone, Copy)]
pub struct Flanger<T> {
    pub generator: T,
    pub offset: f64,
    pub sweep: f64,
    pub sample_rate: u64,
}

// I... don't know if this is correct, hell, i don't know if any of this is correct, but it sounds
// right. Someone with more experience with audio processing check this
impl<G, T> Iterator for Flanger<G>
where
    G: Seek + Iterator<Item = Sample<T>> + Clone,
    T: Deref<Target = f64> + From<f64>,
{
    type Item = Sample<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator
            .next()
            .map(|sample| {
                if sample.phase as f64 / self.sample_rate as f64 > self.offset {
                    let mut temp = self.generator.clone();
                    temp.seek(SeekFrom::Current(
                        (self.sample_rate as f64
                            * self.sweep
                            * (sample.phase as f64 / self.sample_rate as f64 - self.offset).sin())
                        .round() as i64,
                    ))
                    .unwrap();
                    temp.next().map(|sample1| Sample {
                        data: T::from(*sample + *sample1),
                        ..sample
                    })
                } else {
                    Some(sample)
                }
            })
            .flatten()
    }
}

impl<T: Seek> Seek for Flanger<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}
