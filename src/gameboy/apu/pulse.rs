use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    bit,
    gameboy::{memory, GameboyModule, MemoryInterface},
};

use super::APUChannel;

#[derive(Clone, Debug, FromPrimitive)]
enum WaveDuty {
    P12_5 = 0b00,
    P25 = 0b01,
    P50 = 0b10,
    P75 = 0b11,
}
pub struct Pulse {
    dac_enabled: bool,

    wave_duty: WaveDuty,
    wave_length: u16,
    pulse_frame: [u8; 8],
    length_timer: u8,
    inital_envelope_volume: u8,
    envelope_increase: bool,
    sweep_pace: u8,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR21 expires)

    t_cycles: u16,
    timer: u8,
    active: bool,
    frame_index: usize,
    samples: Vec<f32>,

    frame_index_fraction: f32,
    frame_index_fraction_increment: f32,
    sample_rate: u32,
}

impl GameboyModule for Pulse {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        if self.t_cycles == 0 {
            self.sample();
            self.t_cycles = 5;
        }
        self.t_cycles -= 1;
        Ok(self.t_cycles as u32)
    }
}

impl MemoryInterface for Pulse {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::apu::NR21 {
            return Some(self.get_nr21());
        } else if addr == memory::apu::NR22 {
            return Some(self.get_nr22());
        } else if addr == memory::apu::NR23 {
            return Some(self.get_nr23());
        } else if addr == memory::apu::NR24 {
            return Some(self.get_nr24());
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::apu::NR21 {
            self.set_nr21(value);
        } else if addr == memory::apu::NR22 {
            self.set_nr22(value);
        } else if addr == memory::apu::NR23 {
            self.set_nr23(value);
        } else if addr == memory::apu::NR24 {
            self.set_nr24(value);
        } else {
            return None;
        }
        return Some(());
    }
}

impl Pulse {
    const PULSE_FRAME_SIZE: usize = 8;
    pub fn new(sample_rate: u32) -> Self {
        Self {
            dac_enabled: false,

            wave_duty: WaveDuty::P12_5,
            wave_length: 0,
            pulse_frame: [15, 0, 0, 0, 0, 0, 0, 0],

            length_timer: 0,
            inital_envelope_volume: 0,
            envelope_increase: false,
            sweep_pace: 0,

            shall_trigger: false,
            sound_length_enable: false,

            t_cycles: 0,
            timer: 0,
            active: false,
            frame_index: 0,
            samples: Vec::new(),

            sample_rate,
            frame_index_fraction: 0.,
            frame_index_fraction_increment: 0.,
        }
    }

    fn get_nr21(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.wave_duty as u8) << 6;
        byte |= self.length_timer & 0b11111;
        byte
    }
    fn get_nr22(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= self.inital_envelope_volume << 4;
        byte |= (self.envelope_increase as u8) << 3;
        byte |= self.sweep_pace & 0b111;
        byte
    }
    fn get_nr23(&self) -> u8 {
        (self.wave_length & 0xFF) as u8
    }
    fn get_nr24(&self) -> u8 {
        let mut byte: u8 = 0;
        //bit 7 trigger is read only returning 1s for this section
        byte |= 0xFF_u8 << 7;
        byte |= (self.sound_length_enable as u8) << 6;
        byte |= ((self.wave_length >> 8) & 0b111) as u8;
        byte
    }

    fn set_nr21(&mut self, value: u8) {
        self.wave_duty = FromPrimitive::from_u8(value >> 6).expect("couldn't convert wave duty");
        self.length_timer = value & 0b11111;

        self.pulse_frame = match self.wave_duty {
            WaveDuty::P12_5 => [15, 0, 0, 0, 0, 0, 0, 0],
            WaveDuty::P25 => [15, 15, 0, 0, 0, 0, 0, 0],
            WaveDuty::P50 => [15, 15, 15, 15, 0, 0, 0, 0],
            WaveDuty::P75 => [15, 15, 15, 15, 15, 15, 0, 0],
        }
    }
    fn set_nr22(&mut self, value: u8) {
        self.inital_envelope_volume = value >> 4;
        self.envelope_increase = bit!(value, 4) != 0;
        self.sweep_pace = value & 0b111;

        if value & 0xF8 == 0 {
            self.dac_enabled = false;
            self.active = false;
        } else {
            self.dac_enabled = true;
        }
    }
    fn set_nr23(&mut self, value: u8) {
        self.wave_length &= 0x0700;
        self.wave_length |= value as u16;

        self.frame_index_fraction_increment =
            (131072. / (2048 - self.wave_length) as f32) * (Pulse::PULSE_FRAME_SIZE as f32 / self.sample_rate as f32);
    }
    fn set_nr24(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;
        self.wave_length &= 0x00FF;
        self.wave_length |= ((value & 0b111) as u16) << 8;

        if self.shall_trigger {
            self.active = true;
        }

        self.frame_index_fraction_increment =
            (131072. / (2048 - self.wave_length) as f32) * (Pulse::PULSE_FRAME_SIZE as f32 / self.sample_rate as f32);
    }
}

impl APUChannel for Pulse {
    fn tick_timer(&mut self) {
        if self.timer == 63 {
            if self.sound_length_enable {
                self.active = false;
            }
        }
        self.timer = self.timer.wrapping_add(1);
    }

    fn sample(&mut self) {
        if self.samples.len() as f32 <= self.sample_rate as f32 * 0.016742 {
            self.frame_index_fraction += self.frame_index_fraction_increment;
            self.frame_index_fraction %= Pulse::PULSE_FRAME_SIZE as f32;

            self.frame_index = self.frame_index_fraction as usize;

            let digital_sample = std::cmp::min(self.pulse_frame[self.frame_index], self.inital_envelope_volume);

            if self.active {
                self.samples.push(Self::dac(digital_sample, self.dac_enabled));
            } else {
                self.samples.push(0.0);
            }
        }
    }

    fn get_samples(&mut self) -> &Vec<f32> {
        &self.samples
    }

    fn reset_samples(&mut self) {
        self.samples.clear();
    }
}

pub struct PulseSweep {
    sweep_pace_for_frequency: u8,
    sweep_decrease: bool,
    sweep_slope: u8,

    dac_enabled: bool,

    wave_duty: WaveDuty,
    wave_length: u16,
    pulse_frame: [u8; 8],

    length_timer: u8,
    inital_envelope_volume: u8,
    envelope_increase: bool,
    sweep_pace: u8,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR21 expires)

    t_cycles: u16,
    timer: u8,
    active: bool,
    frame_index: usize,
    samples: Vec<f32>,

    frame_index_fraction: f32,
    frame_index_fraction_increment: f32,
    sample_rate: u32,
}

impl GameboyModule for PulseSweep {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        if self.t_cycles == 0 {
            self.sample();
            self.t_cycles = 5;
        }
        self.t_cycles -= 1;
        Ok(self.t_cycles as u32)
    }
}

impl MemoryInterface for PulseSweep {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::apu::NR10 {
            return Some(self.get_nr10());
        } else if addr == memory::apu::NR11 {
            return Some(self.get_nr11());
        } else if addr == memory::apu::NR12 {
            return Some(self.get_nr12());
        } else if addr == memory::apu::NR13 {
            return Some(self.get_nr13());
        } else if addr == memory::apu::NR14 {
            return Some(self.get_nr14());
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::apu::NR10 {
            self.set_nr10(value);
        } else if addr == memory::apu::NR11 {
            self.set_nr11(value);
        } else if addr == memory::apu::NR12 {
            self.set_nr12(value);
        } else if addr == memory::apu::NR13 {
            self.set_nr13(value);
        } else if addr == memory::apu::NR14 {
            self.set_nr14(value);
        } else {
            return None;
        }
        return Some(());
    }
}

impl PulseSweep {
    const PULSE_SWEEP_FRAME_SIZE: usize = 8;
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sweep_pace_for_frequency: 0,
            sweep_decrease: false,
            sweep_slope: 0,

            dac_enabled: false,

            wave_duty: WaveDuty::P12_5,
            wave_length: 0,
            pulse_frame: [15, 0, 0, 0, 0, 0, 0, 0],

            length_timer: 0,
            inital_envelope_volume: 0,
            envelope_increase: false,
            sweep_pace: 0,

            shall_trigger: false,
            sound_length_enable: false,

            t_cycles: 0,
            timer: 0,
            active: false,
            frame_index: 0,
            samples: Vec::new(),

            sample_rate,
            frame_index_fraction: 0.,
            frame_index_fraction_increment: 0.,
        }
    }

    fn get_nr10(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= ((self.sweep_pace_for_frequency & 0b111) as u8) << 4;
        byte |= (self.sweep_decrease as u8) << 3;
        byte |= self.sweep_slope & 0b111;
        byte
    }

    fn get_nr11(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.wave_duty as u8) << 6;
        byte |= self.length_timer & 0b11111;
        byte
    }
    fn get_nr12(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= self.inital_envelope_volume << 4;
        byte |= (self.envelope_increase as u8) << 3;
        byte |= self.sweep_pace & 0b111;
        byte
    }
    fn get_nr13(&self) -> u8 {
        (self.wave_length & 0xFF) as u8
    }
    fn get_nr14(&self) -> u8 {
        let mut byte: u8 = 0;
        //bit 7 trigger is read only returning 1s for this section
        byte |= 0xFF_u8 << 7;
        byte |= (self.sound_length_enable as u8) << 6;
        byte |= ((self.wave_length >> 8) & 0b111) as u8;
        byte
    }

    fn set_nr10(&mut self, value: u8) {
        self.sweep_pace_for_frequency = (value >> 4) & 0b111;
        self.sweep_decrease = bit!(value, 3) != 0;
        self.sweep_slope = value & 0b111;
    }

    fn set_nr11(&mut self, value: u8) {
        self.wave_duty = FromPrimitive::from_u8(value >> 6).expect("couldn't convert wave duty");
        self.length_timer = value & 0b11111;

        self.pulse_frame = match self.wave_duty {
            WaveDuty::P12_5 => [15, 0, 0, 0, 0, 0, 0, 0],
            WaveDuty::P25 => [15, 15, 0, 0, 0, 0, 0, 0],
            WaveDuty::P50 => [15, 15, 15, 15, 0, 0, 0, 0],
            WaveDuty::P75 => [15, 15, 15, 15, 15, 15, 0, 0],
        }
    }
    fn set_nr12(&mut self, value: u8) {
        self.inital_envelope_volume = value >> 4;
        self.envelope_increase = bit!(value, 4) != 0;
        self.sweep_pace = value & 0b111;

        if value & 0xF8 == 0 {
            self.dac_enabled = false;
            self.active = false;
        } else {
            self.dac_enabled = true;
        }
    }
    fn set_nr13(&mut self, value: u8) {
        self.wave_length &= 0x0700;
        self.wave_length |= value as u16;

        self.frame_index_fraction_increment = (131072. / (2048 - self.wave_length) as f32)
            * (PulseSweep::PULSE_SWEEP_FRAME_SIZE as f32 / self.sample_rate as f32);
    }
    fn set_nr14(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;
        self.wave_length &= 0x00FF;
        self.wave_length |= ((value & 0b111) as u16) << 8;

        if self.shall_trigger {
            self.active = true;
        }

        self.frame_index_fraction_increment = (131072. / (2048 - self.wave_length) as f32)
            * (PulseSweep::PULSE_SWEEP_FRAME_SIZE as f32 / self.sample_rate as f32);
    }
}

impl APUChannel for PulseSweep {
    fn tick_timer(&mut self) {
        if self.timer == 63 {
            if self.sound_length_enable {
                self.active = false;
            }
        }
        self.timer = self.timer.wrapping_add(1);
    }

    fn sample(&mut self) {
        if self.samples.len() as f32 <= self.sample_rate as f32 * 0.016742 {
            self.frame_index_fraction += self.frame_index_fraction_increment;
            self.frame_index_fraction %= PulseSweep::PULSE_SWEEP_FRAME_SIZE as f32;

            self.frame_index = self.frame_index_fraction as usize;

            let digital_sample = std::cmp::min(self.pulse_frame[self.frame_index], self.inital_envelope_volume);

            if self.active {
                self.samples.push(Self::dac(digital_sample, self.dac_enabled));
            } else {
                self.samples.push(0.0);
            }
        }
    }

    fn get_samples(&mut self) -> &Vec<f32> {
        &self.samples
    }

    fn reset_samples(&mut self) {
        self.samples.clear();
    }
}
