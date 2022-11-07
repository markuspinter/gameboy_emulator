use super::memory;

mod noise;
mod pulse;
mod utils;
mod wave;

pub struct APU {
    pulse_sweep: pulse::PulseSweep,

    pulse: pulse::Pulse,

    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,

    noise: noise::Noise,

    nr50: u8,
    nr51: u8,
    nr52: u8,
}

impl super::MemoryInterface for APU {
    fn read8(&self, addr: u16) -> Option<u8> {
        if let Some(res) = self.pulse_sweep.read8(addr) {
            return Some(res);
        } else if let Some(res) = self.pulse.read8(addr) {
            return Some(res);
        } else if addr == memory::apu::NR30 {
            return Some(self.nr30);
        } else if addr == memory::apu::NR31 {
            return Some(self.nr31);
        } else if addr == memory::apu::NR32 {
            return Some(self.nr32);
        } else if addr == memory::apu::NR33 {
            return Some(self.nr33);
        } else if addr == memory::apu::NR34 {
            return Some(self.nr34);
        } else if let Some(res) = self.noise.read8(addr) {
            return Some(res);
        } else if addr == memory::apu::NR50 {
            return Some(self.nr50);
        } else if addr == memory::apu::NR51 {
            return Some(self.nr51);
        } else if addr == memory::apu::NR52 {
            return Some(self.nr52);
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if let Some(res) = self.pulse_sweep.write8(addr, value) {
        } else if let Some(res) = self.pulse.write8(addr, value) {
        } else if addr == memory::apu::NR30 {
            self.nr30 = value;
        } else if addr == memory::apu::NR31 {
            self.nr31 = value;
        } else if addr == memory::apu::NR32 {
            self.nr32 = value;
        } else if addr == memory::apu::NR33 {
            self.nr33 = value;
        } else if addr == memory::apu::NR34 {
            self.nr34 = value;
        } else if let Some(res) = self.noise.write8(addr, value) {
        } else if addr == memory::apu::NR50 {
            self.nr50 = value;
        } else if addr == memory::apu::NR51 {
            self.nr51 = value;
        } else if addr == memory::apu::NR52 {
            self.nr52 = value;
        } else {
            return None;
        }
        return Some(());
    }
}

impl APU {
    pub fn new() -> Self {
        Self {
            pulse_sweep: pulse::PulseSweep::new(),

            pulse: pulse::Pulse::new(),

            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,

            noise: noise::Noise::new(),

            nr50: 0,
            nr51: 0,
            nr52: 0,
        }
    }
}
