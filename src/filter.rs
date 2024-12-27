use std::{
    io::{Seek, SeekFrom},
    ops::Deref,
};

use crate::wave::{SawtoothWaveGenerator, SineWaveGenerator, SquareWaveGenerator};

#[derive(Clone, Copy, Debug)]
pub struct Frequency(pub f64);

impl Deref for Frequency {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Frequency {
    fn from(value: f64) -> Self {
        Frequency(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Amplitude(pub f64);

impl From<f64> for Amplitude {
    fn from(value: f64) -> Self {
        Amplitude(value)
    }
}

impl Deref for Amplitude {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sample<T> {
    pub data: T,
    pub phase: u64,
}

impl<T> Deref for Sample<T>
where
    T: Deref<Target = f64>,
{
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &(*self.data)
    }
}

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

pub trait FrequencyDomainFilterable {
    fn min_cutoff(self, min_cutoff: f64) -> MinCutoff<Self>
    where
        Self: Sized,
    {
        MinCutoff {
            generator: self,
            min_cutoff,
        }
    }

    fn slide(
        self,
        frequency_slide: f64,
        frequency_slide_delta: f64,
        sample_rate: u64,
    ) -> FrequencySlide<Self>
    where
        Self: Sized,
    {
        FrequencySlide {
            generator: self,
            frequency_slide: frequency_slide / sample_rate as f64,
            frequency_slide_delta: frequency_slide_delta / sample_rate as f64,
        }
    }

    fn vibrato(self, vibrato_speed: f64, vibrato_depth: f64, sample_rate: u64) -> Vibrato<Self>
    where
        Self: Sized,
    {
        Vibrato {
            generator: self,
            vibrato_speed,
            vibrato_depth,
            sample_rate,
        }
    }

    fn arpeggiation(self, freq_mult: f64, delay: f64, sample_rate: u64) -> Arpeggiation<Self>
    where
        Self: Sized,
    {
        Arpeggiation {
            generator: self,
            delay,
            freq_mult,
            sample_rate,
        }
    }

    fn retrigger(self, rate: f64, num_retriggers: u64, sample_rate: u64) -> Retrigger<Self>
    where
        Self: Sized + Seek,
    {
        Retrigger {
            generator: self,
            rate,
            sample_rate,
            num_retriggers,
            phase: 0,
            retriggers: Vec::new(),
        }
    }

    fn square_wave(self, sample_rate: u64, duty_cycle: f64) -> SquareWaveGenerator<Self>
    where
        Self: Sized,
    {
        SquareWaveGenerator {
            generator: self,
            sample_rate,
            duty_cycle,
        }
    }

    fn sine_wave(self, sample_rate: u64, duty_cycle: f64) -> SineWaveGenerator<Self>
    where
        Self: Sized,
    {
        SineWaveGenerator {
            generator: self,
            sample_rate,
            duty_cycle,
        }
    }

    fn sawtooth_wave(self, sample_rate: u64, duty_cycle: f64) -> SawtoothWaveGenerator<Self>
    where
        Self: Sized,
    {
        SawtoothWaveGenerator {
            generator: self,
            sample_rate,
            duty_cycle,
        }
    }
}

impl<T: Iterator<Item = Sample<Frequency>>> FrequencyDomainFilterable for T {}

#[derive(Clone, Copy)]
pub struct Arpeggiation<T> {
    pub generator: T,
    pub freq_mult: f64,
    pub delay: f64,
    pub sample_rate: u64,
}

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

pub trait AmplitudeDomainFilterable {
    fn retrigger(self, rate: f64, num_retriggers: u64, sample_rate: u64) -> Retrigger<Self>
    where
        Self: Sized + Seek,
    {
        Retrigger {
            generator: self,
            rate,
            num_retriggers,
            sample_rate,
            phase: 0,
            retriggers: Vec::new(),
        }
    }

    fn flanger(self, offset: f64, sweep: f64, sample_rate: u64) -> Flanger<Self>
    where
        Self: Sized + Seek,
    {
        Flanger {
            generator: self,
            sample_rate,
            offset,
            sweep,
        }
    }

    fn envelope(
        self,
        attack_time: f64,
        sustain_time: f64,
        sustain_punch: f64,
        decay_time: f64,
        sample_rate: u64,
    ) -> Envelope<Self>
    where
        Self: Sized,
    {
        Envelope {
            generator: self,
            attack_time,
            sustain_time,
            sustain_punch,
            decay_time,
            sample_rate,
        }
    }
}

impl<T: Iterator<Item = Sample<Amplitude>>> AmplitudeDomainFilterable for T {}

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
