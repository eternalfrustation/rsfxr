use alsa::{
    pcm::{Format, HwParams},
    Direction, ValueOr, PCM,
};
use rsfxr::filter::{AmplitudeDomainFilterable, ConstantFrequencyGenerator, FrequencyDomainFilterable};
use rsfxr::wave::WhiteNoiseGenerator;

fn main() {
    let sample_rate = 44100;
    let wave_builder = ConstantFrequencyGenerator::new(100.0);
    let square_wave = wave_builder.clone().square_wave(sample_rate, 0.5);
    let sine_wave = wave_builder.clone().sine_wave(sample_rate, 1.0);
    let sawtooth_wave = wave_builder.sawtooth_wave(sample_rate, 1.0);
    let white_noise = WhiteNoiseGenerator::new();

    let pcm = PCM::new("default", Direction::Playback, false).unwrap();
    {
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(1).unwrap();
        hwp.set_rate(sample_rate as u32, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::float()).unwrap();
        hwp.set_access(alsa::pcm::Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();
    }

    {
        let io = pcm.io_f32().unwrap();
        println!("Playing square wave");
        io.writei(
            square_wave
                .envelope(0.0, 1.0, 1.0, 1.0, sample_rate)
                .retrigger(2.0, 4, sample_rate)
                .map(|f| *f as f32)
                .collect::<Vec<f32>>()
                .as_slice(),
        )
        .unwrap();
        println!("Playing sawtooth wave");
        io.writei(
            sawtooth_wave
                .envelope(1.0, 1.0, 1.0, 1.0, sample_rate)
                .map(|f| *f as f32)
                .collect::<Vec<f32>>()
                .as_slice(),
        )
        .unwrap();
        println!("Playing sine wave");
        io.writei(
            sine_wave
                .envelope(0.0, 1.0, 1.0, 1.0, sample_rate)
                .retrigger(2.0, 4, sample_rate)
                .map(|f| *f as f32)
                .collect::<Vec<f32>>()
                .as_slice(),
        )
        .unwrap();
        println!("Playing white noise");
        io.writei(
            white_noise
                .envelope(1.0, 1.0, 1.0, 1.0, sample_rate)
                .map(|f| *f as f32)
                .collect::<Vec<f32>>()
                .as_slice(),
        )
        .unwrap();
    }
    pcm.drain().unwrap();
    pcm.drop().unwrap();
}
