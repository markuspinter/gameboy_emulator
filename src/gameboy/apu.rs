use super::memory;

pub mod noise;
pub mod pulse;
pub mod wave;

pub struct APU {
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,

    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,

    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,

    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,

    nr50: u8,
    nr51: u8,
    nr52: u8,
}

impl super::MemoryInterface for APU {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::apu::NR10 {
            return Some(self.nr10);
        } else if addr == memory::apu::NR11 {
            return Some(self.nr11);
        } else if addr == memory::apu::NR12 {
            return Some(self.nr12);
        } else if addr == memory::apu::NR13 {
            return Some(self.nr13);
        } else if addr == memory::apu::NR14 {
            return Some(self.nr14);
        } else if addr == memory::apu::NR21 {
            return Some(self.nr21);
        } else if addr == memory::apu::NR22 {
            return Some(self.nr22);
        } else if addr == memory::apu::NR23 {
            return Some(self.nr23);
        } else if addr == memory::apu::NR24 {
            return Some(self.nr24);
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
        } else if addr == memory::apu::NR41 {
            return Some(self.nr41);
        } else if addr == memory::apu::NR42 {
            return Some(self.nr42);
        } else if addr == memory::apu::NR43 {
            return Some(self.nr43);
        } else if addr == memory::apu::NR44 {
            return Some(self.nr44);
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
        if addr == memory::apu::NR10 {
            self.nr10 = value;
        } else if addr == memory::apu::NR11 {
            self.nr11 = value;
        } else if addr == memory::apu::NR12 {
            self.nr12 = value;
        } else if addr == memory::apu::NR13 {
            self.nr13 = value;
        } else if addr == memory::apu::NR14 {
            self.nr14 = value;
        } else if addr == memory::apu::NR21 {
            self.nr21 = value;
        } else if addr == memory::apu::NR22 {
            self.nr22 = value;
        } else if addr == memory::apu::NR23 {
            self.nr23 = value;
        } else if addr == memory::apu::NR24 {
            self.nr24 = value;
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
        } else if addr == memory::apu::NR41 {
            self.nr41 = value;
        } else if addr == memory::apu::NR42 {
            self.nr42 = value;
        } else if addr == memory::apu::NR43 {
            self.nr43 = value;
        } else if addr == memory::apu::NR44 {
            self.nr44 = value;
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
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,

            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,

            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,

            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,

            nr50: 0,
            nr51: 0,
            nr52: 0,
        }
    }
}
