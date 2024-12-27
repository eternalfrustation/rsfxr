use std::{io::{Seek, SeekFrom}, ops::Deref};

use crate::Sample;

#[derive(Clone)]
pub struct Retrigger<T> {
    pub generator: T,
    pub rate: f64,
    pub num_retriggers: u64,
    pub retriggers: Vec<T>,
    pub sample_rate: u64,
    pub phase: u64,
}

impl<G, T> Iterator for Retrigger<G>
where
    G: Seek + Iterator<Item = Sample<T>> + Clone,
    T: Deref<Target = f64> + From<f64>,
{
    type Item = Sample<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.generator.next();
        let mut samples = self
            .retriggers
            .iter_mut()
            .flat_map(|r| r.next())
            .collect::<Vec<Self::Item>>();
        if let Some(sample) = sample {
            self.phase = sample.phase;
            samples.push(sample);
        } else {
            self.phase += 1;
        }

        if self.num_retriggers as usize >= self.retriggers.len()
            && ((self.phase as f64 * self.rate) as u64 > self.sample_rate * (self.retriggers.len() as u64 + 1))
        {
            let mut temp = self.generator.clone();
            println!("Phase: {}", self.phase);
            temp.rewind().unwrap();
            self.retriggers.push(temp);
        }
        if samples.len() > 0 {
            Some(Sample {
                data: T::from(samples.into_iter().map(|v| *v).sum::<f64>()),
                phase: self.phase,
            })
        } else {
            None
        }
    }
}

impl<T: Seek> Seek for Retrigger<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}
