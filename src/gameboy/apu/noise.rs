use crate::{
    bit,
    gameboy::{memory, GameboyModule, MemoryInterface},
};

use super::{APUChannel, APUEnvelope, APU};

#[derive(Copy, Clone)]
pub enum LFSRWidth {
    LFSR15Bits,
    LFSR7Bits,
}

pub struct Noise {
    dac_enabled: bool,

    length_timer: u8,
    inital_envelope_volume: u8,
    envelope_increase: bool,
    sweep_pace: u8,
    clock_shift: u8,
    lfsr_width: LFSRWidth,
    clock_divider: u8,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR41 expires)

    t_cycles: u16,
    timer: u8,
    active: bool,
    samples: Vec<f32>,

    lfsr: u16,

    curr_inital_envelope_volume: u8,
    curr_envelope_increase: bool,
    curr_sweep_pace: u8,
    sweep_volume: u8,
    envelope_tick: u8,

    wave_length_cycles: u16,
}

impl GameboyModule for Noise {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        let apu = &gb.apu;
        if self.t_cycles == 0 {
            self.tick_sampler();

            self.t_cycles = (self.wave_length_cycles * 16) + 1;
        }
        self.sample(&apu);
        self.t_cycles -= 1;
        Ok(self.t_cycles as u32)
    }
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
        Self {
            dac_enabled: false,

            length_timer: 0,
            inital_envelope_volume: 0,
            envelope_increase: false,
            sweep_pace: 0,
            clock_shift: 0,
            lfsr_width: LFSRWidth::LFSR15Bits,
            clock_divider: 0,
            shall_trigger: false,
            sound_length_enable: false,

            t_cycles: 0,
            timer: 0,
            active: false,
            samples: Vec::with_capacity(2048),

            lfsr: 0,

            curr_inital_envelope_volume: 0,
            curr_envelope_increase: false,
            curr_sweep_pace: 0,
            sweep_volume: 0,
            envelope_tick: 0,

            wave_length_cycles: 0,
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

        if value & 0xF8 == 0 {
            self.dac_enabled = false;
            self.active = false;
        } else {
            self.dac_enabled = true;
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

        let mut clock_divider = 2 * self.clock_divider;
        if self.clock_divider == 0 {
            clock_divider = 1;
        }

        self.wave_length_cycles = clock_divider as u16 * (1 << self.clock_shift as u16);
        // println!(
        //     "frame index incr {}, denom {}, freq {}",
        //     self.frame_index_fraction_increment,
        //     (clock_divider as u32 * (1 << self.clock_shift as u32)),
        //     (2.0 * 262144.) / ((clock_divider as u32 * (1 << self.clock_shift as u32)) as f32)
        // );
    }
    fn set_nr44(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;

        if self.shall_trigger {
            self.active = true;
            self.curr_envelope_increase = self.envelope_increase;
            self.curr_inital_envelope_volume = self.inital_envelope_volume;
            self.curr_sweep_pace = self.sweep_pace;
            self.sweep_volume = self.inital_envelope_volume;
        }
    }
}

impl APUChannel for Noise {
    fn tick_timer(&mut self) {
        if self.timer == 63 {
            if self.sound_length_enable {
                self.active = false;
            }
        }
        self.timer = self.timer.wrapping_add(1);
    }

    fn tick_sampler(&mut self) {
        let new_bit = !(bit!(self.lfsr, 0) ^ bit!(self.lfsr, 1)); //xnor operation
        self.lfsr = (self.lfsr & !(1 << 15)) | (new_bit << 15);
        if matches!(self.lfsr_width, LFSRWidth::LFSR7Bits) {
            self.lfsr = (self.lfsr & !(1 << 7)) | (new_bit << 7);
        }
        self.lfsr = self.lfsr >> 1;
    }

    fn sample(&mut self, apu: &APU) {
        let digital_sample = match (self.lfsr & 0b1) != 0 {
            true => self.sweep_volume,
            false => 0,
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

impl APUEnvelope for Noise {
    fn tick_envelope_sweep(&mut self) {
        if self.curr_sweep_pace > 0 {
            if self.envelope_tick == 0 {
                if self.curr_envelope_increase {
                    if self.sweep_volume == self.inital_envelope_volume {
                        self.sweep_volume = 1;
                    } else {
                        self.sweep_volume += 1;
                    }
                } else {
                    if self.sweep_volume <= 1 {
                        self.sweep_volume = self.inital_envelope_volume;
                    } else {
                        self.sweep_volume -= 1;
                    }
                }
                // println!("sweep_volume {}", self.sweep_volume);
                self.envelope_tick = self.curr_sweep_pace;
            } else {
                self.envelope_tick -= 1;
            }
        }
    }
}
