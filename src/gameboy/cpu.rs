use std::fmt::Error;

use super::{memory::Memory, GameboyModule, MemoryInterface};

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

pub struct CPU {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub e: u16,
    pub f: u16,
    pub h: u16,
    pub l: u16,

    pub pc: u16,
    pub sp: u16,

    pub halted: bool,
    pub interrupt_master_enable: bool,
}

impl GameboyModule for CPU {
    fn tick(&self, memory: &Memory) -> Result<u32, std::fmt::Error> {
        self.decode_execute(memory)
    }
}

impl CPU {
    fn decode_execute(&self, memory: &Memory) -> Result<u32, std::fmt::Error> {
        let mut opcode: u16 = match memory.read8(self.pc) {
            Ok(num) => u16::from(num),
            Err(_) => return Err(Error),
        };
        if opcode == 0xCB {
            opcode = match memory.read8(self.pc + 1) {
                Ok(num) => u16::from(num),
                Err(_) => return Err(Error),
            };
            opcode += 0x100;
        }
        Ok(instructions::execute_opcode(opcode, self, memory))
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
            pc: 0x0000,
            sp: 0x0000,

            halted: false,
            interrupt_master_enable: false,
        }
    }

    fn get_reg8(&self, reg: Register8) -> u16 {
        match reg {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    fn get_reg16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::PC => self.pc,
            Register16::SP => self.sp,
            Register16::AF => (self.a as u16) << 8 | self.f as u16,
            Register16::BC => (self.b as u16) << 8 | self.c as u16,
            Register16::DE => (self.d as u16) << 8 | self.e as u16,
            Register16::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    fn set_reg8(&self, reg: Register8, value: u16) {
        match reg {
            Register8::A => self.a = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::F => self.f = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        }
    }

    fn set_reg16(&self, reg: Register16, value: u16) {
        match reg {
            Register16::PC => self.pc = value,
            Register16::SP => self.sp = value,
            Register16::AF => {
                self.a = (value >> 8);
                self.f = value;
            }
            Register16::BC => {
                self.b = (value >> 8);
                self.c = value;
            }
            Register16::DE => {
                self.d = (value >> 8);
                self.e = value;
            }
            Register16::HL => {
                self.h = (value >> 8);
                self.l = value;
            }
        }
    }

    fn get_flag(&self, flag: Flag) -> u16 {
        (self.f >> (flag as u16)) & 1
    }

    fn set_flag(&self, flag: Flag, value: bool) {
        self.f = (self.f & !1 << (flag as u16)) | ((value as u16) << (flag as u16))
    }

    fn get_hl(&self) -> u16 {
        self.h << 8 | self.l
    }

    fn set_hl(&self, value: u16) {
        self.h = (value & 0xFF00) >> 8;
        self.l = value & 0x00FF;
    }

    fn get_bc(&self) -> u16 {
        self.b << 8 | self.c
    }

    fn set_bc(&self, value: u16) {
        self.b = (value & 0xFF00) >> 8;
        self.c = value & 0x00FF;
    }

    fn get_de(&self) -> u16 {
        self.d << 8 | self.e
    }

    fn set_de(&self, value: u16) {
        self.d = (value & 0xFF00) >> 8;
        self.e = value & 0x00FF;
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

    fn f_nc(&self) -> bool {
        (self.f & (1 << FLAGC)) == 0
    }

    fn f_nz(&self) -> bool {
        (self.f & (1 << FLAGZ)) == 0
    }
}
