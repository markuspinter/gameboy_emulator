use std::{sync::mpsc, time::Duration};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};

use crate::{
    bit,
    gameboy::{memory, MemoryInterface},
};

use super::utils::{speed, CustomSource};

#[derive(Clone, Debug, FromPrimitive)]
enum WaveOutputLevel {
    Mute = 0b00,
    P100 = 0b01,
    P50 = 0b10,
    P25 = 0b11,
}

pub struct Wave {
    dac_enable: bool,

    wave_length: u16,
    length_timer: u8,
    output_level: WaveOutputLevel,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR21 expires)
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    noise_mpsc: mpsc::Sender<WaveParameters>,

    wave_pattern_ram: [u8; memory::apu::WAVE_PATTERN_RAM.size],
    wave_pattern_vec: Vec<f32>,
}

impl MemoryInterface for Wave {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::apu::NR30 {
            return Some(self.get_nr30());
        } else if addr == memory::apu::NR31 {
            return Some(self.get_nr31());
        } else if addr == memory::apu::NR32 {
            return Some(self.get_nr32());
        } else if addr == memory::apu::NR33 {
            return Some(self.get_nr33());
        } else if addr == memory::apu::NR34 {
            return Some(self.get_nr34());
        } else if addr >= memory::apu::WAVE_PATTERN_RAM.begin && addr <= memory::apu::WAVE_PATTERN_RAM.end {
            return Some(self.wave_pattern_ram[addr as usize]);
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::apu::NR30 {
            self.set_nr30(value);
        } else if addr == memory::apu::NR31 {
            self.set_nr31(value);
        } else if addr == memory::apu::NR32 {
            self.set_nr32(value);
        } else if addr == memory::apu::NR33 {
            self.set_nr33(value);
        } else if addr == memory::apu::NR34 {
            self.set_nr34(value);
        } else if addr >= memory::apu::WAVE_PATTERN_RAM.begin && addr <= memory::apu::WAVE_PATTERN_RAM.end {
            self.set_wave_pattern(addr, value);
        } else {
            return None;
        }
        return Some(());
    }
}

impl Wave {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            dac_enable: false,

            wave_length: 0,

            length_timer: 0,
            output_level: WaveOutputLevel::Mute,

            shall_trigger: false,
            sound_length_enable: false,

            sink: Sink::try_new(&stream_handle).unwrap(),
            stream: _stream,
            stream_handle: stream_handle,
            noise_mpsc: mpsc::channel().0,

            wave_pattern_ram: [0; memory::apu::WAVE_PATTERN_RAM.size],
            wave_pattern_vec: vec![0.0; memory::apu::WAVE_PATTERN_RAM.size * 2],
        }
    }

    fn get_nr30(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.dac_enable as u8) << 7;
        byte
    }

    fn get_nr31(&self) -> u8 {
        self.length_timer
    }
    fn get_nr32(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= ((self.output_level as u8) & 0b11) << 5;
        byte
    }
    fn get_nr33(&self) -> u8 {
        (self.wave_length & 0xFF) as u8
    }
    fn get_nr34(&self) -> u8 {
        let mut byte: u8 = 0;
        //bit 7 trigger is read only returning 1s for this section
        byte |= 0xFF_u8 << 7;
        byte |= (self.sound_length_enable as u8) << 6;
        byte |= ((self.wave_length >> 8) & 0b111) as u8;
        byte
    }

    fn set_nr30(&mut self, value: u8) {
        self.dac_enable = bit!(value, 7) != 0;
        println!("nr30 {:#010b}, {}", value, self.dac_enable);
        if !self.sink.empty() {
            if !self.dac_enable {
                self.sink.stop();
            }
        }
    }

    fn set_nr31(&mut self, value: u8) {
        self.length_timer = value;
    }
    fn set_nr32(&mut self, value: u8) {
        self.output_level = FromPrimitive::from_u8((value >> 5) & 0b11).expect("couldn't convert wave output level");
        for index in 0..memory::apu::WAVE_PATTERN_RAM.size {
            self.wave_pattern_vec[index * 2] = ((self.wave_pattern_ram[index] >> 4)
                >> match self.output_level {
                    WaveOutputLevel::Mute => 8, //mute
                    WaveOutputLevel::P100 => 0,
                    WaveOutputLevel::P50 => 1,
                    WaveOutputLevel::P25 => 2,
                }) as f32
                / 16.0;
            self.wave_pattern_vec[index * 2 + 1] = ((self.wave_pattern_ram[index] & 0x0F)
                >> match self.output_level {
                    WaveOutputLevel::Mute => 8, //mute
                    WaveOutputLevel::P100 => 0,
                    WaveOutputLevel::P50 => 1,
                    WaveOutputLevel::P25 => 2,
                }) as f32
                / 16.0;
        }

        // update wave pattern
        if !self.sink.empty() {
            let freq = 65536.0 / (2048 - self.wave_length as u32) as f32;
            self.noise_mpsc
                .send(WaveParameters {
                    wave_table: self.wave_pattern_vec.clone(),
                    frequency: freq as f32,
                })
                .unwrap();
        }
    }
    fn set_nr33(&mut self, value: u8) {
        self.wave_length &= 0x0700;
        self.wave_length |= value as u16;

        if !self.sink.empty() {
            let freq = 65536.0 / (2048 - self.wave_length as u32) as f32;
            self.noise_mpsc
                .send(WaveParameters {
                    wave_table: self.wave_pattern_vec.clone(),
                    frequency: freq as f32,
                })
                .unwrap();
        }
    }
    fn set_nr34(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;
        self.wave_length &= 0x00FF;
        self.wave_length |= ((value & 0b111) as u16) << 8;

        if self.sink.empty() {
            if self.shall_trigger {
                let duration =
                    Duration::from_micros((((1.0 / 256.0) * (64.0 - self.length_timer as f32)) * 1e6) as u64);
                let freq = 65536.0 / (2048 - self.wave_length as u32) as f32;
                // log::debug!("start sound for: {:?}", duration);
                let oscillator = WaveOscillator::new(44100, self.wave_pattern_vec.clone());
                let res = speed::<WaveOscillator, WaveParameters>(oscillator);
                self.noise_mpsc = res.1;

                if self.sound_length_enable {
                    self.sink.append(res.0.take_duration(duration).amplify(0.05));
                } else {
                    self.sink.append(res.0.amplify(0.05));
                }
                // self.sink.append(res.0.take_duration(duration).amplify(0.05));
                self.sink.play();

                self.noise_mpsc
                    .send(WaveParameters {
                        wave_table: self.wave_pattern_vec.clone(),
                        frequency: freq as f32,
                    })
                    .unwrap();
            }
        } else {
            let freq = 65536.0 / (2048 - self.wave_length as u32) as f32;
            self.noise_mpsc
                .send(WaveParameters {
                    wave_table: self.wave_pattern_vec.clone(),
                    frequency: freq as f32,
                })
                .unwrap();
        }
    }
    fn set_wave_pattern(&mut self, addr: u16, value: u8) {
        let index = (addr - memory::apu::WAVE_PATTERN_RAM.begin) as usize;
        self.wave_pattern_ram[index] = value;
        self.wave_pattern_vec[index * 2] = ((value >> 4)
            >> match self.output_level {
                WaveOutputLevel::Mute => 8, //mute
                WaveOutputLevel::P100 => 0,
                WaveOutputLevel::P50 => 1,
                WaveOutputLevel::P25 => 2,
            }) as f32
            / 16.0;
        self.wave_pattern_vec[index * 2 + 1] = ((value & 0x0F)
            >> match self.output_level {
                WaveOutputLevel::Mute => 8, //mute
                WaveOutputLevel::P100 => 0,
                WaveOutputLevel::P50 => 1,
                WaveOutputLevel::P25 => 2,
            }) as f32
            / 16.0;
        //update wave pattern
        if !self.sink.empty() {
            let freq = 65536.0 / (2048 - self.wave_length as u32) as f32;
            self.noise_mpsc
                .send(WaveParameters {
                    wave_table: self.wave_pattern_vec.clone(),
                    frequency: freq as f32,
                })
                .unwrap();
        }
    }
}

struct WaveParameters {
    wave_table: Vec<f32>,
    frequency: f32,
}

struct WaveOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveOscillator {
        return WaveOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn get_sample(&mut self) -> f32 {
        let sample: f32;

        sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;

        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl CustomSource<WaveParameters> for WaveOscillator {
    fn set_parameters(&mut self, parameters: WaveParameters) {
        self.wave_table = parameters.wave_table;
        self.index_increment = parameters.frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }
}

impl Source for WaveOscillator {
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

impl Iterator for WaveOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}
