use std::{sync::mpsc, time::Duration};

use rodio::Source;

use std::collections::VecDeque;

pub struct AudioQueue {
    pub queue: VecDeque<f32>,
    pub shall_clear_old_samples: bool,
}

/// Simple audio queue interface with mpsc using rodio
/// uses 2 channels
pub struct AudioDriver {
    channels: u16,
    sample_rate: u32,
    audio_queue: VecDeque<f32>,
    rx: mpsc::Receiver<AudioQueue>,
}

impl AudioDriver {
    pub fn new(sample_rate: u32, channels: u16, rx: mpsc::Receiver<AudioQueue>) -> Self {
        let se = Self {
            channels,
            sample_rate,

            audio_queue: VecDeque::with_capacity(2048),
            rx,
        };

        return se;
    }

    fn get_sample(&mut self) -> f32 {
        let sample: f32;

        // if self.audio_queue.len() as f32 > self.sample_rate as f32 * 0.016742 * 2. {
        sample = match self.audio_queue.pop_front() {
            Some(val) => {
                // println!("{}", val);
                val
            }
            None => 0.0,
        };
        // } else {
        //     sample = 1.0;
        // }

        return 0.0;
    }

    fn queue_samples(&mut self, queue: &mut AudioQueue) {
        if queue.shall_clear_old_samples {
            self.audio_queue.clear();
        }
        // if self.audio_queue.len() >= 2 * queue.queue.len() {
        //     self.audio_queue.clear();
        // }
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
    use rodio::{OutputStream, Sink};

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
    let oscillator = AudioDriver::new(44100, 1, rx);

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
