use super::memory;

mod driver;
mod noise;
mod pulse;
mod utils;
mod wave;

pub struct APU {
    pulse_sweep: pulse::PulseSweep,

    pulse: pulse::Pulse,

    wave: wave::Wave,

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
        } else if let Some(res) = self.wave.read8(addr) {
            return Some(res);
        } else if let Some(res) = self.noise.read8(addr) {
            return Some(res);
        } else if addr == memory::apu::NR50 {
            return Some(self.nr50);
        } else if addr == memory::apu::NR51 {
            return Some(self.nr51);
        } else if addr == memory::apu::NR52 {
            log::trace!("read nr52");
            return Some(self.nr52);
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if let Some(res) = self.pulse_sweep.write8(addr, value) {
        } else if let Some(res) = self.pulse.write8(addr, value) {
        } else if let Some(res) = self.wave.write8(addr, value) {
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

            wave: wave::Wave::new(),

            noise: noise::Noise::new(),

            nr50: 0,
            nr51: 0,
            nr52: 0,
        }
    }
}
