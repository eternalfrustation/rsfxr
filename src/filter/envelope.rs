use std::io::{Seek, SeekFrom};

use crate::{Amplitude, Sample};

#[derive(Clone, Copy)]
pub struct Envelope<T> {
    pub generator: T,
    pub attack_time: f64,
    pub sustain_time: f64,
    pub sustain_punch: f64,
    pub decay_time: f64,
    pub sample_rate: u64,
}

impl<T: Iterator<Item = Sample<Amplitude>>> Iterator for Envelope<T> {
    type Item = Sample<Amplitude>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().and_then(|Sample { phase, data }| {
            if (phase as f64 / self.sample_rate as f64) < self.attack_time {
                Some(*data * phase as f64 / (self.sample_rate as f64 * self.attack_time))
            } else if (phase as f64)
                < (self.attack_time + self.sustain_time) * self.sample_rate as f64
            {
                Some(*data * self.sustain_punch)
            } else if (phase as f64)
                < (self.attack_time + self.sustain_time + self.decay_time) * self.sample_rate as f64
            {
                Some(
                    *data
                        * (1.0
                            - (phase as f64
                                - (self.attack_time + self.sustain_time)
                                    * self.sample_rate as f64)
                                / (self.decay_time * self.sample_rate as f64)),
                )
            } else {
                None
            }
            .map(|v| Sample {
                data: Amplitude(v),
                phase,
            })
        })
    }
}

impl<T: Seek> Seek for Envelope<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        self.generator.seek(pos)
    }
}
