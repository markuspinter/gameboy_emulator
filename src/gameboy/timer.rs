use crate::bit;

use super::{memory, Gameboy, GameboyModule};

#[derive(Clone, Debug)]
enum TimerPrescaler {
    Presc1024 = 1024,
    Presc16 = 16,
    Presc64 = 64,
    Presc256 = 256,
}

#[derive(Clone, Debug)]
struct TimerControl {
    unused_bits: u8,
    enable: bool,
    prescaler: TimerPrescaler,
}

impl std::convert::From<TimerControl> for u8 {
    fn from(tac: TimerControl) -> u8 {
        let mut byte: u8 = 0x00;
        byte |= (tac.unused_bits & 0x1F) << 3;
        byte |= (tac.enable as u8) << 2;
        byte |= tac.prescaler as u8 & 0b11;
        byte
    }
}

impl std::convert::From<u8> for TimerControl {
    fn from(byte: u8) -> Self {
        Self {
            unused_bits: byte >> 3,
            enable: bit!(byte, 2) != 0,
            prescaler: match byte & 0b11 {
                0b00 => TimerPrescaler::Presc1024,
                0b01 => TimerPrescaler::Presc16,
                0b10 => TimerPrescaler::Presc64,
                0b11 => TimerPrescaler::Presc256,
                _ => panic!("TimerControl: prescaler > 3 should not exist"),
            },
        }
    }
}

pub struct Timer {
    div_timer_tick: u16,
    timer_tick: u16,
    glitch_tick: u16,

    // registers
    div: u8,
    tima: u8,
    tma: u8,
    tac: TimerControl,
}

impl GameboyModule for Timer {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        self.tick_div(gb);
        if self.tac.enable {
            self.tick_timer(gb);
        }
        Ok(0)
    }
}

impl super::MemoryInterface for Timer {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::timer::DIV {
            return Some(self.div);
        } else if addr == memory::timer::TIMA {
            return Some(self.tima);
        } else if addr == memory::timer::TMA {
            return Some(self.tma);
        } else if addr == memory::timer::TAC {
            return Some(u8::from(self.tac.clone()));
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::timer::DIV {
            //NOTE: check if timer prescaler is influenced by div and handle special case if so:
            //https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html
            // if self.tac.enable && matches!(self.tac.prescaler, TimerPrescaler::PRESC_1024) && (self.div & 0x02 != 0) {
            //     self.tick_tima();
            // }
            self.div = 0x00;
            return Some(());
        } else if addr == memory::timer::TIMA {
            //TODO: handle advanced time write
            self.tima = value;
            return Some(());
        } else if addr == memory::timer::TMA {
            self.tma = value;
            return Some(());
        } else if addr == memory::timer::TAC {
            self.tac = value.into();
            return Some(());
        }

        return None;
    }
}

impl Timer {
    const DIV_PRESCALER: u16 = 256;

    pub fn new() -> Self {
        Self {
            div_timer_tick: 0,
            timer_tick: 0,
            glitch_tick: 0,
            div: 0,
            tima: 0,
            tma: 0,
            tac: TimerControl::from(0),
        }
    }

    fn tick_tima(&mut self) {
        self.tima = self.tima.wrapping_add(1);
        if self.tima == 0 {
            self.glitch_tick = 4;
        }
    }
    fn tick_timer(&mut self, gb: &mut Gameboy) {
        if self.glitch_tick == 0 {
            self.timer_tick += 1;

            self.timer_tick %= self.tac.prescaler as u16;
            if self.timer_tick == 0 {
                self.tick_tima();
            }
        } else {
            self.glitch_tick -= 1;
            if self.glitch_tick == 0 {
                self.tima = self.tma;
                // if gb.cpu.interrupt_master_enable {
                gb.cpu.if_register.timer = true;
                // }
            }
        }
    }

    fn tick_div(&mut self, gb: &mut Gameboy) {
        self.div_timer_tick += 1;

        self.div_timer_tick %= Timer::DIV_PRESCALER;
        if self.div_timer_tick == 0 {
            let prev_div = self.div;
            self.div = self.div.wrapping_add(1);

            if (prev_div >> 4) & 0b1 == 1 && (self.div >> 4) & 0b1 == 1 {
                gb.apu.tick_div();
            }
        }
    }
}
