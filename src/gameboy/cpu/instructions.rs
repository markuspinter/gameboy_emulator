use crate::{
    bit,
    gameboy::{cpu::Flag, Gameboy, MemoryInterface},
};

use super::CPU;

#[derive(Debug)]
enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    SP_H,
    SP_L,
}

fn split_Reg16(r: &Reg16) -> (Reg8, Reg8) {
    match r {
        Reg16::BC => (Reg8::B, Reg8::C),
        Reg16::DE => (Reg8::D, Reg8::E),
        Reg16::HL => (Reg8::H, Reg8::L),
        Reg16::AF => (Reg8::A, Reg8::F),
        Reg16::SP => (Reg8::SP_H, Reg8::SP_L),
    }
}

#[derive(Debug)]
enum Func1 {
    INC,
    DEC,
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SRL,
    SWAP,
}

#[derive(Debug)]
enum Func2 {
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    BIT,
    RES,
    SET,
}

fn reg_get16(cpu: &CPU, reg: &Reg16) -> u16 {
    match reg {
        Reg16::AF => ((cpu.a as u16) << 8) | reg_get8(cpu, &Reg8::F) as u16,
        Reg16::BC => ((cpu.b as u16) << 8) | cpu.c as u16,
        Reg16::DE => ((cpu.d as u16) << 8) | cpu.e as u16,
        Reg16::HL => ((cpu.h as u16) << 8) | cpu.l as u16,
        Reg16::SP => cpu.sp,
    }
}

fn reg_set16(cpu: &mut CPU, reg: &Reg16, val: u16) {
    let low = (val & 0xFF) as u8;
    let high = (val >> 8) as u8;
    match reg {
        Reg16::AF => {
            reg_set8(cpu, &Reg8::A, high);
            reg_set8(cpu, &Reg8::F, low)
        }
        Reg16::BC => {
            cpu.b = high;
            cpu.c = low
        }
        Reg16::DE => {
            cpu.d = high;
            cpu.e = low
        }
        Reg16::HL => {
            cpu.h = high;
            cpu.l = low
        }
        Reg16::SP => {
            cpu.sp = val;
        }
    }
}

fn reg_get8(cpu: &CPU, reg: &Reg8) -> u8 {
    match reg {
        Reg8::A => cpu.a,
        Reg8::F => cpu.f,
        Reg8::B => cpu.b,
        Reg8::C => cpu.c,
        Reg8::D => cpu.d,
        Reg8::E => cpu.e,
        Reg8::H => cpu.h,
        Reg8::L => cpu.l,
        Reg8::SP_H => ((cpu.sp >> 8) & 0x00FF) as u8,
        Reg8::SP_L => (cpu.sp & 0x00FF) as u8,
    }
}

fn reg_set8(cpu: &mut CPU, reg: &Reg8, val: u8) {
    match reg {
        Reg8::A => cpu.a = val,
        Reg8::F => cpu.f = val & 0xF0,
        Reg8::B => cpu.b = val,
        Reg8::C => cpu.c = val,
        Reg8::D => cpu.d = val,
        Reg8::E => cpu.e = val,
        Reg8::H => cpu.h = val,
        Reg8::L => cpu.l = val,
        Reg8::SP_H => cpu.sp = (cpu.sp & 0x00FF) | ((val as u16) << 8),
        Reg8::SP_L => cpu.sp = (cpu.sp & 0xFF00) | (val as u16),
    }
}

fn reg_inc16(cpu: &mut CPU, reg: &Reg16) {
    let val = reg_get16(cpu, reg);
    let val = val.wrapping_add(1);
    reg_set16(cpu, reg, val);
}

fn reg_inc8(cpu: &mut CPU, reg: &Reg8) {
    let val = reg_get8(cpu, reg);
    let val = val.wrapping_add(1);
    reg_set8(cpu, reg, val);
}

fn reg_dec16(cpu: &mut CPU, reg: &Reg16) {
    let val = reg_get16(cpu, reg);
    let val = val.wrapping_sub(1);
    reg_set16(cpu, reg, val);
}

fn reg_dec8(cpu: &mut CPU, reg: &Reg8) {
    let val = reg_get8(cpu, reg);
    let val = val.wrapping_sub(1);
    reg_set8(cpu, reg, val);
}

#[derive(Debug)]
pub enum Interrupt {
    VBLANK,
    LCD_STAT,
    TIMER,
    SERIAL,
    JOYPAD,
}

#[derive(Debug)]
pub struct InterruptRegister {
    vblank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl std::convert::From<InterruptRegister> for u8 {
    fn from(ir: InterruptRegister) -> u8 {
        let mut byte: u8 = 0x00;
        byte |= (ir.joypad as u8) << 4;
        byte |= (ir.serial as u8) << 3;
        byte |= (ir.timer as u8) << 2;
        byte |= (ir.lcd_stat as u8) << 1;
        byte |= (ir.vblank as u8);
        byte
    }
}

impl std::convert::From<u8> for InterruptRegister {
    fn from(byte: u8) -> Self {
        Self {
            vblank: bit!(byte, 0) != 0,
            lcd_stat: bit!(byte, 1) != 0,
            timer: bit!(byte, 2) != 0,
            serial: bit!(byte, 3) != 0,
            joypad: bit!(byte, 4) != 0,
        }
    }
}

pub fn set_int(cpu: &mut CPU, int: Interrupt) {
    match int {
        Interrupt::VBLANK => cpu.if_register.vblank = true,
        Interrupt::LCD_STAT => cpu.if_register.lcd_stat = true,
        Interrupt::TIMER => cpu.if_register.timer = true,
        Interrupt::SERIAL => cpu.if_register.serial = true,
        Interrupt::JOYPAD => cpu.if_register.joypad = true,
    }
}

pub fn handle_int(cpu: &mut CPU, gb: &mut Gameboy) {
    if cpu.interrupt_master_enable {
        if cpu.ie_register.vblank && cpu.if_register.vblank {
            cpu.if_register.vblank = false;
            execute_int(cpu, 0x40, gb);
        } else if cpu.ie_register.lcd_stat && cpu.if_register.lcd_stat {
            cpu.if_register.lcd_stat = false;
            execute_int(cpu, 0x48, gb);
        } else if cpu.ie_register.timer && cpu.if_register.timer {
            cpu.if_register.timer = false;
            execute_int(cpu, 0x50, gb);
        } else if cpu.ie_register.serial && cpu.if_register.serial {
            cpu.if_register.serial = false;
            execute_int(cpu, 0x58, gb);
        } else if cpu.ie_register.joypad && cpu.if_register.joypad {
            cpu.if_register.joypad = false;
            execute_int(cpu, 0x60, gb);
        }
    }
}

fn execute_int(cpu: &mut CPU, address: u16, gb: &mut Gameboy) {
    cpu.interrupt_master_enable = false;
    _push(cpu, cpu.pc, gb);
    cpu.pc = address;
}

pub fn execute_instruction_extension(cpu: &mut CPU, gb: &mut Gameboy) -> (u16, u16) {
    let instr = gb.read8(cpu.pc.wrapping_add(1)).unwrap();

    log::debug!(
        "{:#06X}: 0xCB opcode {:#04X} | {}",
        cpu.pc,
        instr,
        CPU_COMMANDS[instr as usize + 0x100]
    );

    /* This is really dumb: We decode the instruction by just matching all 256 possibilities (and again for the CB extension).
                            I was too lazy to find out the the bit relationships and patterns. ¯\_(ツ)_/¯¯
    */

    let _ = match instr {
        0x00 => opf18(cpu, &Func1::RLC, &Reg8::B),
        0x01 => opf18(cpu, &Func1::RLC, &Reg8::C),
        0x02 => opf18(cpu, &Func1::RLC, &Reg8::D),
        0x03 => opf18(cpu, &Func1::RLC, &Reg8::E),
        0x04 => opf18(cpu, &Func1::RLC, &Reg8::H),
        0x05 => opf18(cpu, &Func1::RLC, &Reg8::L),
        0x06 => opf1m8(cpu, &Func1::RLC, &Reg16::HL, gb),
        0x07 => opf18(cpu, &Func1::RLC, &Reg8::A),

        0x08 => opf18(cpu, &Func1::RRC, &Reg8::B),
        0x09 => opf18(cpu, &Func1::RRC, &Reg8::C),
        0x0A => opf18(cpu, &Func1::RRC, &Reg8::D),
        0x0B => opf18(cpu, &Func1::RRC, &Reg8::E),
        0x0C => opf18(cpu, &Func1::RRC, &Reg8::H),
        0x0D => opf18(cpu, &Func1::RRC, &Reg8::L),
        0x0E => opf1m8(cpu, &Func1::RRC, &Reg16::HL, gb),
        0x0F => opf18(cpu, &Func1::RRC, &Reg8::A),

        0x10 => opf18(cpu, &Func1::RL, &Reg8::B),
        0x11 => opf18(cpu, &Func1::RL, &Reg8::C),
        0x12 => opf18(cpu, &Func1::RL, &Reg8::D),
        0x13 => opf18(cpu, &Func1::RL, &Reg8::E),
        0x14 => opf18(cpu, &Func1::RL, &Reg8::H),
        0x15 => opf18(cpu, &Func1::RL, &Reg8::L),
        0x16 => opf1m8(cpu, &Func1::RL, &Reg16::HL, gb),
        0x17 => opf18(cpu, &Func1::RL, &Reg8::A),

        0x18 => opf18(cpu, &Func1::RR, &Reg8::B),
        0x19 => opf18(cpu, &Func1::RR, &Reg8::C),
        0x1A => opf18(cpu, &Func1::RR, &Reg8::D),
        0x1B => opf18(cpu, &Func1::RR, &Reg8::E),
        0x1C => opf18(cpu, &Func1::RR, &Reg8::H),
        0x1D => opf18(cpu, &Func1::RR, &Reg8::L),
        0x1E => opf1m8(cpu, &Func1::RR, &Reg16::HL, gb),
        0x1F => opf18(cpu, &Func1::RR, &Reg8::A),

        0x20 => opf18(cpu, &Func1::SLA, &Reg8::B),
        0x21 => opf18(cpu, &Func1::SLA, &Reg8::C),
        0x22 => opf18(cpu, &Func1::SLA, &Reg8::D),
        0x23 => opf18(cpu, &Func1::SLA, &Reg8::E),
        0x24 => opf18(cpu, &Func1::SLA, &Reg8::H),
        0x25 => opf18(cpu, &Func1::SLA, &Reg8::L),
        0x26 => opf1m8(cpu, &Func1::SLA, &Reg16::HL, gb),
        0x27 => opf18(cpu, &Func1::SLA, &Reg8::A),

        0x28 => opf18(cpu, &Func1::SRA, &Reg8::B),
        0x29 => opf18(cpu, &Func1::SRA, &Reg8::C),
        0x2A => opf18(cpu, &Func1::SRA, &Reg8::D),
        0x2B => opf18(cpu, &Func1::SRA, &Reg8::E),
        0x2C => opf18(cpu, &Func1::SRA, &Reg8::H),
        0x2D => opf18(cpu, &Func1::SRA, &Reg8::L),
        0x2E => opf1m8(cpu, &Func1::SRA, &Reg16::HL, gb),
        0x2F => opf18(cpu, &Func1::SRA, &Reg8::A),

        0x30 => opf18(cpu, &Func1::SWAP, &Reg8::B),
        0x31 => opf18(cpu, &Func1::SWAP, &Reg8::C),
        0x32 => opf18(cpu, &Func1::SWAP, &Reg8::D),
        0x33 => opf18(cpu, &Func1::SWAP, &Reg8::E),
        0x34 => opf18(cpu, &Func1::SWAP, &Reg8::H),
        0x35 => opf18(cpu, &Func1::SWAP, &Reg8::L),
        0x36 => opf1m8(cpu, &Func1::SWAP, &Reg16::HL, gb),
        0x37 => opf18(cpu, &Func1::SWAP, &Reg8::A),

        0x38 => opf18(cpu, &Func1::SRL, &Reg8::B),
        0x39 => opf18(cpu, &Func1::SRL, &Reg8::C),
        0x3A => opf18(cpu, &Func1::SRL, &Reg8::D),
        0x3B => opf18(cpu, &Func1::SRL, &Reg8::E),
        0x3C => opf18(cpu, &Func1::SRL, &Reg8::H),
        0x3D => opf18(cpu, &Func1::SRL, &Reg8::L),
        0x3E => opf1m8(cpu, &Func1::SRL, &Reg16::HL, gb),
        0x3F => opf18(cpu, &Func1::SRL, &Reg8::A),

        0x40 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x00),
        0x41 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x00),
        0x42 => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x00),
        0x43 => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x00),
        0x44 => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x00),
        0x45 => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x00),
        0x46 => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x00, gb),
        0x47 => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x00),

        0x48 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x01),
        0x49 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x01),
        0x4A => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x01),
        0x4B => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x01),
        0x4C => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x01),
        0x4D => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x01),
        0x4E => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x01, gb),
        0x4F => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x01),

        0x50 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x02),
        0x51 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x02),
        0x52 => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x02),
        0x53 => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x02),
        0x54 => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x02),
        0x55 => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x02),
        0x56 => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x02, gb),
        0x57 => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x02),
        0x58 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x03),
        0x59 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x03),
        0x5A => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x03),
        0x5B => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x03),
        0x5C => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x03),
        0x5D => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x03),
        0x5E => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x03, gb),
        0x5F => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x03),

        0x60 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x04),
        0x61 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x04),
        0x62 => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x04),
        0x63 => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x04),
        0x64 => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x04),
        0x65 => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x04),
        0x66 => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x04, gb),
        0x67 => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x04),
        0x68 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x05),
        0x69 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x05),
        0x6A => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x05),
        0x6B => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x05),
        0x6C => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x05),
        0x6D => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x05),
        0x6E => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x05, gb),
        0x6F => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x05),

        0x70 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x06),
        0x71 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x06),
        0x72 => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x06),
        0x73 => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x06),
        0x74 => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x06),
        0x75 => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x06),
        0x76 => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x06, gb),
        0x77 => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x06),
        0x78 => opf2h8(cpu, &Func2::BIT, &Reg8::B, 0x07),
        0x79 => opf2h8(cpu, &Func2::BIT, &Reg8::C, 0x07),
        0x7A => opf2h8(cpu, &Func2::BIT, &Reg8::D, 0x07),
        0x7B => opf2h8(cpu, &Func2::BIT, &Reg8::E, 0x07),
        0x7C => opf2h8(cpu, &Func2::BIT, &Reg8::H, 0x07),
        0x7D => opf2h8(cpu, &Func2::BIT, &Reg8::L, 0x07),
        0x7E => opf2mh8(cpu, &Func2::BIT, &Reg16::HL, 0x07, gb),
        0x7F => opf2h8(cpu, &Func2::BIT, &Reg8::A, 0x07),

        0x80 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x00),
        0x81 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x00),
        0x82 => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x00),
        0x83 => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x00),
        0x84 => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x00),
        0x85 => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x00),
        0x86 => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x00, gb),
        0x87 => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x00),
        0x88 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x01),
        0x89 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x01),
        0x8A => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x01),
        0x8B => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x01),
        0x8C => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x01),
        0x8D => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x01),
        0x8E => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x01, gb),
        0x8F => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x01),
        0x90 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x02),
        0x91 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x02),
        0x92 => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x02),
        0x93 => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x02),
        0x94 => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x02),
        0x95 => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x02),
        0x96 => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x02, gb),
        0x97 => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x02),
        0x98 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x03),
        0x99 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x03),
        0x9A => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x03),
        0x9B => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x03),
        0x9C => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x03),
        0x9D => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x03),
        0x9E => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x03, gb),
        0x9F => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x03),
        0xA0 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x04),
        0xA1 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x04),
        0xA2 => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x04),
        0xA3 => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x04),
        0xA4 => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x04),
        0xA5 => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x04),
        0xA6 => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x04, gb),
        0xA7 => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x04),
        0xA8 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x05),
        0xA9 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x05),
        0xAA => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x05),
        0xAB => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x05),
        0xAC => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x05),
        0xAD => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x05),
        0xAE => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x05, gb),
        0xAF => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x05),
        0xB0 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x06),
        0xB1 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x06),
        0xB2 => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x06),
        0xB3 => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x06),
        0xB4 => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x06),
        0xB5 => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x06),
        0xB6 => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x06, gb),
        0xB7 => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x06),
        0xB8 => opf2h8(cpu, &Func2::RES, &Reg8::B, 0x07),
        0xB9 => opf2h8(cpu, &Func2::RES, &Reg8::C, 0x07),
        0xBA => opf2h8(cpu, &Func2::RES, &Reg8::D, 0x07),
        0xBB => opf2h8(cpu, &Func2::RES, &Reg8::E, 0x07),
        0xBC => opf2h8(cpu, &Func2::RES, &Reg8::H, 0x07),
        0xBD => opf2h8(cpu, &Func2::RES, &Reg8::L, 0x07),
        0xBE => opf2mh8(cpu, &Func2::RES, &Reg16::HL, 0x07, gb),
        0xBF => opf2h8(cpu, &Func2::RES, &Reg8::A, 0x07),

        0xC0 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x00),
        0xC1 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x00),
        0xC2 => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x00),
        0xC3 => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x00),
        0xC4 => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x00),
        0xC5 => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x00),
        0xC6 => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x00, gb),
        0xC7 => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x00),
        0xC8 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x01),
        0xC9 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x01),
        0xCA => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x01),
        0xCB => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x01),
        0xCC => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x01),
        0xCD => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x01),
        0xCE => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x01, gb),
        0xCF => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x01),
        0xD0 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x02),
        0xD1 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x02),
        0xD2 => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x02),
        0xD3 => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x02),
        0xD4 => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x02),
        0xD5 => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x02),
        0xD6 => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x02, gb),
        0xD7 => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x02),
        0xD8 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x03),
        0xD9 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x03),
        0xDA => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x03),
        0xDB => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x03),
        0xDC => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x03),
        0xDD => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x03),
        0xDE => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x03, gb),
        0xDF => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x03),
        0xE0 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x04),
        0xE1 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x04),
        0xE2 => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x04),
        0xE3 => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x04),
        0xE4 => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x04),
        0xE5 => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x04),
        0xE6 => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x04, gb),
        0xE7 => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x04),
        0xE8 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x05),
        0xE9 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x05),
        0xEA => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x05),
        0xEB => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x05),
        0xEC => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x05),
        0xED => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x05),
        0xEE => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x05, gb),
        0xEF => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x05),
        0xF0 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x06),
        0xF1 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x06),
        0xF2 => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x06),
        0xF3 => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x06),
        0xF4 => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x06),
        0xF5 => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x06),
        0xF6 => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x06, gb),
        0xF7 => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x06),
        0xF8 => opf2h8(cpu, &Func2::SET, &Reg8::B, 0x07),
        0xF9 => opf2h8(cpu, &Func2::SET, &Reg8::C, 0x07),
        0xFA => opf2h8(cpu, &Func2::SET, &Reg8::D, 0x07),
        0xFB => opf2h8(cpu, &Func2::SET, &Reg8::E, 0x07),
        0xFC => opf2h8(cpu, &Func2::SET, &Reg8::H, 0x07),
        0xFD => opf2h8(cpu, &Func2::SET, &Reg8::L, 0x07),
        0xFE => opf2mh8(cpu, &Func2::SET, &Reg16::HL, 0x07, gb),
        0xFF => opf2h8(cpu, &Func2::SET, &Reg8::A, 0x07),
    };
    let lowh = instr & 0xF;
    let tcycles = match lowh {
        0x6 | 0xE => 16,
        _ => 8,
    };
    (cpu.pc.wrapping_add(2), tcycles)
}

pub fn execute_instruction(cpu: &mut CPU, gb: &mut Gameboy) -> (u16, u16) {
    let pc = cpu.pc;
    let instr = gb.read8(pc).unwrap();

    log::debug!(
        "{:#06X}: opcode {:#04X} | {}",
        cpu.pc,
        instr,
        CPU_COMMANDS[instr as usize]
    );

    /* This is really dumb: We decode the instruction by just matching all 256 possibilities (and again for the CB extension).
                            I was too lazy to find out the the bit relationships and patterns. ¯\_(ツ)_/¯¯
    */

    let new_pc = match instr {
        0x00 => (cpu.pc.wrapping_add(1), 4), /* NOP */
        0x01 => ldi16(cpu, &Reg16::BC, gb),
        0x02 => sd8(cpu, &Reg16::BC, &Reg8::A, gb),
        0x03 => inc16(cpu, &Reg16::BC),
        0x04 => opf18(cpu, &Func1::INC, &Reg8::B),
        0x05 => opf18(cpu, &Func1::DEC, &Reg8::B),
        0x06 => ldi8(cpu, &Reg8::B, gb),
        0x07 => opf18(cpu, &Func1::RLC, &Reg8::A),
        0x08 => saveSP(cpu, gb),
        0x09 => add16(cpu, &Reg16::HL, &Reg16::BC),
        0x0A => ld8(cpu, &Reg8::A, &Reg16::BC, gb),
        0x0B => dec16(cpu, &Reg16::BC),
        0x0C => opf18(cpu, &Func1::INC, &Reg8::C),
        0x0D => opf18(cpu, &Func1::DEC, &Reg8::C),
        0x0E => ldi8(cpu, &Reg8::C, gb),
        0x0F => opf18(cpu, &Func1::RRC, &Reg8::A),

        0x10 => (cpu.pc.wrapping_add(2), 4), //TODO: support cgb mode
        0x11 => ldi16(cpu, &Reg16::DE, gb),
        0x12 => sd8(cpu, &Reg16::DE, &Reg8::A, gb),
        0x13 => inc16(cpu, &Reg16::DE),
        0x14 => opf18(cpu, &Func1::INC, &Reg8::D),
        0x15 => opf18(cpu, &Func1::DEC, &Reg8::D),
        0x16 => ldi8(cpu, &Reg8::D, gb),
        0x17 => opf18(cpu, &Func1::RL, &Reg8::A),
        0x18 => jr(cpu, gb),
        0x19 => add16(cpu, &Reg16::HL, &Reg16::DE),
        0x1A => ld8(cpu, &Reg8::A, &Reg16::DE, gb),
        0x1B => dec16(cpu, &Reg16::DE),
        0x1C => opf18(cpu, &Func1::INC, &Reg8::E),
        0x1D => opf18(cpu, &Func1::DEC, &Reg8::E),
        0x1E => ldi8(cpu, &Reg8::E, gb),
        0x1F => opf18(cpu, &Func1::RR, &Reg8::A),

        0x20 => jr_cond(cpu, !cpu.f_z(), gb),
        0x21 => ldi16(cpu, &Reg16::HL, gb),
        0x22 => sdinc8(cpu, &Reg16::HL, &Reg8::A, gb),
        0x23 => inc16(cpu, &Reg16::HL),
        0x24 => opf18(cpu, &Func1::INC, &Reg8::H),
        0x25 => opf18(cpu, &Func1::DEC, &Reg8::H),
        0x26 => ldi8(cpu, &Reg8::H, gb),
        0x27 => daa(cpu),
        0x28 => jr_cond(cpu, cpu.f_z(), gb),
        0x29 => add16(cpu, &Reg16::HL, &Reg16::HL),
        0x2A => ldinc8(cpu, &Reg8::A, &Reg16::HL, gb),
        0x2B => dec16(cpu, &Reg16::HL),
        0x2C => opf18(cpu, &Func1::INC, &Reg8::L),
        0x2D => opf18(cpu, &Func1::DEC, &Reg8::L),
        0x2E => ldi8(cpu, &Reg8::L, gb),
        0x2F => cpl_Akku(cpu),

        0x30 => jr_cond(cpu, !cpu.f_c(), gb),
        0x31 => ldi16(cpu, &Reg16::SP, gb),
        0x32 => sddec8(cpu, &Reg16::HL, &Reg8::A, gb),
        0x33 => inc16(cpu, &Reg16::SP),
        0x34 => opf1m8(cpu, &Func1::INC, &Reg16::HL, gb),
        0x35 => opf1m8(cpu, &Func1::DEC, &Reg16::HL, gb),
        0x36 => sdi8(cpu, &Reg16::HL, gb),
        0x37 => scf(cpu),
        0x38 => jr_cond(cpu, cpu.f_c(), gb),
        0x39 => add16(cpu, &Reg16::HL, &Reg16::SP),
        0x3A => lddec8(cpu, &Reg8::A, &Reg16::HL, gb),
        0x3B => dec16(cpu, &Reg16::SP),
        0x3C => opf18(cpu, &Func1::INC, &Reg8::A),
        0x3D => opf18(cpu, &Func1::DEC, &Reg8::A),
        0x3E => ldi8(cpu, &Reg8::A, gb),
        0x3F => ccf(cpu),

        0x40 => mov8(cpu, &Reg8::B, &Reg8::B),
        0x41 => mov8(cpu, &Reg8::B, &Reg8::C),
        0x42 => mov8(cpu, &Reg8::B, &Reg8::D),
        0x43 => mov8(cpu, &Reg8::B, &Reg8::E),
        0x44 => mov8(cpu, &Reg8::B, &Reg8::H),
        0x45 => mov8(cpu, &Reg8::B, &Reg8::L),
        0x46 => ld8(cpu, &Reg8::B, &Reg16::HL, gb),
        0x47 => mov8(cpu, &Reg8::B, &Reg8::A),

        0x48 => mov8(cpu, &Reg8::C, &Reg8::B),
        0x49 => mov8(cpu, &Reg8::C, &Reg8::C),
        0x4A => mov8(cpu, &Reg8::C, &Reg8::D),
        0x4B => mov8(cpu, &Reg8::C, &Reg8::E),
        0x4C => mov8(cpu, &Reg8::C, &Reg8::H),
        0x4D => mov8(cpu, &Reg8::C, &Reg8::L),
        0x4E => ld8(cpu, &Reg8::C, &Reg16::HL, gb),
        0x4F => mov8(cpu, &Reg8::C, &Reg8::A),

        0x50 => mov8(cpu, &Reg8::D, &Reg8::B),
        0x51 => mov8(cpu, &Reg8::D, &Reg8::C),
        0x52 => mov8(cpu, &Reg8::D, &Reg8::D),
        0x53 => mov8(cpu, &Reg8::D, &Reg8::E),
        0x54 => mov8(cpu, &Reg8::D, &Reg8::H),
        0x55 => mov8(cpu, &Reg8::D, &Reg8::L),
        0x56 => ld8(cpu, &Reg8::D, &Reg16::HL, gb),
        0x57 => mov8(cpu, &Reg8::D, &Reg8::A),

        0x58 => mov8(cpu, &Reg8::E, &Reg8::B),
        0x59 => mov8(cpu, &Reg8::E, &Reg8::C),
        0x5A => mov8(cpu, &Reg8::E, &Reg8::D),
        0x5B => mov8(cpu, &Reg8::E, &Reg8::E),
        0x5C => mov8(cpu, &Reg8::E, &Reg8::H),
        0x5D => mov8(cpu, &Reg8::E, &Reg8::L),
        0x5E => ld8(cpu, &Reg8::E, &Reg16::HL, gb),
        0x5F => mov8(cpu, &Reg8::E, &Reg8::A),

        0x60 => mov8(cpu, &Reg8::H, &Reg8::B),
        0x61 => mov8(cpu, &Reg8::H, &Reg8::C),
        0x62 => mov8(cpu, &Reg8::H, &Reg8::D),
        0x63 => mov8(cpu, &Reg8::H, &Reg8::E),
        0x64 => mov8(cpu, &Reg8::H, &Reg8::H),
        0x65 => mov8(cpu, &Reg8::H, &Reg8::L),
        0x66 => ld8(cpu, &Reg8::H, &Reg16::HL, gb),
        0x67 => mov8(cpu, &Reg8::H, &Reg8::A),

        0x68 => mov8(cpu, &Reg8::L, &Reg8::B),
        0x69 => mov8(cpu, &Reg8::L, &Reg8::C),
        0x6A => mov8(cpu, &Reg8::L, &Reg8::D),
        0x6B => mov8(cpu, &Reg8::L, &Reg8::E),
        0x6C => mov8(cpu, &Reg8::L, &Reg8::H),
        0x6D => mov8(cpu, &Reg8::L, &Reg8::L),
        0x6E => ld8(cpu, &Reg8::L, &Reg16::HL, gb),
        0x6F => mov8(cpu, &Reg8::L, &Reg8::A),

        0x70 => sd8(cpu, &Reg16::HL, &Reg8::B, gb),
        0x71 => sd8(cpu, &Reg16::HL, &Reg8::C, gb),
        0x72 => sd8(cpu, &Reg16::HL, &Reg8::D, gb),
        0x73 => sd8(cpu, &Reg16::HL, &Reg8::E, gb),
        0x74 => sd8(cpu, &Reg16::HL, &Reg8::H, gb),
        0x75 => sd8(cpu, &Reg16::HL, &Reg8::L, gb),
        0x76 => halt(cpu),
        0x77 => sd8(cpu, &Reg16::HL, &Reg8::A, gb),
        0x78 => mov8(cpu, &Reg8::A, &Reg8::B),
        0x79 => mov8(cpu, &Reg8::A, &Reg8::C),
        0x7A => mov8(cpu, &Reg8::A, &Reg8::D),
        0x7B => mov8(cpu, &Reg8::A, &Reg8::E),
        0x7C => mov8(cpu, &Reg8::A, &Reg8::H),
        0x7D => mov8(cpu, &Reg8::A, &Reg8::L),
        0x7E => ld8(cpu, &Reg8::A, &Reg16::HL, gb),
        0x7F => mov8(cpu, &Reg8::A, &Reg8::A),

        0x80 => opf28(cpu, &Func2::ADD, &Reg8::A, &Reg8::B),
        0x81 => opf28(cpu, &Func2::ADD, &Reg8::A, &Reg8::C),
        0x82 => opf28(cpu, &Func2::ADD, &Reg8::A, &Reg8::D),
        0x83 => opf28(cpu, &Func2::ADD, &Reg8::A, &Reg8::E),
        0x84 => opf28(cpu, &Func2::ADD, &Reg8::A, &Reg8::H),
        0x85 => opf28(cpu, &Func2::ADD, &Reg8::A, &Reg8::L),
        0x86 => opf2m8(cpu, &Func2::ADD, &Reg8::A, &Reg16::HL, gb),
        0x87 => opf28(cpu, &Func2::ADD, &Reg8::A, &Reg8::A),

        0x88 => opf28(cpu, &Func2::ADC, &Reg8::A, &Reg8::B),
        0x89 => opf28(cpu, &Func2::ADC, &Reg8::A, &Reg8::C),
        0x8A => opf28(cpu, &Func2::ADC, &Reg8::A, &Reg8::D),
        0x8B => opf28(cpu, &Func2::ADC, &Reg8::A, &Reg8::E),
        0x8C => opf28(cpu, &Func2::ADC, &Reg8::A, &Reg8::H),
        0x8D => opf28(cpu, &Func2::ADC, &Reg8::A, &Reg8::L),
        0x8E => opf2m8(cpu, &Func2::ADC, &Reg8::A, &Reg16::HL, gb),
        0x8F => opf28(cpu, &Func2::ADC, &Reg8::A, &Reg8::A),

        0x90 => opf28(cpu, &Func2::SUB, &Reg8::A, &Reg8::B),
        0x91 => opf28(cpu, &Func2::SUB, &Reg8::A, &Reg8::C),
        0x92 => opf28(cpu, &Func2::SUB, &Reg8::A, &Reg8::D),
        0x93 => opf28(cpu, &Func2::SUB, &Reg8::A, &Reg8::E),
        0x94 => opf28(cpu, &Func2::SUB, &Reg8::A, &Reg8::H),
        0x95 => opf28(cpu, &Func2::SUB, &Reg8::A, &Reg8::L),
        0x96 => opf2m8(cpu, &Func2::SUB, &Reg8::A, &Reg16::HL, gb),
        0x97 => opf28(cpu, &Func2::SUB, &Reg8::A, &Reg8::A),

        0x98 => opf28(cpu, &Func2::SBC, &Reg8::A, &Reg8::B),
        0x99 => opf28(cpu, &Func2::SBC, &Reg8::A, &Reg8::C),
        0x9A => opf28(cpu, &Func2::SBC, &Reg8::A, &Reg8::D),
        0x9B => opf28(cpu, &Func2::SBC, &Reg8::A, &Reg8::E),
        0x9C => opf28(cpu, &Func2::SBC, &Reg8::A, &Reg8::H),
        0x9D => opf28(cpu, &Func2::SBC, &Reg8::A, &Reg8::L),
        0x9E => opf2m8(cpu, &Func2::SBC, &Reg8::A, &Reg16::HL, gb),
        0x9F => opf28(cpu, &Func2::SBC, &Reg8::A, &Reg8::A),

        0xA0 => opf28(cpu, &Func2::AND, &Reg8::A, &Reg8::B),
        0xA1 => opf28(cpu, &Func2::AND, &Reg8::A, &Reg8::C),
        0xA2 => opf28(cpu, &Func2::AND, &Reg8::A, &Reg8::D),
        0xA3 => opf28(cpu, &Func2::AND, &Reg8::A, &Reg8::E),
        0xA4 => opf28(cpu, &Func2::AND, &Reg8::A, &Reg8::H),
        0xA5 => opf28(cpu, &Func2::AND, &Reg8::A, &Reg8::L),
        0xA6 => opf2m8(cpu, &Func2::AND, &Reg8::A, &Reg16::HL, gb),
        0xA7 => opf28(cpu, &Func2::AND, &Reg8::A, &Reg8::A),

        0xA8 => opf28(cpu, &Func2::XOR, &Reg8::A, &Reg8::B),
        0xA9 => opf28(cpu, &Func2::XOR, &Reg8::A, &Reg8::C),
        0xAA => opf28(cpu, &Func2::XOR, &Reg8::A, &Reg8::D),
        0xAB => opf28(cpu, &Func2::XOR, &Reg8::A, &Reg8::E),
        0xAC => opf28(cpu, &Func2::XOR, &Reg8::A, &Reg8::H),
        0xAD => opf28(cpu, &Func2::XOR, &Reg8::A, &Reg8::L),
        0xAE => opf2m8(cpu, &Func2::XOR, &Reg8::A, &Reg16::HL, gb),
        0xAF => opf28(cpu, &Func2::XOR, &Reg8::A, &Reg8::A),

        0xB0 => opf28(cpu, &Func2::OR, &Reg8::A, &Reg8::B),
        0xB1 => opf28(cpu, &Func2::OR, &Reg8::A, &Reg8::C),
        0xB2 => opf28(cpu, &Func2::OR, &Reg8::A, &Reg8::D),
        0xB3 => opf28(cpu, &Func2::OR, &Reg8::A, &Reg8::E),
        0xB4 => opf28(cpu, &Func2::OR, &Reg8::A, &Reg8::H),
        0xB5 => opf28(cpu, &Func2::OR, &Reg8::A, &Reg8::L),
        0xB6 => opf2m8(cpu, &Func2::OR, &Reg8::A, &Reg16::HL, gb),
        0xB7 => opf28(cpu, &Func2::OR, &Reg8::A, &Reg8::A),

        0xB8 => opf28(cpu, &Func2::CP, &Reg8::A, &Reg8::B),
        0xB9 => opf28(cpu, &Func2::CP, &Reg8::A, &Reg8::C),
        0xBA => opf28(cpu, &Func2::CP, &Reg8::A, &Reg8::D),
        0xBB => opf28(cpu, &Func2::CP, &Reg8::A, &Reg8::E),
        0xBC => opf28(cpu, &Func2::CP, &Reg8::A, &Reg8::H),
        0xBD => opf28(cpu, &Func2::CP, &Reg8::A, &Reg8::L),
        0xBE => opf2m8(cpu, &Func2::CP, &Reg8::A, &Reg16::HL, gb),
        0xBF => opf28(cpu, &Func2::CP, &Reg8::A, &Reg8::A),

        0xC0 => ret_cond(cpu, !cpu.f_z(), gb),
        0xC1 => pop16(cpu, &Reg16::BC, gb),
        0xC2 => jp_cond(cpu, !cpu.f_z(), gb),
        0xC3 => jp(cpu, gb),
        0xC4 => call_cond(cpu, !cpu.f_z(), gb),
        0xC5 => push16(cpu, &Reg16::BC, gb),
        0xC6 => opf2i8(cpu, &Func2::ADD, &Reg8::A, gb),
        0xC7 => rst(cpu, 0x00, gb),
        0xC8 => ret_cond(cpu, cpu.f_z(), gb),
        0xC9 => ret(cpu, gb),
        0xCA => jp_cond(cpu, cpu.f_z(), gb),
        0xCB => execute_instruction_extension(cpu, gb),
        0xCC => call_cond(cpu, cpu.f_z(), gb),
        0xCD => call(cpu, gb),
        0xCE => opf2i8(cpu, &Func2::ADC, &Reg8::A, gb),
        0xCF => rst(cpu, 0x08, gb),

        0xD0 => ret_cond(cpu, !cpu.f_c(), gb),
        0xD1 => pop16(cpu, &Reg16::DE, gb),
        0xD2 => jp_cond(cpu, !cpu.f_c(), gb),
        0xD3 => panic!("CPU: instruction 0xD3 does not exist!"),
        0xD4 => call_cond(cpu, !cpu.f_c(), gb),
        0xD5 => push16(cpu, &Reg16::DE, gb),
        0xD6 => opf2i8(cpu, &Func2::SUB, &Reg8::A, gb),
        0xD7 => rst(cpu, 0x10, gb),
        0xD8 => ret_cond(cpu, cpu.f_c(), gb),
        0xD9 => reti(cpu, gb),
        0xDA => jp_cond(cpu, cpu.f_c(), gb),
        0xDB => panic!("CPU: instruction 0xDB does not exist!"),
        0xDC => call_cond(cpu, cpu.f_c(), gb),
        0xDD => panic!("CPU: instruction 0xDD does not exist!"),
        0xDE => opf2i8(cpu, &Func2::SBC, &Reg8::A, gb),
        0xDF => rst(cpu, 0x18, gb),

        0xE0 => sdihigh8(cpu, &Reg8::A, gb),
        0xE1 => pop16(cpu, &Reg16::HL, gb),
        0xE2 => sdhigh8(cpu, &Reg8::A, reg_get8(cpu, &Reg8::C), gb),
        0xE3 => panic!("CPU: instruction 0xE3 does not exist!"),
        0xE4 => panic!("CPU: instruction 0xE4 does not exist!"),
        0xE5 => push16(cpu, &Reg16::HL, gb),
        0xE6 => opf2i8(cpu, &Func2::AND, &Reg8::A, gb),
        0xE7 => rst(cpu, 0x20, gb),
        0xE8 => addi16(cpu, &Reg16::SP, gb),
        0xE9 => jp_reg(cpu, &Reg16::HL),
        0xEA => sdiabs8(cpu, &Reg8::A, gb),
        0xEB => panic!("CPU: instruction 0xEB does not exist!"),
        0xEC => panic!("CPU: instruction 0xEC does not exist!"),
        0xED => panic!("CPU: instruction 0xED does not exist!"),
        0xEE => opf2i8(cpu, &Func2::XOR, &Reg8::A, gb),
        0xEF => rst(cpu, 0x28, gb),

        0xF0 => ldihigh8(cpu, &Reg8::A, gb),
        0xF1 => pop16(cpu, &Reg16::AF, gb),
        0xF2 => ldhigh8(cpu, &Reg8::A, reg_get8(cpu, &Reg8::C), gb),
        0xF3 => di(cpu),
        0xF4 => panic!("CPU: instruction 0xF4 does not exist!"),
        0xF5 => push16(cpu, &Reg16::AF, gb),
        0xF6 => opf2i8(cpu, &Func2::OR, &Reg8::A, gb),
        0xF7 => rst(cpu, 0x30, gb),
        0xF8 => movoff16(cpu, &Reg16::HL, &Reg16::SP, gb),
        0xF9 => mov16(cpu, &Reg16::SP, &Reg16::HL),
        0xFA => ldiabs8(cpu, &Reg8::A, gb),
        0xFB => ei(cpu),
        0xFC => panic!("CPU: instruction 0xFC does not exist!"),
        0xFD => panic!("CPU: instruction 0xFD does not exist!"),
        0xFE => opf2i8(cpu, &Func2::CP, &Reg8::A, gb),
        0xFF => rst(cpu, 0x38, gb),
    };
    if instr == 0x07 || instr == 0x0F || instr == 0x17 || instr == 0x1f {
        // Rotate on Akkumulator will reset ZF for whatever reason
        cpu.set_flag(Flag::Z, false);
    }
    new_pc
}

fn get_imm16(cpu: &CPU, gb: &Gameboy) -> u16 {
    let low = gb.read8(cpu.pc.wrapping_add(1)).unwrap();
    let high = gb.read8(cpu.pc.wrapping_add(2)).unwrap();
    ((high as u16) << 8) | low as u16
}

fn ldi16(cpu: &mut CPU, dst: &Reg16, gb: &Gameboy) -> (u16, u16) {
    let imm = get_imm16(cpu, gb);
    reg_set16(cpu, &dst, imm);
    (cpu.pc.wrapping_add(3), 12)
}

fn ldi8(cpu: &mut CPU, dst: &Reg8, gb: &Gameboy) -> (u16, u16) {
    let val = gb.read8(cpu.pc.wrapping_add(1)).unwrap();
    reg_set8(cpu, &dst, val);
    (cpu.pc.wrapping_add(2), 8)
}

fn ldhigh8(cpu: &mut CPU, dst: &Reg8, offset: u8, gb: &Gameboy) -> (u16, u16) {
    let address = 0xFF00 + offset as u16;
    let val = gb.read8(address).unwrap();
    reg_set8(cpu, &dst, val);
    (cpu.pc.wrapping_add(1), 8)
}

fn ldihigh8(cpu: &mut CPU, dst: &Reg8, gb: &Gameboy) -> (u16, u16) {
    let imm = gb.read8(cpu.pc.wrapping_add(1)).unwrap();
    ldhigh8(cpu, &dst, imm, gb);
    (cpu.pc.wrapping_add(2), 12)
}

fn ldiabs8(cpu: &mut CPU, dst: &Reg8, gb: &Gameboy) -> (u16, u16) {
    let imm = get_imm16(cpu, gb);
    let val = gb.read8(imm).unwrap();
    reg_set8(cpu, &dst, val);
    (cpu.pc.wrapping_add(3), 16)
}

fn ld8(cpu: &mut CPU, dst: &Reg8, src: &Reg16, gb: &Gameboy) -> (u16, u16) {
    reg_set8(cpu, dst, gb.read8(reg_get16(cpu, src)).unwrap());
    (cpu.pc.wrapping_add(1), 8)
}

fn ldinc8(cpu: &mut CPU, dst: &Reg8, src: &Reg16, gb: &Gameboy) -> (u16, u16) {
    ld8(cpu, dst, src, gb);
    inc16(cpu, src);
    (cpu.pc.wrapping_add(1), 8)
}

fn lddec8(cpu: &mut CPU, dst: &Reg8, src: &Reg16, gb: &Gameboy) -> (u16, u16) {
    ld8(cpu, dst, src, gb);
    dec16(cpu, src);
    (cpu.pc.wrapping_add(1), 8)
}

fn sd8(cpu: &mut CPU, dst: &Reg16, src: &Reg8, gb: &mut Gameboy) -> (u16, u16) {
    gb.write8(reg_get16(cpu, &dst), reg_get8(cpu, &src));
    (cpu.pc.wrapping_add(1), 8)
}

fn sdi8(cpu: &mut CPU, src: &Reg16, gb: &mut Gameboy) -> (u16, u16) {
    let address = reg_get16(cpu, src);
    let val = gb.read8(cpu.pc.wrapping_add(1)).unwrap();
    gb.write8(address, val);
    (cpu.pc.wrapping_add(2), 12)
}

fn sdhigh8(cpu: &mut CPU, src: &Reg8, offset: u8, gb: &mut Gameboy) -> (u16, u16) {
    let address = 0xFF00 + offset as u16;
    let val = reg_get8(cpu, src);
    gb.write8(address, val);
    (cpu.pc.wrapping_add(1), 8)
}

fn sdihigh8(cpu: &mut CPU, src: &Reg8, gb: &mut Gameboy) -> (u16, u16) {
    let imm = gb.read8(cpu.pc.wrapping_add(1)).unwrap();
    sdhigh8(cpu, &src, imm, gb);
    (cpu.pc.wrapping_add(2), 12)
}

fn sdiabs8(cpu: &mut CPU, src: &Reg8, gb: &mut Gameboy) -> (u16, u16) {
    let imm = get_imm16(cpu, gb);
    let val = reg_get8(cpu, &src);
    gb.write8(imm, val);
    (cpu.pc.wrapping_add(3), 16)
}

fn sdinc8(cpu: &mut CPU, dst: &Reg16, src: &Reg8, gb: &mut Gameboy) -> (u16, u16) {
    sd8(cpu, &dst, &src, gb);
    inc16(cpu, &dst);
    (cpu.pc.wrapping_add(1), 8)
}

fn sddec8(cpu: &mut CPU, dst: &Reg16, src: &Reg8, gb: &mut Gameboy) -> (u16, u16) {
    sd8(cpu, &dst, &src, gb);
    dec16(cpu, &dst);
    (cpu.pc.wrapping_add(1), 8)
}

fn inc16(cpu: &mut CPU, reg: &Reg16) -> (u16, u16) {
    reg_inc16(cpu, &reg);
    (cpu.pc.wrapping_add(1), 8)
}

fn dec16(cpu: &mut CPU, reg: &Reg16) -> (u16, u16) {
    reg_dec16(cpu, &reg);
    (cpu.pc.wrapping_add(1), 8)
}

fn saveSP(cpu: &mut CPU, gb: &mut Gameboy) -> (u16, u16) {
    let imm = get_imm16(cpu, gb);
    let sp = reg_get16(cpu, &Reg16::SP);
    let low = (sp & 0xFF) as u8;
    let high = (sp >> 8) as u8;
    gb.write8(imm, low);
    gb.write8(imm.wrapping_add(1), high); // SPECI: not sure if wrapping here is ok
    (cpu.pc.wrapping_add(3), 20)
}

fn mov8(cpu: &mut CPU, dst: &Reg8, src: &Reg8) -> (u16, u16) {
    reg_set8(cpu, dst, reg_get8(cpu, src));
    (cpu.pc.wrapping_add(1), 4)
}

fn mov16(cpu: &mut CPU, dst: &Reg16, src: &Reg16) -> (u16, u16) {
    let sv = reg_get16(cpu, &src);
    reg_set16(cpu, &dst, sv);
    (cpu.pc.wrapping_add(1), 16)
}

fn movoff16(cpu: &mut CPU, dst: &Reg16, src: &Reg16, gb: &Gameboy) -> (u16, u16) {
    let imm = gb.read8(cpu.pc.wrapping_add(1)).unwrap() as i8;
    let sv = reg_get16(cpu, &src);
    let val = _addi16(cpu, sv, imm);
    reg_set16(cpu, &dst, val);
    (cpu.pc.wrapping_add(2), 12)
}

/* ALU OPERATIONS */

fn _inc8(cpu: &mut CPU, val: u8) -> u8 {
    let res = val.wrapping_add(1);
    cpu.set_flag(Flag::Z, res == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, (val & 0xF) == 0xF);
    res
}

fn _dec8(cpu: &mut CPU, val: u8) -> u8 {
    let res = val.wrapping_sub(1);
    cpu.set_flag(Flag::Z, res == 0);
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, (val & 0xF) == 0);
    res
}

fn _rlc(cpu: &mut CPU, val: u8) -> u8 {
    let b7 = val >> 7;
    let ret = (val << 1) | b7;
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, b7 == 0x01);
    ret
}

fn _rl(cpu: &mut CPU, val: u8, through_carry: bool) -> u8 {
    let b7 = val >> 7;
    let ret = (val << 1)
        | match through_carry {
            true => cpu.f_c() as u8,
            false => b7,
        };
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, b7 == 0x01);
    ret
}

fn _rr(cpu: &mut CPU, val: u8, through_carry: bool) -> u8 {
    let b0 = val & 0x01;
    let ret = (val >> 1)
        | ((match through_carry {
            true => cpu.f_c() as u8,
            false => b0,
        }) << 7);
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, b0 == 0x01);
    ret
}

fn _sla(cpu: &mut CPU, val: u8) -> u8 {
    let b7 = val >> 7;
    let ret = val << 1;
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, b7 == 0x01);
    ret
}

fn _sra(cpu: &mut CPU, val: u8) -> u8 {
    let b0 = val & 0x01;
    let ret = ((val as i8) >> 1) as u8;
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, b0 == 0x01);
    ret
}

fn _srl(cpu: &mut CPU, val: u8) -> u8 {
    let b0 = val & 0x01;
    let ret = val >> 1;
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, b0 == 0x01);
    ret
}

fn _swap(cpu: &mut CPU, val: u8) -> u8 {
    let high = val >> 4;
    let low = val & 0x0F;
    let ret = (low << 4) | high;
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, false);
    ret
}

fn _bit(cpu: &mut CPU, val: u8, bit: u8) {
    let b = (val >> bit) & 0x01;
    cpu.set_flag(Flag::Z, b != 0x01);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, true);
}

fn _res(cpu: &CPU, val: u8, bit: u8) -> u8 {
    val & !(1 << bit)
}

fn _set(cpu: &CPU, val: u8, bit: u8) -> u8 {
    val | (1 << bit)
}

fn exec_opf18(cpu: &mut CPU, f1: &Func1, val: u8) -> u8 {
    match f1 {
        Func1::INC => _inc8(cpu, val),
        Func1::DEC => _dec8(cpu, val),
        Func1::RLC => _rl(cpu, val, false),
        Func1::RRC => _rr(cpu, val, false),
        Func1::RL => _rl(cpu, val, true),
        Func1::RR => _rr(cpu, val, true),
        Func1::SLA => _sla(cpu, val),
        Func1::SRA => _sra(cpu, val),
        Func1::SRL => _srl(cpu, val),
        Func1::SWAP => _swap(cpu, val),
    }
}

fn opf18(cpu: &mut CPU, f1: &Func1, reg: &Reg8) -> (u16, u16) {
    let val = exec_opf18(cpu, &f1, reg_get8(cpu, &reg));
    reg_set8(cpu, &reg, val);
    (cpu.pc.wrapping_add(1), 4)
}

fn opf1m8(cpu: &mut CPU, f1: &Func1, reg: &Reg16, gb: &mut Gameboy) -> (u16, u16) {
    let address = reg_get16(cpu, &reg);
    let val = exec_opf18(cpu, &f1, gb.read8(address).unwrap());
    gb.write8(address, val);
    (cpu.pc.wrapping_add(1), 12)
}

fn _add8(cpu: &mut CPU, dst: u8, src: u8) -> u8 {
    let res = dst.wrapping_add(src);
    cpu.set_flag(Flag::Z, res == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, ((src & 0xF) + (dst & 0xF)) & 0x10 == 0x10);
    cpu.set_flag(Flag::C, res < dst);
    res
}

fn _sub8(cpu: &mut CPU, dst: u8, src: u8) -> u8 {
    let res = dst.wrapping_sub(src);
    cpu.set_flag(Flag::Z, res == 0);
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, (src & 0xF) > (dst & 0xF));
    cpu.set_flag(Flag::C, src > dst);
    res
}

fn _and8(cpu: &mut CPU, dst: u8, src: u8) -> u8 {
    let res = dst & src;
    cpu.set_flag(Flag::Z, res == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, true);
    cpu.set_flag(Flag::C, false);
    res
}

fn _xor8(cpu: &mut CPU, dst: u8, src: u8) -> u8 {
    let res = dst ^ src;
    cpu.set_flag(Flag::Z, res == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, false);
    res
}

fn _or8(cpu: &mut CPU, dst: u8, src: u8) -> u8 {
    let res = dst | src;
    cpu.set_flag(Flag::Z, res == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, false);
    res
}

fn _adc_sbc_getsrc(cpu: &CPU, src: u8) -> (u8, bool) {
    let cy = cpu.f_c() as u8;
    let mut new_cy = false;
    let mut new_hf = false;
    // if cy == 1 && src == 0xF {
    //     new_hf = true;
    // }
    if cy == 1 && src == 0xFF {
        new_cy = true;
    }
    (src.wrapping_add(cy), new_cy)
}

fn _adc8(cpu: &mut CPU, dst: u8, src: u8) -> u8 {
    let cy = cpu.f_c() as u8;
    let sum: u16 = (dst as u16) + (src as u16) + (cy as u16);
    let xor: u8 = dst ^ src ^ cy;
    let info = sum ^ (xor as u16);
    let ret = (sum & 0xFF) as u8;
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, info & 0x10 == 0x10);
    cpu.set_flag(Flag::C, info & 0x100 == 0x100);
    ret
}

fn _sbc8(cpu: &mut CPU, dst: u8, src: u8) -> u8 {
    let cy = cpu.f_c() as u8;
    let sum: u16 = (dst as u16).wrapping_sub(src as u16).wrapping_sub(cy as u16);
    let xor: u8 = dst ^ src ^ cy;
    let info = sum ^ (xor as u16);
    let ret = (sum & 0xFF) as u8;
    cpu.set_flag(Flag::Z, ret == 0);
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, info & 0x10 == 0x10);
    cpu.set_flag(Flag::C, info & 0x100 == 0x100);
    ret
}

fn exec_opf28(cpu: &mut CPU, f2: &Func2, dst: u8, src: u8) -> u8 {
    match f2 {
        Func2::ADD => _add8(cpu, dst, src),
        Func2::ADC => _adc8(cpu, dst, src),
        Func2::SUB => _sub8(cpu, dst, src),
        Func2::SBC => _sbc8(cpu, dst, src),
        Func2::AND => _and8(cpu, dst, src),
        Func2::XOR => _xor8(cpu, dst, src),
        Func2::OR => _or8(cpu, dst, src),
        Func2::CP => _sub8(cpu, dst, src), /* CP is just SUB without writeback */
        Func2::BIT => {
            _bit(cpu, dst, src);
            0
        }
        Func2::SET => _set(cpu, dst, src),
        Func2::RES => _res(cpu, dst, src),
    }
}

fn opf2h8(cpu: &mut CPU, f2: &Func2, dst: &Reg8, src: u8) -> (u16, u16) {
    let val = exec_opf28(cpu, &f2, reg_get8(cpu, &dst), src);
    let writeback = !matches!(f2, Func2::CP) && !matches!(f2, Func2::BIT);
    if writeback {
        reg_set8(cpu, &dst, val);
    }
    (cpu.pc.wrapping_add(1), 8)
}

fn opf28(cpu: &mut CPU, f2: &Func2, dst: &Reg8, src: &Reg8) -> (u16, u16) {
    opf2h8(cpu, &f2, &dst, reg_get8(cpu, &src));
    (cpu.pc.wrapping_add(1), 4)
}

fn opf2m8(cpu: &mut CPU, f2: &Func2, dst: &Reg8, src: &Reg16, gb: &Gameboy) -> (u16, u16) {
    let val = exec_opf28(cpu, &f2, reg_get8(cpu, &dst), gb.read8(reg_get16(cpu, &src)).unwrap());
    let writeback = !matches!(f2, Func2::CP) && !matches!(f2, Func2::BIT);
    if writeback {
        reg_set8(cpu, &dst, val);
    }
    (cpu.pc.wrapping_add(1), 8)
}

fn opf2mh8(cpu: &mut CPU, f2: &Func2, dst: &Reg16, src: u8, gb: &mut Gameboy) -> (u16, u16) {
    let address = reg_get16(cpu, &dst);
    let val = exec_opf28(cpu, &f2, gb.read8(address).unwrap(), src);
    let writeback = !matches!(f2, Func2::CP) && !matches!(f2, Func2::BIT);
    if writeback {
        gb.write8(address, val);
    }
    (cpu.pc.wrapping_add(1), 16)
}

fn opf2i8(cpu: &mut CPU, f2: &Func2, dst: &Reg8, gb: &Gameboy) -> (u16, u16) {
    let imm = gb.read8(cpu.pc.wrapping_add(1)).unwrap();

    let val = exec_opf28(cpu, &f2, reg_get8(cpu, &dst), imm);
    let writeback = !matches!(f2, Func2::CP) && !matches!(f2, Func2::BIT);
    if writeback {
        reg_set8(cpu, &dst, val);
    }

    (cpu.pc.wrapping_add(2), 8)
}

fn _pop(cpu: &mut CPU, gb: &Gameboy) -> u16 {
    let sp = reg_get16(cpu, &Reg16::SP);
    let low = gb.read8(sp).unwrap();
    let high = gb.read8(sp.wrapping_add(1)).unwrap();
    let val = ((high as u16) << 8) | low as u16;
    reg_set16(cpu, &Reg16::SP, sp.wrapping_add(2));
    val
}

fn _push(cpu: &mut CPU, val: u16, gb: &mut Gameboy) {
    let sp = reg_get16(cpu, &Reg16::SP);
    let sp = sp.wrapping_sub(2);
    reg_set16(cpu, &Reg16::SP, sp);
    let low = (val & 0xFF) as u8;
    let high = (val >> 8) as u8;
    gb.write8(sp, low);
    gb.write8(sp.wrapping_add(1), high);
}

fn pop16(cpu: &mut CPU, dst: &Reg16, gb: &Gameboy) -> (u16, u16) {
    let val = _pop(cpu, gb);
    reg_set16(cpu, &dst, val);
    (cpu.pc.wrapping_add(1), 12)
}

fn push16(cpu: &mut CPU, src: &Reg16, gb: &mut Gameboy) -> (u16, u16) {
    let val = reg_get16(cpu, &src);
    _push(cpu, val, gb);
    (cpu.pc.wrapping_add(1), 16)
}

fn add16(cpu: &mut CPU, dst: &Reg16, src: &Reg16) -> (u16, u16) {
    let (dh, dl) = split_Reg16(dst);
    let (sh, sl) = split_Reg16(src);
    let zf = cpu.f_z();
    opf28(cpu, &Func2::ADD, &dl, &sl);
    opf28(cpu, &Func2::ADC, &dh, &sh);
    cpu.set_flag(Flag::Z, zf);
    (cpu.pc.wrapping_add(1), 8)
}

fn _addi16(cpu: &mut CPU, dst: u16, src: i8) -> u16 {
    let src = (src as i16) as u16;
    let mut dl = (dst & 0xFF) as u8;
    let mut dh = (dst >> 8) as u8;
    let sl = (src & 0xFF) as u8;
    let sh = (src >> 8) as u8;
    dl = _add8(cpu, dl, sl);
    let flags = cpu.f; // copy flags
    dh = _adc8(cpu, dh, sh);
    cpu.f = flags & 0xF0;
    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);

    ((dh as u16) << 8) | dl as u16
}

fn addi16(cpu: &mut CPU, dst: &Reg16, gb: &Gameboy) -> (u16, u16) {
    let imm = gb.read8(cpu.pc.wrapping_add(1)).unwrap() as i8;
    let dv = reg_get16(cpu, &dst);
    let res = _addi16(cpu, dv, imm);
    reg_set16(cpu, &dst, res);
    (cpu.pc.wrapping_add(2), 16)
}

fn reti(cpu: &mut CPU, gb: &Gameboy) -> (u16, u16) {
    cpu.interrupt_master_enable = true;
    ret(cpu, gb)
}

fn ret(cpu: &mut CPU, gb: &Gameboy) -> (u16, u16) {
    let ret = _pop(cpu, gb);
    (ret, 16)
}

fn ret_cond(cpu: &mut CPU, cond: bool, gb: &Gameboy) -> (u16, u16) {
    if cond {
        let (ret, _) = ret(cpu, gb);
        return (ret, 20);
    }
    (cpu.pc.wrapping_add(1), 8)
}

fn jp(cpu: &mut CPU, gb: &Gameboy) -> (u16, u16) {
    (get_imm16(cpu, gb), 16)
}

fn jp_cond(cpu: &mut CPU, cond: bool, gb: &Gameboy) -> (u16, u16) {
    if cond {
        return jp(cpu, gb);
    }
    (cpu.pc.wrapping_add(3), 12)
}

fn jp_reg(cpu: &mut CPU, reg: &Reg16) -> (u16, u16) {
    (reg_get16(cpu, &reg), 1)
}

fn jr(cpu: &mut CPU, gb: &Gameboy) -> (u16, u16) {
    let imm = gb.read8(cpu.pc.wrapping_add(1)).unwrap();
    (cpu.pc.wrapping_add(((imm as i8) as i16) as u16).wrapping_add(2), 12)
}

fn jr_cond(cpu: &mut CPU, cond: bool, gb: &Gameboy) -> (u16, u16) {
    if cond {
        return jr(cpu, gb);
    }
    (cpu.pc.wrapping_add(2), 8)
}

fn call(cpu: &mut CPU, gb: &mut Gameboy) -> (u16, u16) {
    _push(cpu, cpu.pc.wrapping_add(3), gb);
    (get_imm16(cpu, gb), 24)
}

fn call_cond(cpu: &mut CPU, cond: bool, gb: &mut Gameboy) -> (u16, u16) {
    if cond {
        return call(cpu, gb);
    }
    (cpu.pc.wrapping_add(3), 12)
}

fn ei(cpu: &mut CPU) -> (u16, u16) {
    cpu.interrupt_master_enable = true;
    (cpu.pc.wrapping_add(1), 4)
}

fn di(cpu: &mut CPU) -> (u16, u16) {
    cpu.interrupt_master_enable = false;
    (cpu.pc.wrapping_add(1), 4)
}

fn rst(cpu: &mut CPU, target: u8, gb: &mut Gameboy) -> (u16, u16) {
    _push(cpu, cpu.pc.wrapping_add(1), gb);
    (target as u16, 16)
}

fn halt(cpu: &mut CPU) -> (u16, u16) {
    cpu.halted = true;
    (cpu.pc.wrapping_add(1), 4)
}

fn cpl_Akku(cpu: &mut CPU) -> (u16, u16) {
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, true);
    reg_set8(cpu, &Reg8::A, !reg_get8(cpu, &Reg8::A));
    (cpu.pc.wrapping_add(1), 4)
}

fn ccf(cpu: &mut CPU) -> (u16, u16) {
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, !cpu.f_c());
    (cpu.pc.wrapping_add(1), 4)
}

fn scf(cpu: &mut CPU) -> (u16, u16) {
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, true);
    (cpu.pc.wrapping_add(1), 4)
}

fn daa(cpu: &mut CPU) -> (u16, u16) {
    let cy = cpu.f_c();
    let n = cpu.f_n();
    let h = cpu.f_h();
    let mut a = reg_get8(cpu, &Reg8::A);
    // note: assumes a is a uint8_t and wraps from 0xff to 0
    if !n {
        // after an addition, adjust if (half-)carry occurred or if result is out of bounds
        if cy || a > 0x99 {
            a = a.wrapping_add(0x60);
            cpu.set_flag(Flag::C, true);
        }
        if h || (a & 0x0f) > 0x09 {
            a = a.wrapping_add(0x6);
        }
    } else {
        // after a subtraction, only adjust if (half-)carry occurred
        if cy {
            a = a.wrapping_sub(0x60);
        }
        if h {
            a = a.wrapping_sub(0x6);
        }
    }
    // these flags are always updated
    cpu.set_flag(Flag::Z, a == 0); // the usual z flag
    cpu.set_flag(Flag::H, false); // h flag is always cleared
    reg_set8(cpu, &Reg8::A, a);
    (cpu.pc.wrapping_add(1), 4)
}

const CPU_COMMANDS: [&str; 0x200] = [
    "NOP",
    "LD BC,d16",
    "LD (BC),A",
    "INC BC",
    "INC B",
    "DEC B",
    "LD B,d8",
    "RLCA",
    "LD (a16),SP",
    "ADD HL,BC",
    "LD A,(BC)",
    "DEC BC",
    "INC C",
    "DEC C",
    "LD C,d8",
    "RRCA",
    "STOP 0",
    "LD DE,d16",
    "LD (DE),A",
    "INC DE",
    "INC D",
    "DEC D",
    "LD D,d8",
    "RLA",
    "JR r8",
    "ADD HL,DE",
    "LD A,(DE)",
    "DEC DE",
    "INC E",
    "DEC E",
    "LD E,d8",
    "RRA",
    "JR NZ,r8",
    "LD HL,d16",
    "LD (HL+),A",
    "INC HL",
    "INC H",
    "DEC H",
    "LD H,d8",
    "DAA",
    "JR Z,r8",
    "ADD HL,HL",
    "LD A,(HL+)",
    "DEC HL",
    "INC L",
    "DEC L",
    "LD L,d8",
    "CPL",
    "JR NC,r8",
    "LD SP,d16",
    "LD (HL-),A",
    "INC SP",
    "INC (HL)",
    "DEC (HL)",
    "LD (HL),d8",
    "SCF",
    "JR C,r8",
    "ADD HL,SP",
    "LD A,(HL-)",
    "DEC SP",
    "INC A",
    "DEC A",
    "LD A,d8",
    "CCF",
    "LD B,B",
    "LD B,C",
    "LD B,D",
    "LD B,E",
    "LD B,H",
    "LD B,L",
    "LD B,(HL)",
    "LD B,A",
    "LD C,B",
    "LD C,C",
    "LD C,D",
    "LD C,E",
    "LD C,H",
    "LD C,L",
    "LD C,(HL)",
    "LD C,A",
    "LD D,B",
    "LD D,C",
    "LD D,D",
    "LD D,E",
    "LD D,H",
    "LD D,L",
    "LD D,(HL)",
    "LD D,A",
    "LD E,B",
    "LD E,C",
    "LD E,D",
    "LD E,E",
    "LD E,H",
    "LD E,L",
    "LD E,(HL)",
    "LD E,A",
    "LD H,B",
    "LD H,C",
    "LD H,D",
    "LD H,E",
    "LD H,H",
    "LD H,L",
    "LD H,(HL)",
    "LD H,A",
    "LD L,B",
    "LD L,C",
    "LD L,D",
    "LD L,E",
    "LD L,H",
    "LD L,L",
    "LD L,(HL)",
    "LD L,A",
    "LD (HL),B",
    "LD (HL),C",
    "LD (HL),D",
    "LD (HL),E",
    "LD (HL),H",
    "LD (HL),L",
    "HALT",
    "LD (HL),A",
    "LD A,B",
    "LD A,C",
    "LD A,D",
    "LD A,E",
    "LD A,H",
    "LD A,L",
    "LD A,(HL)",
    "LD A,A",
    "ADD A,B",
    "ADD A,C",
    "ADD A,D",
    "ADD A,E",
    "ADD A,H",
    "ADD A,L",
    "ADD A,(HL)",
    "ADD A,A",
    "ADC A,B",
    "ADC A,C",
    "ADC A,D",
    "ADC A,E",
    "ADC A,H",
    "ADC A,L",
    "ADC A,(HL)",
    "ADC A,A",
    "SUB B",
    "SUB C",
    "SUB D",
    "SUB E",
    "SUB H",
    "SUB L",
    "SUB (HL)",
    "SUB A",
    "SBC A,B",
    "SBC A,C",
    "SBC A,D",
    "SBC A,E",
    "SBC A,H",
    "SBC A,L",
    "SBC A,(HL)",
    "SBC A,A",
    "AND B",
    "AND C",
    "AND D",
    "AND E",
    "AND H",
    "AND L",
    "AND (HL)",
    "AND A",
    "XOR B",
    "XOR C",
    "XOR D",
    "XOR E",
    "XOR H",
    "XOR L",
    "XOR (HL)",
    "XOR A",
    "OR B",
    "OR C",
    "OR D",
    "OR E",
    "OR H",
    "OR L",
    "OR (HL)",
    "OR A",
    "CP B",
    "CP C",
    "CP D",
    "CP E",
    "CP H",
    "CP L",
    "CP (HL)",
    "CP A",
    "RET NZ",
    "POP BC",
    "JP NZ,a16",
    "JP a16",
    "CALL NZ,a16",
    "PUSH BC",
    "ADD A,d8",
    "RST 00H",
    "RET Z",
    "RET",
    "JP Z,a16",
    "PREFIX CB",
    "CALL Z,a16",
    "CALL a16",
    "ADC A,d8",
    "RST 08H",
    "RET NC",
    "POP DE",
    "JP NC,a16",
    "",
    "CALL NC,a16",
    "PUSH DE",
    "SUB d8",
    "RST 10H",
    "RET C",
    "RETI",
    "JP C,a16",
    "",
    "CALL C,a16",
    "",
    "SBC A,d8",
    "RST 18H",
    "LDH (a8),A",
    "POP HL",
    "LD (C),A",
    "",
    "",
    "PUSH HL",
    "AND d8",
    "RST 20H",
    "ADD SP,r8",
    "JP (HL)",
    "LD (a16),A",
    "",
    "",
    "",
    "XOR d8",
    "RST 28H",
    "LDH A,(a8)",
    "POP AF",
    "LD A,(C)",
    "DI",
    "",
    "PUSH AF",
    "OR d8",
    "RST 30H",
    "LD HL,SP+r8",
    "LD SP,HL",
    "LD A,(a16)",
    "EI",
    "",
    "",
    "CP d8",
    "RST 38H",
    "RLC B",
    "RLC C",
    "RLC D",
    "RLC E",
    "RLC H",
    "RLC L",
    "RLC (HL)",
    "RLC A",
    "RRC B",
    "RRC C",
    "RRC D",
    "RRC E",
    "RRC H",
    "RRC L",
    "RRC (HL)",
    "RRC A",
    "RL B",
    "RL C",
    "RL D",
    "RL E",
    "RL H",
    "RL L",
    "RL (HL)",
    "RL A",
    "RR B",
    "RR C",
    "RR D",
    "RR E",
    "RR H",
    "RR L",
    "RR (HL)",
    "RR A",
    "SLA B",
    "SLA C",
    "SLA D",
    "SLA E",
    "SLA H",
    "SLA L",
    "SLA (HL)",
    "SLA A",
    "SRA B",
    "SRA C",
    "SRA D",
    "SRA E",
    "SRA H",
    "SRA L",
    "SRA (HL)",
    "SRA A",
    "SWAP B",
    "SWAP C",
    "SWAP D",
    "SWAP E",
    "SWAP H",
    "SWAP L",
    "SWAP (HL)",
    "SWAP A",
    "SRL B",
    "SRL C",
    "SRL D",
    "SRL E",
    "SRL H",
    "SRL L",
    "SRL (HL)",
    "SRL A",
    "BIT 0,B",
    "BIT 0,C",
    "BIT 0,D",
    "BIT 0,E",
    "BIT 0,H",
    "BIT 0,L",
    "BIT 0,(HL)",
    "BIT 0,A",
    "BIT 1,B",
    "BIT 1,C",
    "BIT 1,D",
    "BIT 1,E",
    "BIT 1,H",
    "BIT 1,L",
    "BIT 1,(HL)",
    "BIT 1,A",
    "BIT 2,B",
    "BIT 2,C",
    "BIT 2,D",
    "BIT 2,E",
    "BIT 2,H",
    "BIT 2,L",
    "BIT 2,(HL)",
    "BIT 2,A",
    "BIT 3,B",
    "BIT 3,C",
    "BIT 3,D",
    "BIT 3,E",
    "BIT 3,H",
    "BIT 3,L",
    "BIT 3,(HL)",
    "BIT 3,A",
    "BIT 4,B",
    "BIT 4,C",
    "BIT 4,D",
    "BIT 4,E",
    "BIT 4,H",
    "BIT 4,L",
    "BIT 4,(HL)",
    "BIT 4,A",
    "BIT 5,B",
    "BIT 5,C",
    "BIT 5,D",
    "BIT 5,E",
    "BIT 5,H",
    "BIT 5,L",
    "BIT 5,(HL)",
    "BIT 5,A",
    "BIT 6,B",
    "BIT 6,C",
    "BIT 6,D",
    "BIT 6,E",
    "BIT 6,H",
    "BIT 6,L",
    "BIT 6,(HL)",
    "BIT 6,A",
    "BIT 7,B",
    "BIT 7,C",
    "BIT 7,D",
    "BIT 7,E",
    "BIT 7,H",
    "BIT 7,L",
    "BIT 7,(HL)",
    "BIT 7,A",
    "RES 0,B",
    "RES 0,C",
    "RES 0,D",
    "RES 0,E",
    "RES 0,H",
    "RES 0,L",
    "RES 0,(HL)",
    "RES 0,A",
    "RES 1,B",
    "RES 1,C",
    "RES 1,D",
    "RES 1,E",
    "RES 1,H",
    "RES 1,L",
    "RES 1,(HL)",
    "RES 1,A",
    "RES 2,B",
    "RES 2,C",
    "RES 2,D",
    "RES 2,E",
    "RES 2,H",
    "RES 2,L",
    "RES 2,(HL)",
    "RES 2,A",
    "RES 3,B",
    "RES 3,C",
    "RES 3,D",
    "RES 3,E",
    "RES 3,H",
    "RES 3,L",
    "RES 3,(HL)",
    "RES 3,A",
    "RES 4,B",
    "RES 4,C",
    "RES 4,D",
    "RES 4,E",
    "RES 4,H",
    "RES 4,L",
    "RES 4,(HL)",
    "RES 4,A",
    "RES 5,B",
    "RES 5,C",
    "RES 5,D",
    "RES 5,E",
    "RES 5,H",
    "RES 5,L",
    "RES 5,(HL)",
    "RES 5,A",
    "RES 6,B",
    "RES 6,C",
    "RES 6,D",
    "RES 6,E",
    "RES 6,H",
    "RES 6,L",
    "RES 6,(HL)",
    "RES 6,A",
    "RES 7,B",
    "RES 7,C",
    "RES 7,D",
    "RES 7,E",
    "RES 7,H",
    "RES 7,L",
    "RES 7,(HL)",
    "RES 7,A",
    "SET 0,B",
    "SET 0,C",
    "SET 0,D",
    "SET 0,E",
    "SET 0,H",
    "SET 0,L",
    "SET 0,(HL)",
    "SET 0,A",
    "SET 1,B",
    "SET 1,C",
    "SET 1,D",
    "SET 1,E",
    "SET 1,H",
    "SET 1,L",
    "SET 1,(HL)",
    "SET 1,A",
    "SET 2,B",
    "SET 2,C",
    "SET 2,D",
    "SET 2,E",
    "SET 2,H",
    "SET 2,L",
    "SET 2,(HL)",
    "SET 2,A",
    "SET 3,B",
    "SET 3,C",
    "SET 3,D",
    "SET 3,E",
    "SET 3,H",
    "SET 3,L",
    "SET 3,(HL)",
    "SET 3,A",
    "SET 4,B",
    "SET 4,C",
    "SET 4,D",
    "SET 4,E",
    "SET 4,H",
    "SET 4,L",
    "SET 4,(HL)",
    "SET 4,A",
    "SET 5,B",
    "SET 5,C",
    "SET 5,D",
    "SET 5,E",
    "SET 5,H",
    "SET 5,L",
    "SET 5,(HL)",
    "SET 5,A",
    "SET 6,B",
    "SET 6,C",
    "SET 6,D",
    "SET 6,E",
    "SET 6,H",
    "SET 6,L",
    "SET 6,(HL)",
    "SET 6,A",
    "SET 7,B",
    "SET 7,C",
    "SET 7,D",
    "SET 7,E",
    "SET 7,H",
    "SET 7,L",
    "SET 7,(HL)",
    "SET 7,A",
];
