use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rodio::OutputStream;

use crate::{
    bit,
    gameboy::{memory, GameboyModule, MemoryInterface},
};

use super::{APUChannel, APU};

#[derive(Clone, Debug, FromPrimitive)]
enum WaveOutputLevel {
    Mute = 0b00,
    P100 = 0b01,
    P50 = 0b10,
    P25 = 0b11,
}

pub struct Wave {
    dac_enabled: bool,

    wave_length: u16,
    length_timer: u8,
    output_level: WaveOutputLevel,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR21 expires)

    wave_pattern_ram: [u8; memory::apu::WAVE_PATTERN_RAM.size],
    wave_pattern_vec: Vec<u8>,

    t_cycles: u32,
    timer: u8,
    active: bool,
    frame_index: usize,
    samples: Vec<f32>,

    wave_length_cycles: f32,
    frame_index_fraction_increment: f32,
    sample_rate: u32,
}

impl GameboyModule for Wave {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        let apu = &gb.apu;
        if self.t_cycles == 0 {
            self.tick_sampler();

            self.t_cycles = self.wave_length_cycles as u32 * 2 + 1;
        }
        self.sample(&apu);

        self.t_cycles -= 1;
        Ok(self.t_cycles as u32)
    }
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
    const WAVE_PATTERN_FRAME_SIZE: usize = 32;
    pub fn new(sample_rate: u32) -> Self {
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        Self {
            dac_enabled: false,

            wave_length: 0,

            length_timer: 0,
            output_level: WaveOutputLevel::Mute,

            shall_trigger: false,
            sound_length_enable: false,

            wave_pattern_ram: [0; memory::apu::WAVE_PATTERN_RAM.size],
            wave_pattern_vec: vec![0; memory::apu::WAVE_PATTERN_RAM.size * 2],
            t_cycles: 0,
            timer: 0,
            active: false,
            frame_index: 0,
            samples: Vec::with_capacity(2048),

            sample_rate,
            wave_length_cycles: 0.,
            frame_index_fraction_increment: 0.,
        }
    }

    fn get_nr30(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.dac_enabled as u8) << 7;
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
        self.dac_enabled = bit!(value, 7) != 0;
        if !self.dac_enabled {
            self.active = false;
            log::debug!("dac disabled");
        }
    }

    fn set_nr31(&mut self, value: u8) {
        self.length_timer = value;
    }

    fn set_nr32(&mut self, value: u8) {
        self.output_level = FromPrimitive::from_u8((value >> 5) & 0b11).expect("couldn't convert wave output level");
    }

    fn set_nr33(&mut self, value: u8) {
        self.wave_length &= 0x0700;
        self.wave_length |= value as u16;

        self.wave_length_cycles = (2048 - self.wave_length) as f32;
        log::debug!(
            "new period {}, freq {}",
            self.wave_length_cycles,
            (65536. / (2048 - self.wave_length) as f32)
        );
    }

    fn set_nr34(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;
        self.wave_length &= 0x00FF;
        self.wave_length |= ((value & 0b111) as u16) << 8;

        if self.shall_trigger {
            self.active = true;
        }

        self.wave_length_cycles = (2048 - self.wave_length) as f32;
        log::debug!(
            "new period {}, freq {}\nshall trigger {}\nsound length enable {}",
            self.wave_length_cycles,
            (65536. / (2048 - self.wave_length) as f32),
            self.shall_trigger,
            self.sound_length_enable,
        );
    }

    fn set_wave_pattern(&mut self, addr: u16, value: u8) {
        let index = (addr - memory::apu::WAVE_PATTERN_RAM.begin) as usize;
        self.wave_pattern_ram[index] = value;
        self.wave_pattern_vec[index * 2] = value >> 4;
        self.wave_pattern_vec[index * 2 + 1] = value & 0x0F;
    }
}

impl APUChannel for Wave {
    fn tick_timer(&mut self) {
        if self.timer == 255 {
            if self.sound_length_enable {
                self.active = false;
            }
        }
        self.timer = self.timer.wrapping_add(1);
    }

    fn tick_sampler(&mut self) {
        self.frame_index += 1;
        self.frame_index %= Wave::WAVE_PATTERN_FRAME_SIZE;
    }

    fn sample(&mut self, apu: &APU) {
        let digital_sample = self.wave_pattern_vec[self.frame_index]
            >> match self.output_level {
                WaveOutputLevel::Mute => 4,
                WaveOutputLevel::P100 => 0,
                WaveOutputLevel::P50 => 1,
                WaveOutputLevel::P25 => 2,
            };

        let analog_sample = self.dac(apu, digital_sample, self.dac_enabled);

        self.samples.push(analog_sample.0);
        self.samples.push(analog_sample.1);
    }

    fn get_samples(&mut self) -> &Vec<f32> {
        &self.samples
    }

    fn reset_samples(&mut self) {
        self.samples.clear();
    }

    fn is_active(&self) -> bool {
        self.active
    }
}
