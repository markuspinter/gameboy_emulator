use std::fmt::{self, Error};

use super::{memory::Memory, Gameboy, GameboyModule, MemoryInterface};

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

    pub halted: bool,
    pub interrupt_master_enable: bool,
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

impl CPU {
    fn decode_execute(&mut self, gb: &mut Gameboy) -> Result<u32, std::fmt::Error> {
        let mut opcode: u16 = match gb.read8(self.pc) {
            Ok(num) => u16::from(num),
            Err(_) => return Err(Error),
        };
        let cycles;
        if opcode == 0xCB {
            (self.pc, cycles) = instructions::execute_instruction_extension(self, gb);
            Ok(cycles as u32)
        } else {
            (self.pc, cycles) = instructions::execute_instruction(self, gb);
            Ok(cycles as u32)
        }
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

            halted: false,
            interrupt_master_enable: false,
        }
    }

    // fn _get_reg8(&self, reg: Register8) -> u16 {
    //     match reg {
    //         Register8::A => self.a,
    //         Register8::B => self.b,
    //         Register8::C => self.c,
    //         Register8::D => self.d,
    //         Register8::E => self.e,
    //         Register8::F => self.f,
    //         Register8::H => self.h,
    //         Register8::L => self.l,
    //     }
    // }

    // fn _get_reg16(&self, reg: Register16) -> u16 {
    //     match reg {
    //         Register16::PC => self.pc,
    //         Register16::SP => self.sp,
    //         Register16::AF => (self.a as u16) << 8 | self.f as u16,
    //         Register16::BC => (self.b as u16) << 8 | self.c as u16,
    //         Register16::DE => (self.d as u16) << 8 | self.e as u16,
    //         Register16::HL => (self.h as u16) << 8 | self.l as u16,
    //     }
    // }

    // fn _set_reg8(&mut self, reg: Register8, value: u16) {
    //     match reg {
    //         Register8::A => self.a = value,
    //         Register8::B => self.b = value,
    //         Register8::C => self.c = value,
    //         Register8::D => self.d = value,
    //         Register8::E => self.e = value,
    //         Register8::F => self.f = value,
    //         Register8::H => self.h = value,
    //         Register8::L => self.l = value,
    //     }
    // }

    // fn _set_reg16(&mut self, reg: Register16, value: u16) {
    //     match reg {
    //         Register16::PC => self.pc = value,
    //         Register16::SP => self.sp = value,
    //         Register16::AF => {
    //             self.a = value >> 8;
    //             self.f = value;
    //         }
    //         Register16::BC => {
    //             self.b = value >> 8;
    //             self.c = value;
    //         }
    //         Register16::DE => {
    //             self.d = value >> 8;
    //             self.e = value;
    //         }
    //         Register16::HL => {
    //             self.h = value >> 8;
    //             self.l = value;
    //         }
    //     }
    // }

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
