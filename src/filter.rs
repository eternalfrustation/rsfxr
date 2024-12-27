use std::io::Seek;

use arpeggiation::Arpeggiation;
use envelope::Envelope;
use flanger::Flanger;
use frequency_slide::FrequencySlide;
use min_cutoff::MinCutoff;
use retrigger::Retrigger;
use vibrato::Vibrato;

use crate::{
    wave::{SawtoothWaveGenerator, SineWaveGenerator, SquareWaveGenerator},
    Amplitude, Frequency, Sample,
};

pub mod arpeggiation;
pub mod envelope;
pub mod flanger;
pub mod frequency_slide;
pub mod min_cutoff;
pub mod retrigger;
pub mod vibrato;

/// Contains filters which can be applied to iterators of frequency samples, i.e., to
/// `Iterator<Item = Sample<Frequency>>`
pub trait FrequencyDomainFilterable {
    /// Stops the stream when frequency drops below a threshold.
    fn min_cutoff(self, min_cutoff: f64) -> MinCutoff<Self>
    where
        Self: Sized,
    {
        MinCutoff {
            generator: self,
            min_cutoff,
        }
    }

    /// Adds `frequency_slide` to the frequency on every call to `next()`.
    /// Adds `frequency_slide_delta` to the frequency_slide on every call to `next()`.
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

    /// Does a sin wave on the frequency.
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

    /// Multiplies the frequency after a certain point
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

    /// Generates `num_triggers` new streams identical to current one, but from the beginning, and
    /// adds them to the current one
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

    /// Converts a stream of frequency samples to amplitude samples, using a square wave.
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

    /// Converts a stream of frequency samples to amplitude samples, using a sine wave.
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

    /// Converts a stream of frequency samples to amplitude samples, using a sawtooth wave.
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

pub trait AmplitudeDomainFilterable {
    /// Generates `num_triggers` new streams identical to current one, but from the beginning, and
    /// adds them to the current one
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

    /// Creates a new stream, delays it, then varies the delay based on a sine wave, then adds the
    /// streams together
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

    /// Gradual increase and decrease to the amplitude
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
