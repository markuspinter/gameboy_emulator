pub enum LFSRWidth {
    LFSR15Bits,
    LFSR7Bits,
}

struct NoiseParameters {
    clock_shift: u8,
    clock_divider: u8,
}

pub struct Noise {
    length_timer: u8,
    inital_envelope_volume: u8,
    envelope_increase: bool,
    sweep_pace: u8,
    clock_shift: u8,
    lfsr_width: LFSRWidth,
    clock_divider: u8,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR41 expires)
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    noise_mpsc: mpsc::Sender<NoiseParameters>,
}

impl MemoryInterface for Noise {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::apu::NR41 {
            return Some(self.get_nr41());
        } else if addr == memory::apu::NR42 {
            return Some(self.get_nr42());
        } else if addr == memory::apu::NR43 {
            return Some(self.get_nr43());
        } else if addr == memory::apu::NR44 {
            return Some(self.get_nr44());
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::apu::NR41 {
            self.set_nr41(value);
        } else if addr == memory::apu::NR42 {
            self.set_nr42(value);
        } else if addr == memory::apu::NR43 {
            self.set_nr43(value);
        } else if addr == memory::apu::NR44 {
            self.set_nr44(value);
        } else {
            return None;
        }
        return Some(());
    }
}

impl Noise {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            length_timer: 0,
            inital_envelope_volume: 0,
            envelope_increase: false,
            sweep_pace: 0,
            clock_shift: 0,
            lfsr_width: LFSRWidth::LFSR15Bits,
            clock_divider: 0,
            shall_trigger: false,
            sound_length_enable: false,

            sink: Sink::try_new(&stream_handle).unwrap(),
            stream: _stream,
            stream_handle: stream_handle,
            noise_mpsc: mpsc::channel().0,
        }
    }

    fn get_nr41(&self) -> u8 {
        self.length_timer & 0b11111
    }
    fn get_nr42(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= self.inital_envelope_volume << 4;
        byte |= (self.envelope_increase as u8) << 3;
        byte |= self.sweep_pace & 0b111;
        byte
    }
    fn get_nr43(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= self.clock_shift << 4;
        byte |= (self.lfsr_width as u8) << 3;
        byte |= self.clock_divider & 0b111;
        byte
    }
    fn get_nr44(&self) -> u8 {
        let mut byte: u8 = 0;
        //bit 7 trigger is read only returning 1s for this section
        byte |= 0xFF_u8 << 7;
        byte |= (self.sound_length_enable as u8) << 6;
        byte
    }

    fn set_nr41(&mut self, value: u8) {
        self.length_timer = value & 0b11111;
    }
    fn set_nr42(&mut self, value: u8) {
        self.inital_envelope_volume = value >> 4;
        self.envelope_increase = bit!(value, 4) != 0;
        self.sweep_pace = value & 0b111;
        if !self.sink.empty() {
            if value & 0xF8 == 0 {
                // self.sink.stop();
            }
        }
    }
    fn set_nr43(&mut self, value: u8) {
        self.clock_shift = value >> 4;
        self.lfsr_width = if bit!(value, 3) == 0 {
            LFSRWidth::LFSR15Bits
        } else {
            LFSRWidth::LFSR7Bits
        };
        self.clock_divider = value & 0b111;
    }
    fn set_nr44(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;

        if self.sink.empty() && self.shall_trigger {
            let duration = Duration::from_micros((((1.0 / 256.0) * (64.0 - self.length_timer as f32)) * 1e6) as u64);
            // println!("start sound for: {:?}", duration);
            let oscillator = NoiseOscillator::new(44100);
            let res = speed::<NoiseOscillator, NoiseParameters>(oscillator);
            self.noise_mpsc = res.1;

            // if self.sound_length_enable {
            //     self.sink.append(res.0.take_duration(duration).amplify(0.05));
            // } else {
            //     self.sink.append(res.0.amplify(0.05));
            // }
            // self.sink.append(res.0.take_duration(duration).amplify(0.05));

            // self.noise_mpsc
            //     .send(NoiseParameters {
            //         clock_shift: 0,
            //         clock_divider: 0,
            //     })
            //     .unwrap();
        }
    }
}

use core::time::Duration;
use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};
use std::{collections::VecDeque, sync::mpsc, time::SystemTime};

use crate::{
    bit,
    gameboy::{memory, MemoryInterface},
};

use super::utils::{speed, CustomSource};

pub struct NoiseOscillator {
    sample_rate: u32,
    index: f32,
    index_increment: f32,

    clock_shift: u8,
    lfsr_width: u8,
    clock_divider: u8,

    lfsr_queue: VecDeque<u16>,
    lfsr: u16,
}

impl NoiseOscillator {
    pub fn new(sample_rate: u32) -> Self {
        let mut se = Self {
            sample_rate: sample_rate,

            index_increment: 0.0,
            index: 0.0,

            clock_shift: 0,
            lfsr_width: 15,
            clock_divider: 0,

            lfsr: 0,
            lfsr_queue: VecDeque::new(),
        };

        se.tick();
        se.set_parameters(NoiseParameters {
            clock_shift: 0,
            clock_divider: 0,
        });
        return se;
    }

    fn tick(&mut self) {
        if self.lfsr_queue.len() > 0 {
            self.lfsr_queue.pop_front();
        } else {
            self.lfsr_queue.push_back(self.lfsr);
        }

        let new_bit = !(bit!(self.lfsr, 0) ^ bit!(self.lfsr, 1)); //xnor operation
        self.lfsr = (self.lfsr & !(1 << 15)) | (new_bit << 15);
        self.lfsr = self.lfsr >> 1;
        self.lfsr_queue.push_back(self.lfsr);
    }

    fn get_sample(&mut self) -> f32 {
        let sample: f32;

        sample = self.lerp();
        self.index += self.index_increment;
        self.index %= 2.0 as f32;

        return sample;
    }

    fn lerp(&mut self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % 2;

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        if (truncated_index) == 1 {
            self.tick();
        }
        let val = if (self.lfsr_queue[0] & 0b1) == 0 { -1.0 } else { 1.0 };
        let next_val = if (self.lfsr_queue[1] & 0b1) == 0 { -1.0 } else { 1.0 };

        return truncated_index_weight * val + next_index_weight * next_val;
    }
}

impl CustomSource<NoiseParameters> for NoiseOscillator {
    fn set_parameters(&mut self, parameters: NoiseParameters) {
        // self.index_increment = frequency * 64 as f32
        //                        / self.sample_rate as f32;
        self.clock_shift = parameters.clock_shift;
        self.clock_divider = (parameters.clock_divider * 2);
        self.lfsr = 0;
        self.lfsr_queue.clear();
        self.tick();
        if parameters.clock_divider == 0 {
            self.clock_divider = 1;
        }
        let frequency: u32 = (2_u32 * 262144) / (self.clock_divider as u32 * (1 << self.clock_shift as u32));
        self.index_increment = (frequency as f32 * 2.0) / self.sample_rate as f32;
    }
}

impl Source for NoiseOscillator {
    fn channels(&self) -> u16 {
        return 1;
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

impl Iterator for NoiseOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

#[test]
fn test_noise() {
    let mut oscillator = NoiseOscillator::new(44100);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    // receive pitch mpsc channel too
    let (source, pitch) = speed(oscillator);
    let real_source = source.take_duration(Duration::from_millis(1000));
    sink.append(real_source);

    // std::thread::sleep(std::time::Duration::from_secs(5));
    sink.sleep_until_end();
}
