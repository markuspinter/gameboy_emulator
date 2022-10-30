use std::fmt::{self, Error};

use self::instructions::InterruptRegister;

use super::{
    memory::{self, Memory},
    Gameboy, GameboyModule, MemoryInterface,
};

#[allow(non_snake_case)]
mod instructions;

const FLAGC: u16 = 4;
const FLAGH: u16 = 5;
const FLAGN: u16 = 6;
const FLAGZ: u16 = 7;

pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Register16 {
    PC,
    SP,
    AF,
    BC,
    DE,
    HL,
}

#[repr(u8)]
pub enum Flag {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[derive(Debug)]
pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,

    pub ie_register: InterruptRegister,
    pub if_register: InterruptRegister,

    pub halted: bool,
    pub interrupt_master_enable: bool,
    t_cycles: u16,
}

impl GameboyModule for CPU {
    unsafe fn tick(&mut self, gb_ptr: *mut Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        let ret = self.decode_execute(gb);
        log::debug!("{}", self);
        ret
    }
}

impl fmt::Display for CPU {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "CPU Status: {{\n\ta: {:#04X}\n\tb: {:#04X}\n\tc: {:#04X}\n\td: {:#04X}\n\te: {:#04X}\n\tf: {:#04X}\n\th: {:#04X}\n\tl: {:#04X}\n\tpc: {:#06X}\n\tsp: {:#06X}\n}}",
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l, self.pc, self.sp
        )
    }
}

impl super::MemoryInterface for CPU {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::interrupt::IE {
            return Some(u8::from((self.ie_register.clone())));
        } else if addr == memory::interrupt::IF {
            return Some(u8::from(self.if_register.clone()));
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::interrupt::IE {
            self.ie_register = value.into();
            log::info!("enable interrupt {:#010b}", value);
            return Some(());
        } else if addr == memory::interrupt::IF {
            self.if_register = value.into();
            return Some(());
        }
        return None;
    }
}

impl CPU {
    fn check_halted(&mut self) -> bool {
        if !self.halted {
            return true;
        }
        if (self.ie_register.vblank && self.if_register.vblank)
            || (self.ie_register.lcd_stat && self.if_register.lcd_stat)
            || (self.ie_register.timer && self.if_register.timer)
            || (self.ie_register.serial && self.if_register.serial)
            || (self.ie_register.joypad && self.if_register.joypad)
        {
            self.halted = false;
            return true;
        }
        false
    }

    fn decode_execute(&mut self, gb: &mut Gameboy) -> Result<u32, std::fmt::Error> {
        if self.t_cycles == 0 {
            if !self.check_halted() {
                return Ok(self.t_cycles as u32);
            }
            instructions::handle_int(self, gb);

            if gb.read8(self.pc) == 0xCB {
                (self.pc, self.t_cycles) = instructions::execute_instruction_extension(self, gb);
            } else {
                (self.pc, self.t_cycles) = instructions::execute_instruction(self, gb);
            }
        }
        self.t_cycles -= 1;
        Ok(self.t_cycles as u32)
    }

    pub fn new() -> Self {
        Self {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0100,
            sp: 0x0000,

            ie_register: InterruptRegister::from(0_u8),
            if_register: InterruptRegister::from(0_u8),

            halted: false,
            interrupt_master_enable: false,
            t_cycles: 0,
        }
    }

    fn _get_flag(&self, flag: Flag) -> u8 {
        (self.f >> (flag as u8)) & 1
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        self.f = (self.f & !(1 << (flag as u8))) | ((value as u8) << (flag as u8))
    }

    fn f_c(&self) -> bool {
        (self.f & (1 << FLAGC)) != 0
    }

    fn f_h(&self) -> bool {
        (self.f & (1 << FLAGH)) != 0
    }

    fn f_n(&self) -> bool {
        (self.f & (1 << FLAGN)) != 0
    }

    fn f_z(&self) -> bool {
        (self.f & (1 << FLAGZ)) != 0
    }
}
