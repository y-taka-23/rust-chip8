use cpal::platform::Stream;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{default_host, Device, OutputCallbackInfo, Sample, SampleFormat, StreamConfig};
use std::f32::consts::PI;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Buzzer {
    _stream: Stream,
    volume: Sender<f32>,
}

impl Buzzer {
    pub fn new() -> Self {
        let host = default_host();
        let device = host.default_output_device().unwrap();

        let mut supported_configs_range = device.supported_output_configs().unwrap();
        let supported_config = supported_configs_range
            .next()
            .unwrap()
            .with_max_sample_rate();

        let sample_format = supported_config.sample_format();
        let config: StreamConfig = supported_config.into();

        let (stream, send_volume) = match sample_format {
            SampleFormat::F32 => run_stream::<f32>(device, config),
            SampleFormat::I16 => run_stream::<i16>(device, config),
            SampleFormat::U16 => run_stream::<u16>(device, config),
        };

        Buzzer {
            _stream: stream,
            volume: send_volume,
        }
    }

    pub fn on(&self) {
        self.volume.send(0.2).unwrap();
    }

    pub fn off(&self) {
        self.volume.send(0.0).unwrap();
    }
}

fn run_stream<T: Sample>(device: Device, config: StreamConfig) -> (Stream, Sender<f32>) {
    let sample_rate = config.sample_rate.0 as f32;
    let mut sample_clock = 0.0;
    let mut volume = 0.0;

    let (send_volume, recv_volume): (Sender<f32>, Receiver<f32>) = channel();

    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        if let Ok(vol) = recv_volume.try_recv() {
            volume = vol;
        }
        (sample_clock * 440.0 * 2.0 * PI / sample_rate).sin() * volume
    };

    let data_callback = move |output: &mut [T], _: &OutputCallbackInfo| {
        for frame in output.chunks_mut(config.channels as usize) {
            let point: T = Sample::from::<f32>(&next_value());
            for sample in frame.iter_mut() {
                *sample = point;
            }
        }
    };
    let err_callback = |error| panic!("{:?}", error);

    let stream = device
        .build_output_stream(&config, data_callback, err_callback)
        .unwrap();

    stream.play().unwrap();
    (stream, send_volume)
}
