use std::{sync::mpsc, time::Duration};

use rodio::{Sample, Source};

use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::{collections::VecDeque, time::SystemTime};

use super::wave;

pub struct AudioQueue {
    pub queue: VecDeque<f32>,
    pub shall_clear_old_samples: bool,
}
pub struct AudioDriver {
    channels: u16,
    sample_rate: u32,
    audio_queue: VecDeque<f32>,
    rx: mpsc::Receiver<AudioQueue>,
}

impl AudioDriver {
    const SAMPLE_RATE: u32 = 44100;

    pub fn new(sample_rate: u32, channels: u16, rx: mpsc::Receiver<AudioQueue>) -> Self {
        let mut se = Self {
            channels,
            sample_rate: AudioDriver::SAMPLE_RATE,

            audio_queue: VecDeque::new(),
            rx,
        };

        return se;
    }

    fn get_sample(&mut self) -> f32 {
        let sample: f32;

        sample = match self.audio_queue.pop_front() {
            Some(val) => {
                // println!("{}", val);
                val
            }
            None => 0.0,
        };

        return sample;
    }

    fn queue_samples(&mut self, queue: &mut AudioQueue) {
        // println!("new samples {:?}", queue.queue);
        if queue.shall_clear_old_samples {
            self.audio_queue.clear();
        }
        self.audio_queue.append(&mut queue.queue);
    }
}

impl Source for AudioDriver {
    fn channels(&self) -> u16 {
        return self.channels;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for AudioDriver {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.rx.try_recv() {
            Ok(mut queue) => self.queue_samples(&mut queue),
            _ => (),
        }
        return Some(self.get_sample());
    }
}

#[test]
fn test_audio_driver() {
    let mut test_queue: VecDeque<f32> = VecDeque::new();
    let mut freq = 220.;
    let mut wave_length = (1. / freq * 44100.) as u32;
    let mut curr_frame_sample = 0;
    for i in 0..44100 {
        curr_frame_sample = i % wave_length;
        test_queue.push_back(if curr_frame_sample < wave_length / 2 { 1.0 } else { 0.0 });
        if i % (44100. / 10.) as u32 == 0 {
            freq += 10.;
            wave_length = (1. / freq * 44100.) as u32;
        }
    }

    let (tx, rx) = mpsc::channel();
    let mut oscillator = AudioDriver::new(44100, 1, rx);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(oscillator.amplify(0.1));

    std::thread::sleep(std::time::Duration::from_secs(1));
    tx.send(AudioQueue {
        queue: test_queue,
        shall_clear_old_samples: false,
    })
    .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
}
