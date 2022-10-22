use super::{Register8, Register16, CPU};
use crate::gameboy::memory::Memory;
use crate::gameboy::MemoryInterface;

const FLAGC: u8 = 4;
const FLAGH: u8 = 5;
const FLAGN: u8 = 6;
const FLAGZ: u8 = 7;

fn NOP_00(cpu: &CPU, memory: &Memory) -> u32 { // 00 NOP
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_01(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 01 LD BC,d16
    cpu.set_bc(v);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_02(cpu: &CPU, memory: &Memory) -> u32 { // 02 LD (BC),A
    memory.write8(((cpu.b << 8) + cpu.c), cpu.a);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_03(cpu: &CPU, memory: &Memory) -> u32 { // 03 INC BC
    let mut t: u16 = ((cpu.b << 8) + cpu.c) + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_bc(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_04(cpu: &CPU, memory: &Memory) -> u32 { // 04 INC B
    let mut t: u16 = cpu.b + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.b & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_05(cpu: &CPU, memory: &Memory) -> u32 { // 05 DEC B
    let mut t: u16 = cpu.b - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.b & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_06(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 06 LD B,d8
    cpu.b = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLCA_07(cpu: &CPU, memory: &Memory) -> u32 { // 07 RLCA
    let mut t: u16 = (cpu.a << 1) + (cpu.a >> 7);
    let mut flag: u16 = 0b00000000;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_08(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 08 LD (a16),SP
    memory.write8(v, cpu.sp & 0xFF);
    memory.write8(v+1, cpu.sp >> 8);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 20;
}

fn ADD_09(cpu: &CPU, memory: &Memory) -> u32 { // 09 ADD HL,BC
    let mut t: u16 = cpu.get_hl() + ((cpu.b << 8) + cpu.c);
    let mut flag: u16 = 0b00000000;
    flag += (((cpu.get_hl() & 0xFFF) + (((cpu.b << 8) + cpu.c) & 0xFFF)) > 0xFFF) << FLAGH;
    flag += (t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl() = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_0A(cpu: &CPU, memory: &Memory) -> u32 { // 0A LD A,(BC)
    cpu.a = memory.read8(((cpu.b << 8) + cpu.c));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_0B(cpu: &CPU, memory: &Memory) -> u32 { // 0B DEC BC
    let mut t: u16 = ((cpu.b << 8) + cpu.c) - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_bc(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_0C(cpu: &CPU, memory: &Memory) -> u32 { // 0C INC C
    let mut t: u16 = cpu.c + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.c & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_0D(cpu: &CPU, memory: &Memory) -> u32 { // 0D DEC C
    let mut t: u16 = cpu.c - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.c & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_0E(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 0E LD C,d8
    cpu.c = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRCA_0F(cpu: &CPU, memory: &Memory) -> u32 { // 0F RRCA
    let mut t: u16 = (cpu.a >> 1) + ((cpu.a & 1) << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn STOP_10(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 10 STOP 0
    if cpu.mb.cgb:;
        cpu.mb.switch_speed();
        memory.write8(0xFF04, 0);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_11(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 11 LD DE,d16
    cpu.set_de(v);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_12(cpu: &CPU, memory: &Memory) -> u32 { // 12 LD (DE),A
    memory.write8(((cpu.d << 8) + cpu.e), cpu.a);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_13(cpu: &CPU, memory: &Memory) -> u32 { // 13 INC DE
    let mut t: u16 = ((cpu.d << 8) + cpu.e) + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_de(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_14(cpu: &CPU, memory: &Memory) -> u32 { // 14 INC D
    let mut t: u16 = cpu.d + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.d & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_15(cpu: &CPU, memory: &Memory) -> u32 { // 15 DEC D
    let mut t: u16 = cpu.d - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.d & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_16(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 16 LD D,d8
    cpu.d = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLA_17(cpu: &CPU, memory: &Memory) -> u32 { // 17 RLA
    let mut t: u16 = (cpu.a << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_18(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 18 JR r8
    cpu.pc += 2 + ((v ^ 0x80) - 0x80);
    cpu.pc &= 0xFFFF;
    return 12;
}

fn ADD_19(cpu: &CPU, memory: &Memory) -> u32 { // 19 ADD HL,DE
    let mut t: u16 = cpu.get_hl() + ((cpu.d << 8) + cpu.e);
    let mut flag: u16 = 0b00000000;
    flag += (((cpu.get_hl() & 0xFFF) + (((cpu.d << 8) + cpu.e) & 0xFFF)) > 0xFFF) << FLAGH;
    flag += (t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl() = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_1A(cpu: &CPU, memory: &Memory) -> u32 { // 1A LD A,(DE)
    cpu.a = memory.read8(((cpu.d << 8) + cpu.e));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_1B(cpu: &CPU, memory: &Memory) -> u32 { // 1B DEC DE
    let mut t: u16 = ((cpu.d << 8) + cpu.e) - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_de(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_1C(cpu: &CPU, memory: &Memory) -> u32 { // 1C INC E
    let mut t: u16 = cpu.e + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.e & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_1D(cpu: &CPU, memory: &Memory) -> u32 { // 1D DEC E
    let mut t: u16 = cpu.e - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.e & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_1E(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 1E LD E,d8
    cpu.e = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRA_1F(cpu: &CPU, memory: &Memory) -> u32 { // 1F RRA
    let mut t: u16 = (cpu.a >> 1) + (cpu.f_c() << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_20(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 20 JR NZ,r8
    cpu.pc += 2;
    if cpu.f_nz() -> u32 {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn LD_21(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 21 LD HL,d16
    cpu.set_hl() = v;
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_22(cpu: &CPU, memory: &Memory) -> u32 { // 22 LD (HL+),A
    memory.write8(cpu.get_hl(), cpu.a);
    cpu.set_hl() = cpu.get_hl() +1;
    cpu.set_hl() = cpu.get_hl() & 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_23(cpu: &CPU, memory: &Memory) -> u32 { // 23 INC HL
    let mut t: u16 = cpu.get_hl() + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_hl() = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_24(cpu: &CPU, memory: &Memory) -> u32 { // 24 INC H
    let mut t: u16 = (cpu.get_hl() >> 8) + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += ((((cpu.get_hl() >> 8) & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_25(cpu: &CPU, memory: &Memory) -> u32 { // 25 DEC H
    let mut t: u16 = (cpu.get_hl() >> 8) - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += ((((cpu.get_hl() >> 8) & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_26(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 26 LD H,d8
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (v << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DAA_27(cpu: &CPU, memory: &Memory) -> u32 { // 27 DAA
    let mut t: u16 = cpu.a;
    corr = 0;
    corr |= 0x06 if cpu.f_h() else 0x00;
    corr |= 0x60 if cpu.f_c() else 0x00;
    if cpu.f_n() -> u32 {
        t -= corr;
    } else {
        corr |= 0x06 if (t & 0x0F) > 0x09 else 0x00;
        corr |= 0x60 if t > 0x99 else 0x00;
        t += corr;
    }
    let mut flag: u16 = 0;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (corr & 0x60 != 0) << FLAGC;
    cpu.f &= 0b01000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_28(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 28 JR Z,r8
    cpu.pc += 2;
    if cpu.f_z() -> u32 {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn ADD_29(cpu: &CPU, memory: &Memory) -> u32 { // 29 ADD HL,HL
    let mut t: u16 = cpu.get_hl() + cpu.get_hl();
    let mut flag: u16 = 0b00000000;
    flag += (((cpu.get_hl() & 0xFFF) + (cpu.get_hl() & 0xFFF)) > 0xFFF) << FLAGH;
    flag += (t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl() = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_2A(cpu: &CPU, memory: &Memory) -> u32 { // 2A LD A,(HL+)
    cpu.a = memory.read8(cpu.get_hl());
    cpu.set_hl() = cpu.get_hl() +1;
    cpu.set_hl() = cpu.get_hl() & 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_2B(cpu: &CPU, memory: &Memory) -> u32 { // 2B DEC HL
    let mut t: u16 = cpu.get_hl() - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_hl() = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_2C(cpu: &CPU, memory: &Memory) -> u32 { // 2C INC L
    let mut t: u16 = (cpu.get_hl() & 0xFF) + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += ((((cpu.get_hl() & 0xFF) & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_2D(cpu: &CPU, memory: &Memory) -> u32 { // 2D DEC L
    let mut t: u16 = (cpu.get_hl() & 0xFF) - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += ((((cpu.get_hl() & 0xFF) & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_2E(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 2E LD L,d8
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (v & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn CPL_2F(cpu: &CPU, memory: &Memory) -> u32 { // 2F CPL
    cpu.a = (!cpu.a) & 0xFF;
    let mut flag: u16 = 0b01100000;
    cpu.f &= 0b10010000;
    cpu.f |= flag;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_30(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 30 JR NC,r8
    cpu.pc += 2;
    if cpu.f_nc() -> u32 {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn LD_31(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 31 LD SP,d16
    cpu.sp = v;
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_32(cpu: &CPU, memory: &Memory) -> u32 { // 32 LD (HL-),A
    memory.write8(cpu.get_hl(), cpu.a);
    cpu.set_hl() = cpu.get_hl() - 1;
    cpu.set_hl() = cpu.get_hl() & 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_33(cpu: &CPU, memory: &Memory) -> u32 { // 33 INC SP
    let mut t: u16 = cpu.sp + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.sp = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_34(cpu: &CPU, memory: &Memory) -> u32 { // 34 INC (HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((memory.read8(cpu.get_hl()) & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn DEC_35(cpu: &CPU, memory: &Memory) -> u32 { // 35 DEC (HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((memory.read8(cpu.get_hl()) & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_36(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 36 LD (HL),d8
    memory.write8(cpu.get_hl(), v);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn SCF_37(cpu: &CPU, memory: &Memory) -> u32 { // 37 SCF
    let mut flag: u16 = 0b00010000;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_38(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 38 JR C,r8
    cpu.pc += 2;
    if cpu.f_c() -> u32 {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn ADD_39(cpu: &CPU, memory: &Memory) -> u32 { // 39 ADD HL,SP
    let mut t: u16 = cpu.get_hl() + cpu.sp;
    let mut flag: u16 = 0b00000000;
    flag += (((cpu.get_hl() & 0xFFF) + (cpu.sp & 0xFFF)) > 0xFFF) << FLAGH;
    flag += (t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl() = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_3A(cpu: &CPU, memory: &Memory) -> u32 { // 3A LD A,(HL-)
    cpu.a = memory.read8(cpu.get_hl());
    cpu.set_hl() = cpu.get_hl() - 1;
    cpu.set_hl() = cpu.get_hl() &  0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_3B(cpu: &CPU, memory: &Memory) -> u32 { // 3B DEC SP
    let mut t: u16 = cpu.sp - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.sp = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_3C(cpu: &CPU, memory: &Memory) -> u32 { // 3C INC A
    let mut t: u16 = cpu.a + 1;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_3D(cpu: &CPU, memory: &Memory) -> u32 { // 3D DEC A
    let mut t: u16 = cpu.a - 1;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_3E(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // 3E LD A,d8
    cpu.a = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn CCF_3F(cpu: &CPU, memory: &Memory) -> u32 { // 3F CCF
    let mut flag: u16 = (cpu.f & 0b00010000) ^ 0b00010000;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_40(cpu: &CPU, memory: &Memory) -> u32 { // 40 LD B,B
    cpu.b = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_41(cpu: &CPU, memory: &Memory) -> u32 { // 41 LD B,C
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_42(cpu: &CPU, memory: &Memory) -> u32 { // 42 LD B,D
    cpu.b = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_43(cpu: &CPU, memory: &Memory) -> u32 { // 43 LD B,E
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_44(cpu: &CPU, memory: &Memory) -> u32 { // 44 LD B,H
    cpu.b = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_45(cpu: &CPU, memory: &Memory) -> u32 { // 45 LD B,L
    cpu.b = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_46(cpu: &CPU, memory: &Memory) -> u32 { // 46 LD B,(HL)
    cpu.b = memory.read8(cpu.get_hl());
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_47(cpu: &CPU, memory: &Memory) -> u32 { // 47 LD B,A
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_48(cpu: &CPU, memory: &Memory) -> u32 { // 48 LD C,B
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_49(cpu: &CPU, memory: &Memory) -> u32 { // 49 LD C,C
    cpu.c = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4A(cpu: &CPU, memory: &Memory) -> u32 { // 4A LD C,D
    cpu.c = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4B(cpu: &CPU, memory: &Memory) -> u32 { // 4B LD C,E
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4C(cpu: &CPU, memory: &Memory) -> u32 { // 4C LD C,H
    cpu.c = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4D(cpu: &CPU, memory: &Memory) -> u32 { // 4D LD C,L
    cpu.c = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4E(cpu: &CPU, memory: &Memory) -> u32 { // 4E LD C,(HL)
    cpu.c = memory.read8(cpu.get_hl());
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_4F(cpu: &CPU, memory: &Memory) -> u32 { // 4F LD C,A
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_50(cpu: &CPU, memory: &Memory) -> u32 { // 50 LD D,B
    cpu.d = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_51(cpu: &CPU, memory: &Memory) -> u32 { // 51 LD D,C
    cpu.d = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_52(cpu: &CPU, memory: &Memory) -> u32 { // 52 LD D,D
    cpu.d = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_53(cpu: &CPU, memory: &Memory) -> u32 { // 53 LD D,E
    cpu.d = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_54(cpu: &CPU, memory: &Memory) -> u32 { // 54 LD D,H
    cpu.d = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_55(cpu: &CPU, memory: &Memory) -> u32 { // 55 LD D,L
    cpu.d = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_56(cpu: &CPU, memory: &Memory) -> u32 { // 56 LD D,(HL)
    cpu.d = memory.read8(cpu.get_hl());
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_57(cpu: &CPU, memory: &Memory) -> u32 { // 57 LD D,A
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_58(cpu: &CPU, memory: &Memory) -> u32 { // 58 LD E,B
    cpu.e = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_59(cpu: &CPU, memory: &Memory) -> u32 { // 59 LD E,C
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5A(cpu: &CPU, memory: &Memory) -> u32 { // 5A LD E,D
    cpu.e = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5B(cpu: &CPU, memory: &Memory) -> u32 { // 5B LD E,E
    cpu.e = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5C(cpu: &CPU, memory: &Memory) -> u32 { // 5C LD E,H
    cpu.e = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5D(cpu: &CPU, memory: &Memory) -> u32 { // 5D LD E,L
    cpu.e = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5E(cpu: &CPU, memory: &Memory) -> u32 { // 5E LD E,(HL)
    cpu.e = memory.read8(cpu.get_hl());
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_5F(cpu: &CPU, memory: &Memory) -> u32 { // 5F LD E,A
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_60(cpu: &CPU, memory: &Memory) -> u32 { // 60 LD H,B
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (cpu.b << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_61(cpu: &CPU, memory: &Memory) -> u32 { // 61 LD H,C
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (cpu.c << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_62(cpu: &CPU, memory: &Memory) -> u32 { // 62 LD H,D
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (cpu.d << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_63(cpu: &CPU, memory: &Memory) -> u32 { // 63 LD H,E
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (cpu.e << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_64(cpu: &CPU, memory: &Memory) -> u32 { // 64 LD H,H
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | ((cpu.get_hl() >> 8) << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_65(cpu: &CPU, memory: &Memory) -> u32 { // 65 LD H,L
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | ((cpu.get_hl() & 0xFF) << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_66(cpu: &CPU, memory: &Memory) -> u32 { // 66 LD H,(HL)
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (memory.read8(cpu.get_hl()) << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_67(cpu: &CPU, memory: &Memory) -> u32 { // 67 LD H,A
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (cpu.a << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_68(cpu: &CPU, memory: &Memory) -> u32 { // 68 LD L,B
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (cpu.b & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_69(cpu: &CPU, memory: &Memory) -> u32 { // 69 LD L,C
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (cpu.c & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6A(cpu: &CPU, memory: &Memory) -> u32 { // 6A LD L,D
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (cpu.d & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6B(cpu: &CPU, memory: &Memory) -> u32 { // 6B LD L,E
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (cpu.e & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6C(cpu: &CPU, memory: &Memory) -> u32 { // 6C LD L,H
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | ((cpu.get_hl() >> 8) & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6D(cpu: &CPU, memory: &Memory) -> u32 { // 6D LD L,L
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | ((cpu.get_hl() & 0xFF) & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6E(cpu: &CPU, memory: &Memory) -> u32 { // 6E LD L,(HL)
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (memory.read8(cpu.get_hl()) & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_6F(cpu: &CPU, memory: &Memory) -> u32 { // 6F LD L,A
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (cpu.a & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_70(cpu: &CPU, memory: &Memory) -> u32 { // 70 LD (HL),B
    memory.write8(cpu.get_hl(), cpu.b);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_71(cpu: &CPU, memory: &Memory) -> u32 { // 71 LD (HL),C
    memory.write8(cpu.get_hl(), cpu.c);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_72(cpu: &CPU, memory: &Memory) -> u32 { // 72 LD (HL),D
    memory.write8(cpu.get_hl(), cpu.d);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_73(cpu: &CPU, memory: &Memory) -> u32 { // 73 LD (HL),E
    memory.write8(cpu.get_hl(), cpu.e);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_74(cpu: &CPU, memory: &Memory) -> u32 { // 74 LD (HL),H
    memory.write8(cpu.get_hl(), (cpu.get_hl() >> 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_75(cpu: &CPU, memory: &Memory) -> u32 { // 75 LD (HL),L
    memory.write8(cpu.get_hl(), (cpu.get_hl() & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn HALT_76(cpu: &CPU, memory: &Memory) -> u32 { // 76 HALT
    cpu.halted = true;
    return 4;
}

fn LD_77(cpu: &CPU, memory: &Memory) -> u32 { // 77 LD (HL),A
    memory.write8(cpu.get_hl(), cpu.a);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_78(cpu: &CPU, memory: &Memory) -> u32 { // 78 LD A,B
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_79(cpu: &CPU, memory: &Memory) -> u32 { // 79 LD A,C
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7A(cpu: &CPU, memory: &Memory) -> u32 { // 7A LD A,D
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7B(cpu: &CPU, memory: &Memory) -> u32 { // 7B LD A,E
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7C(cpu: &CPU, memory: &Memory) -> u32 { // 7C LD A,H
    cpu.a = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7D(cpu: &CPU, memory: &Memory) -> u32 { // 7D LD A,L
    cpu.a = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7E(cpu: &CPU, memory: &Memory) -> u32 { // 7E LD A,(HL)
    cpu.a = memory.read8(cpu.get_hl());
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_7F(cpu: &CPU, memory: &Memory) -> u32 { // 7F LD A,A
    cpu.a = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_80(cpu: &CPU, memory: &Memory) -> u32 { // 80 ADD A,B
    let mut t: u16 = cpu.a + cpu.b;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.b & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_81(cpu: &CPU, memory: &Memory) -> u32 { // 81 ADD A,C
    let mut t: u16 = cpu.a + cpu.c;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.c & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_82(cpu: &CPU, memory: &Memory) -> u32 { // 82 ADD A,D
    let mut t: u16 = cpu.a + cpu.d;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.d & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_83(cpu: &CPU, memory: &Memory) -> u32 { // 83 ADD A,E
    let mut t: u16 = cpu.a + cpu.e;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.e & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_84(cpu: &CPU, memory: &Memory) -> u32 { // 84 ADD A,H
    let mut t: u16 = cpu.a + (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + ((cpu.get_hl() >> 8) & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_85(cpu: &CPU, memory: &Memory) -> u32 { // 85 ADD A,L
    let mut t: u16 = cpu.a + (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + ((cpu.get_hl() & 0xFF) & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_86(cpu: &CPU, memory: &Memory) -> u32 { // 86 ADD A,(HL)
    let mut t: u16 = cpu.a + memory.read8(cpu.get_hl());
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (memory.read8(cpu.get_hl()) & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn ADD_87(cpu: &CPU, memory: &Memory) -> u32 { // 87 ADD A,A
    let mut t: u16 = cpu.a + cpu.a;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.a & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_88(cpu: &CPU, memory: &Memory) -> u32 { // 88 ADC A,B
    let mut t: u16 = cpu.a + cpu.b + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.b & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_89(cpu: &CPU, memory: &Memory) -> u32 { // 89 ADC A,C
    let mut t: u16 = cpu.a + cpu.c + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.c & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8A(cpu: &CPU, memory: &Memory) -> u32 { // 8A ADC A,D
    let mut t: u16 = cpu.a + cpu.d + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.d & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8B(cpu: &CPU, memory: &Memory) -> u32 { // 8B ADC A,E
    let mut t: u16 = cpu.a + cpu.e + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.e & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8C(cpu: &CPU, memory: &Memory) -> u32 { // 8C ADC A,H
    let mut t: u16 = cpu.a + (cpu.get_hl() >> 8) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + ((cpu.get_hl() >> 8) & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8D(cpu: &CPU, memory: &Memory) -> u32 { // 8D ADC A,L
    let mut t: u16 = cpu.a + (cpu.get_hl() & 0xFF) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + ((cpu.get_hl() & 0xFF) & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8E(cpu: &CPU, memory: &Memory) -> u32 { // 8E ADC A,(HL)
    let mut t: u16 = cpu.a + memory.read8(cpu.get_hl()) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (memory.read8(cpu.get_hl()) & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn ADC_8F(cpu: &CPU, memory: &Memory) -> u32 { // 8F ADC A,A
    let mut t: u16 = cpu.a + cpu.a + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (cpu.a & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_90(cpu: &CPU, memory: &Memory) -> u32 { // 90 SUB B
    let mut t: u16 = cpu.a - cpu.b;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.b & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_91(cpu: &CPU, memory: &Memory) -> u32 { // 91 SUB C
    let mut t: u16 = cpu.a - cpu.c;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.c & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_92(cpu: &CPU, memory: &Memory) -> u32 { // 92 SUB D
    let mut t: u16 = cpu.a - cpu.d;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.d & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_93(cpu: &CPU, memory: &Memory) -> u32 { // 93 SUB E
    let mut t: u16 = cpu.a - cpu.e;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.e & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_94(cpu: &CPU, memory: &Memory) -> u32 { // 94 SUB H
    let mut t: u16 = cpu.a - (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - ((cpu.get_hl() >> 8) & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_95(cpu: &CPU, memory: &Memory) -> u32 { // 95 SUB L
    let mut t: u16 = cpu.a - (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - ((cpu.get_hl() & 0xFF) & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_96(cpu: &CPU, memory: &Memory) -> u32 { // 96 SUB (HL)
    let mut t: u16 = cpu.a - memory.read8(cpu.get_hl());
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (memory.read8(cpu.get_hl()) & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SUB_97(cpu: &CPU, memory: &Memory) -> u32 { // 97 SUB A
    let mut t: u16 = cpu.a - cpu.a;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.a & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_98(cpu: &CPU, memory: &Memory) -> u32 { // 98 SBC A,B
    let mut t: u16 = cpu.a - cpu.b - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.b & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_99(cpu: &CPU, memory: &Memory) -> u32 { // 99 SBC A,C
    let mut t: u16 = cpu.a - cpu.c - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.c & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9A(cpu: &CPU, memory: &Memory) -> u32 { // 9A SBC A,D
    let mut t: u16 = cpu.a - cpu.d - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.d & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9B(cpu: &CPU, memory: &Memory) -> u32 { // 9B SBC A,E
    let mut t: u16 = cpu.a - cpu.e - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.e & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9C(cpu: &CPU, memory: &Memory) -> u32 { // 9C SBC A,H
    let mut t: u16 = cpu.a - (cpu.get_hl() >> 8) - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - ((cpu.get_hl() >> 8) & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9D(cpu: &CPU, memory: &Memory) -> u32 { // 9D SBC A,L
    let mut t: u16 = cpu.a - (cpu.get_hl() & 0xFF) - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - ((cpu.get_hl() & 0xFF) & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9E(cpu: &CPU, memory: &Memory) -> u32 { // 9E SBC A,(HL)
    let mut t: u16 = cpu.a - memory.read8(cpu.get_hl()) - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (memory.read8(cpu.get_hl()) & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SBC_9F(cpu: &CPU, memory: &Memory) -> u32 { // 9F SBC A,A
    let mut t: u16 = cpu.a - cpu.a - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.a & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A0(cpu: &CPU, memory: &Memory) -> u32 { // A0 AND B
    let mut t: u16 = cpu.a & cpu.b;
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A1(cpu: &CPU, memory: &Memory) -> u32 { // A1 AND C
    let mut t: u16 = cpu.a & cpu.c;
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A2(cpu: &CPU, memory: &Memory) -> u32 { // A2 AND D
    let mut t: u16 = cpu.a & cpu.d;
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A3(cpu: &CPU, memory: &Memory) -> u32 { // A3 AND E
    let mut t: u16 = cpu.a & cpu.e;
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A4(cpu: &CPU, memory: &Memory) -> u32 { // A4 AND H
    let mut t: u16 = cpu.a & (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A5(cpu: &CPU, memory: &Memory) -> u32 { // A5 AND L
    let mut t: u16 = cpu.a & (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A6(cpu: &CPU, memory: &Memory) -> u32 { // A6 AND (HL)
    let mut t: u16 = cpu.a & memory.read8(cpu.get_hl());
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn AND_A7(cpu: &CPU, memory: &Memory) -> u32 { // A7 AND A
    let mut t: u16 = cpu.a & cpu.a;
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_A8(cpu: &CPU, memory: &Memory) -> u32 { // A8 XOR B
    let mut t: u16 = cpu.a ^ cpu.b;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_A9(cpu: &CPU, memory: &Memory) -> u32 { // A9 XOR C
    let mut t: u16 = cpu.a ^ cpu.c;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AA(cpu: &CPU, memory: &Memory) -> u32 { // AA XOR D
    let mut t: u16 = cpu.a ^ cpu.d;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AB(cpu: &CPU, memory: &Memory) -> u32 { // AB XOR E
    let mut t: u16 = cpu.a ^ cpu.e;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AC(cpu: &CPU, memory: &Memory) -> u32 { // AC XOR H
    let mut t: u16 = cpu.a ^ (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AD(cpu: &CPU, memory: &Memory) -> u32 { // AD XOR L
    let mut t: u16 = cpu.a ^ (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AE(cpu: &CPU, memory: &Memory) -> u32 { // AE XOR (HL)
    let mut t: u16 = cpu.a ^ memory.read8(cpu.get_hl());
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn XOR_AF(cpu: &CPU, memory: &Memory) -> u32 { // AF XOR A
    let mut t: u16 = cpu.a ^ cpu.a;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B0(cpu: &CPU, memory: &Memory) -> u32 { // B0 OR B
    let mut t: u16 = cpu.a | cpu.b;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B1(cpu: &CPU, memory: &Memory) -> u32 { // B1 OR C
    let mut t: u16 = cpu.a | cpu.c;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B2(cpu: &CPU, memory: &Memory) -> u32 { // B2 OR D
    let mut t: u16 = cpu.a | cpu.d;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B3(cpu: &CPU, memory: &Memory) -> u32 { // B3 OR E
    let mut t: u16 = cpu.a | cpu.e;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B4(cpu: &CPU, memory: &Memory) -> u32 { // B4 OR H
    let mut t: u16 = cpu.a | (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B5(cpu: &CPU, memory: &Memory) -> u32 { // B5 OR L
    let mut t: u16 = cpu.a | (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B6(cpu: &CPU, memory: &Memory) -> u32 { // B6 OR (HL)
    let mut t: u16 = cpu.a | memory.read8(cpu.get_hl());
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn OR_B7(cpu: &CPU, memory: &Memory) -> u32 { // B7 OR A
    let mut t: u16 = cpu.a | cpu.a;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_B8(cpu: &CPU, memory: &Memory) -> u32 { // B8 CP B
    let mut t: u16 = cpu.a - cpu.b;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.b & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_B9(cpu: &CPU, memory: &Memory) -> u32 { // B9 CP C
    let mut t: u16 = cpu.a - cpu.c;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.c & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BA(cpu: &CPU, memory: &Memory) -> u32 { // BA CP D
    let mut t: u16 = cpu.a - cpu.d;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.d & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BB(cpu: &CPU, memory: &Memory) -> u32 { // BB CP E
    let mut t: u16 = cpu.a - cpu.e;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.e & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BC(cpu: &CPU, memory: &Memory) -> u32 { // BC CP H
    let mut t: u16 = cpu.a - (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - ((cpu.get_hl() >> 8) & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BD(cpu: &CPU, memory: &Memory) -> u32 { // BD CP L
    let mut t: u16 = cpu.a - (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - ((cpu.get_hl() & 0xFF) & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BE(cpu: &CPU, memory: &Memory) -> u32 { // BE CP (HL)
    let mut t: u16 = cpu.a - memory.read8(cpu.get_hl());
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (memory.read8(cpu.get_hl()) & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn CP_BF(cpu: &CPU, memory: &Memory) -> u32 { // BF CP A
    let mut t: u16 = cpu.a - cpu.a;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (cpu.a & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn RET_C0(cpu: &CPU, memory: &Memory) -> u32 { // C0 RET NZ
    if cpu.f_nz() -> u32 {
        cpu.pc = memory.read8((cpu.sp + 1) & 0xFFFF) << 8 // High;
        cpu.pc |= memory.read8(cpu.sp) // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn POP_C1(cpu: &CPU, memory: &Memory) -> u32 { // C1 POP BC
    cpu.b = memory.read8((cpu.sp + 1) & 0xFFFF) // High;
    cpu.c = memory.read8(cpu.sp) // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn JP_C2(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // C2 JP NZ,a16
    if cpu.f_nz() -> u32 {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn JP_C3(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // C3 JP a16
    cpu.pc = v;
    return 16;
}

fn CALL_C4(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // C4 CALL NZ,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_nz() -> u32 {
        memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn PUSH_C5(cpu: &CPU, memory: &Memory) -> u32 { // C5 PUSH BC
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.b) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.c) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn ADD_C6(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // C6 ADD A,d8
    let mut t: u16 = cpu.a + v;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (v & 0xF)) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_C7(cpu: &CPU, memory: &Memory) -> u32 { // C7 RST 00H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 0;
    return 16;
}

fn RET_C8(cpu: &CPU, memory: &Memory) -> u32 { // C8 RET Z
    if cpu.f_z() -> u32 {
        cpu.pc = memory.read8((cpu.sp + 1) & 0xFFFF) << 8 // High;
        cpu.pc |= memory.read8(cpu.sp) // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn RET_C9(cpu: &CPU, memory: &Memory) -> u32 { // C9 RET
    cpu.pc = memory.read8((cpu.sp + 1) & 0xFFFF) << 8 // High;
    cpu.pc |= memory.read8(cpu.sp) // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    return 16;
}

fn JP_CA(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // CA JP Z,a16
    if cpu.f_z() -> u32 {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn PREFIX_CB(cpu: &CPU, memory: &Memory) -> u32 { // CB PREFIX CB
    log::error!("CB cannot be called!");
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CALL_CC(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // CC CALL Z,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_z() -> u32 {
        memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn CALL_CD(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // CD CALL a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = v;
    return 24;
}

fn ADC_CE(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // CE ADC A,d8
    let mut t: u16 = cpu.a + v + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) + (v & 0xF) + cpu.f_c()) > 0xF) << FLAGH;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_CF(cpu: &CPU, memory: &Memory) -> u32 { // CF RST 08H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 8;
    return 16;
}

fn RET_D0(cpu: &CPU, memory: &Memory) -> u32 { // D0 RET NC
    if cpu.f_nc() -> u32 {
        cpu.pc = memory.read8((cpu.sp + 1) & 0xFFFF) << 8 // High;
        cpu.pc |= memory.read8(cpu.sp) // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn POP_D1(cpu: &CPU, memory: &Memory) -> u32 { // D1 POP DE
    cpu.d = memory.read8((cpu.sp + 1) & 0xFFFF) // High;
    cpu.e = memory.read8(cpu.sp) // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn JP_D2(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // D2 JP NC,a16
    if cpu.f_nc() -> u32 {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn CALL_D4(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // D4 CALL NC,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_nc() -> u32 {
        memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn PUSH_D5(cpu: &CPU, memory: &Memory) -> u32 { // D5 PUSH DE
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.d) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.e) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SUB_D6(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // D6 SUB d8
    let mut t: u16 = cpu.a - v;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (v & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_D7(cpu: &CPU, memory: &Memory) -> u32 { // D7 RST 10H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 16;
    return 16;
}

fn RET_D8(cpu: &CPU, memory: &Memory) -> u32 { // D8 RET C
    if cpu.f_c() -> u32 {
        cpu.pc = memory.read8((cpu.sp + 1) & 0xFFFF) << 8 // High;
        cpu.pc |= memory.read8(cpu.sp) // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn RETI_D9(cpu: &CPU, memory: &Memory) -> u32 { // D9 RETI
    cpu.interrupt_master_enable = true;
    cpu.pc = memory.read8((cpu.sp + 1) & 0xFFFF) << 8 // High;
    cpu.pc |= memory.read8(cpu.sp) // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    return 16;
}

fn JP_DA(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // DA JP C,a16
    if cpu.f_c() -> u32 {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn CALL_DC(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // DC CALL C,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_c() -> u32 {
        memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn SBC_DE(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // DE SBC A,d8
    let mut t: u16 = cpu.a - v - cpu.f_c();
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (v & 0xF) - cpu.f_c()) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_DF(cpu: &CPU, memory: &Memory) -> u32 { // DF RST 18H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 24;
    return 16;
}

fn LDH_E0(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // E0 LDH (a8),A
    memory.write8(v + 0xFF00, cpu.a);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn POP_E1(cpu: &CPU, memory: &Memory) -> u32 { // E1 POP HL
    cpu.set_hl() =(memory.read8((cpu.sp + 1) & 0xFFFF) << 8) + memory.read8(cpu.sp) // High;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_E2(cpu: &CPU, memory: &Memory) -> u32 { // E2 LD (C),A
    memory.write8(0xFF00 + cpu.c, cpu.a);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn PUSH_E5(cpu: &CPU, memory: &Memory) -> u32 { // E5 PUSH HL
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.get_hl() >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.get_hl() & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn AND_E6(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // E6 AND d8
    let mut t: u16 = cpu.a & v;
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_E7(cpu: &CPU, memory: &Memory) -> u32 { // E7 RST 20H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 32;
    return 16;
}

fn ADD_E8(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // E8 ADD SP,r8
    let mut t: u16 = cpu.sp + ((v ^ 0x80) - 0x80);
    let mut flag: u16 = 0b00000000;
    flag += (((cpu.sp & 0xF) + (v & 0xF)) > 0xF) << FLAGH;
    flag += (((cpu.sp & 0xFF) + (v & 0xFF)) > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.sp = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn JP_E9(cpu: &CPU, memory: &Memory) -> u32 { // E9 JP (HL)
    cpu.pc = cpu.get_hl();
    return 4;
}

fn LD_EA(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // EA LD (a16),A
    memory.write8(v, cpu.a);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn XOR_EE(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // EE XOR d8
    let mut t: u16 = cpu.a ^ v;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_EF(cpu: &CPU, memory: &Memory) -> u32 { // EF RST 28H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 40;
    return 16;
}

fn LDH_F0(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // F0 LDH A,(a8)
    cpu.a = memory.read8(v + 0xFF00);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn POP_F1(cpu: &CPU, memory: &Memory) -> u32 { // F1 POP AF
    cpu.a = memory.read8((cpu.sp + 1) & 0xFFFF) // High;
    cpu.f = memory.read8(cpu.sp) & 0xF0 & 0xF0 // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_F2(cpu: &CPU, memory: &Memory) -> u32 { // F2 LD A,(C)
    cpu.a = memory.read8(0xFF00 + cpu.c);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DI_F3(cpu: &CPU, memory: &Memory) -> u32 { // F3 DI
    cpu.interrupt_master_enable = false;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn PUSH_F5(cpu: &CPU, memory: &Memory) -> u32 { // F5 PUSH AF
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.a) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.f & 0xF0) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn OR_F6(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // F6 OR d8
    let mut t: u16 = cpu.a | v;
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_F7(cpu: &CPU, memory: &Memory) -> u32 { // F7 RST 30H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 48;
    return 16;
}

fn LD_F8(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // F8 LD HL,SP+r8
    cpu.set_hl() =cpu.sp + ((v ^ 0x80) - 0x80);
    let mut t: u16 = cpu.get_hl();
    let mut flag: u16 = 0b00000000;
    flag += (((cpu.sp & 0xF) + (v & 0xF)) > 0xF) << FLAGH;
    flag += (((cpu.sp & 0xFF) + (v & 0xFF)) > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    cpu.set_hl() = cpu.get_hl() & 0xFFFF;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_F9(cpu: &CPU, memory: &Memory) -> u32 { // F9 LD SP,HL
    cpu.sp = cpu.get_hl();
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_FA(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // FA LD A,(a16)
    cpu.a = memory.read8(v);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn EI_FB(cpu: &CPU, memory: &Memory) -> u32 { // FB EI
    cpu.interrupt_master_enable = true;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_FE(cpu: &CPU, memory: &Memory, v: u8) -> u32 { // FE CP d8
    let mut t: u16 = cpu.a - v;
    let mut flag: u16 = 0b01000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (((cpu.a & 0xF) - (v & 0xF)) < 0) << FLAGH;
    flag += (t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_FF(cpu: &CPU, memory: &Memory) -> u32 { // FF RST 38H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, cpu.pc >> 8) // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, cpu.pc & 0xFF) // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 56;
    return 16;
}

fn RLC_100(cpu: &CPU, memory: &Memory) -> u32 { // 100 RLC B
    let mut t: u16 = (cpu.b << 1) + (cpu.b >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_101(cpu: &CPU, memory: &Memory) -> u32 { // 101 RLC C
    let mut t: u16 = (cpu.c << 1) + (cpu.c >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_102(cpu: &CPU, memory: &Memory) -> u32 { // 102 RLC D
    let mut t: u16 = (cpu.d << 1) + (cpu.d >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_103(cpu: &CPU, memory: &Memory) -> u32 { // 103 RLC E
    let mut t: u16 = (cpu.e << 1) + (cpu.e >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_104(cpu: &CPU, memory: &Memory) -> u32 { // 104 RLC H
    let mut t: u16 = ((cpu.get_hl() >> 8) << 1) + ((cpu.get_hl() >> 8) >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_105(cpu: &CPU, memory: &Memory) -> u32 { // 105 RLC L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) << 1) + ((cpu.get_hl() & 0xFF) >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_106(cpu: &CPU, memory: &Memory) -> u32 { // 106 RLC (HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()) << 1) + (memory.read8(cpu.get_hl()) >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RLC_107(cpu: &CPU, memory: &Memory) -> u32 { // 107 RLC A
    let mut t: u16 = (cpu.a << 1) + (cpu.a >> 7);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_108(cpu: &CPU, memory: &Memory) -> u32 { // 108 RRC B
    let mut t: u16 = (cpu.b >> 1) + ((cpu.b & 1) << 7) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_109(cpu: &CPU, memory: &Memory) -> u32 { // 109 RRC C
    let mut t: u16 = (cpu.c >> 1) + ((cpu.c & 1) << 7) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10A(cpu: &CPU, memory: &Memory) -> u32 { // 10A RRC D
    let mut t: u16 = (cpu.d >> 1) + ((cpu.d & 1) << 7) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10B(cpu: &CPU, memory: &Memory) -> u32 { // 10B RRC E
    let mut t: u16 = (cpu.e >> 1) + ((cpu.e & 1) << 7) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10C(cpu: &CPU, memory: &Memory) -> u32 { // 10C RRC H
    let mut t: u16 = ((cpu.get_hl() >> 8) >> 1) + (((cpu.get_hl() >> 8) & 1) << 7) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10D(cpu: &CPU, memory: &Memory) -> u32 { // 10D RRC L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) >> 1) + (((cpu.get_hl() & 0xFF) & 1) << 7) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10E(cpu: &CPU, memory: &Memory) -> u32 { // 10E RRC (HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()) >> 1) + ((memory.read8(cpu.get_hl()) & 1) << 7) + ((memory.read8(cpu.get_hl()) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RRC_10F(cpu: &CPU, memory: &Memory) -> u32 { // 10F RRC A
    let mut t: u16 = (cpu.a >> 1) + ((cpu.a & 1) << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_110(cpu: &CPU, memory: &Memory) -> u32 { // 110 RL B
    let mut t: u16 = (cpu.b << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_111(cpu: &CPU, memory: &Memory) -> u32 { // 111 RL C
    let mut t: u16 = (cpu.c << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_112(cpu: &CPU, memory: &Memory) -> u32 { // 112 RL D
    let mut t: u16 = (cpu.d << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_113(cpu: &CPU, memory: &Memory) -> u32 { // 113 RL E
    let mut t: u16 = (cpu.e << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_114(cpu: &CPU, memory: &Memory) -> u32 { // 114 RL H
    let mut t: u16 = ((cpu.get_hl() >> 8) << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_115(cpu: &CPU, memory: &Memory) -> u32 { // 115 RL L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_116(cpu: &CPU, memory: &Memory) -> u32 { // 116 RL (HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()) << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RL_117(cpu: &CPU, memory: &Memory) -> u32 { // 117 RL A
    let mut t: u16 = (cpu.a << 1) + cpu.f_c();
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_118(cpu: &CPU, memory: &Memory) -> u32 { // 118 RR B
    let mut t: u16 = (cpu.b >> 1) + (cpu.f_c() << 7) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_119(cpu: &CPU, memory: &Memory) -> u32 { // 119 RR C
    let mut t: u16 = (cpu.c >> 1) + (cpu.f_c() << 7) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11A(cpu: &CPU, memory: &Memory) -> u32 { // 11A RR D
    let mut t: u16 = (cpu.d >> 1) + (cpu.f_c() << 7) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11B(cpu: &CPU, memory: &Memory) -> u32 { // 11B RR E
    let mut t: u16 = (cpu.e >> 1) + (cpu.f_c() << 7) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11C(cpu: &CPU, memory: &Memory) -> u32 { // 11C RR H
    let mut t: u16 = ((cpu.get_hl() >> 8) >> 1) + (cpu.f_c() << 7) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11D(cpu: &CPU, memory: &Memory) -> u32 { // 11D RR L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) >> 1) + (cpu.f_c() << 7) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11E(cpu: &CPU, memory: &Memory) -> u32 { // 11E RR (HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()) >> 1) + (cpu.f_c() << 7) + ((memory.read8(cpu.get_hl()) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RR_11F(cpu: &CPU, memory: &Memory) -> u32 { // 11F RR A
    let mut t: u16 = (cpu.a >> 1) + (cpu.f_c() << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_120(cpu: &CPU, memory: &Memory) -> u32 { // 120 SLA B
    let mut t: u16 = (cpu.b << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_121(cpu: &CPU, memory: &Memory) -> u32 { // 121 SLA C
    let mut t: u16 = (cpu.c << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_122(cpu: &CPU, memory: &Memory) -> u32 { // 122 SLA D
    let mut t: u16 = (cpu.d << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_123(cpu: &CPU, memory: &Memory) -> u32 { // 123 SLA E
    let mut t: u16 = (cpu.e << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_124(cpu: &CPU, memory: &Memory) -> u32 { // 124 SLA H
    let mut t: u16 = ((cpu.get_hl() >> 8) << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_125(cpu: &CPU, memory: &Memory) -> u32 { // 125 SLA L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_126(cpu: &CPU, memory: &Memory) -> u32 { // 126 SLA (HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()) << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SLA_127(cpu: &CPU, memory: &Memory) -> u32 { // 127 SLA A
    let mut t: u16 = (cpu.a << 1);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_128(cpu: &CPU, memory: &Memory) -> u32 { // 128 SRA B
    let mut t: u16 = ((cpu.b >> 1) | (cpu.b & 0x80)) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_129(cpu: &CPU, memory: &Memory) -> u32 { // 129 SRA C
    let mut t: u16 = ((cpu.c >> 1) | (cpu.c & 0x80)) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12A(cpu: &CPU, memory: &Memory) -> u32 { // 12A SRA D
    let mut t: u16 = ((cpu.d >> 1) | (cpu.d & 0x80)) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12B(cpu: &CPU, memory: &Memory) -> u32 { // 12B SRA E
    let mut t: u16 = ((cpu.e >> 1) | (cpu.e & 0x80)) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12C(cpu: &CPU, memory: &Memory) -> u32 { // 12C SRA H
    let mut t: u16 = (((cpu.get_hl() >> 8) >> 1) | ((cpu.get_hl() >> 8) & 0x80)) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12D(cpu: &CPU, memory: &Memory) -> u32 { // 12D SRA L
    let mut t: u16 = (((cpu.get_hl() & 0xFF) >> 1) | ((cpu.get_hl() & 0xFF) & 0x80)) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12E(cpu: &CPU, memory: &Memory) -> u32 { // 12E SRA (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()) >> 1) | (memory.read8(cpu.get_hl()) & 0x80)) + ((memory.read8(cpu.get_hl()) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SRA_12F(cpu: &CPU, memory: &Memory) -> u32 { // 12F SRA A
    let mut t: u16 = ((cpu.a >> 1) | (cpu.a & 0x80)) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_130(cpu: &CPU, memory: &Memory) -> u32 { // 130 SWAP B
    let mut t: u16 = ((cpu.b & 0xF0) >> 4) | ((cpu.b & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_131(cpu: &CPU, memory: &Memory) -> u32 { // 131 SWAP C
    let mut t: u16 = ((cpu.c & 0xF0) >> 4) | ((cpu.c & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_132(cpu: &CPU, memory: &Memory) -> u32 { // 132 SWAP D
    let mut t: u16 = ((cpu.d & 0xF0) >> 4) | ((cpu.d & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_133(cpu: &CPU, memory: &Memory) -> u32 { // 133 SWAP E
    let mut t: u16 = ((cpu.e & 0xF0) >> 4) | ((cpu.e & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_134(cpu: &CPU, memory: &Memory) -> u32 { // 134 SWAP H
    let mut t: u16 = (((cpu.get_hl() >> 8) & 0xF0) >> 4) | (((cpu.get_hl() >> 8) & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_135(cpu: &CPU, memory: &Memory) -> u32 { // 135 SWAP L
    let mut t: u16 = (((cpu.get_hl() & 0xFF) & 0xF0) >> 4) | (((cpu.get_hl() & 0xFF) & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_136(cpu: &CPU, memory: &Memory) -> u32 { // 136 SWAP (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()) & 0xF0) >> 4) | ((memory.read8(cpu.get_hl()) & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SWAP_137(cpu: &CPU, memory: &Memory) -> u32 { // 137 SWAP A
    let mut t: u16 = ((cpu.a & 0xF0) >> 4) | ((cpu.a & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_138(cpu: &CPU, memory: &Memory) -> u32 { // 138 SRL B
    let mut t: u16 = (cpu.b >> 1) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_139(cpu: &CPU, memory: &Memory) -> u32 { // 139 SRL C
    let mut t: u16 = (cpu.c >> 1) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13A(cpu: &CPU, memory: &Memory) -> u32 { // 13A SRL D
    let mut t: u16 = (cpu.d >> 1) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13B(cpu: &CPU, memory: &Memory) -> u32 { // 13B SRL E
    let mut t: u16 = (cpu.e >> 1) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13C(cpu: &CPU, memory: &Memory) -> u32 { // 13C SRL H
    let mut t: u16 = ((cpu.get_hl() >> 8) >> 1) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13D(cpu: &CPU, memory: &Memory) -> u32 { // 13D SRL L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) >> 1) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13E(cpu: &CPU, memory: &Memory) -> u32 { // 13E SRL (HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()) >> 1) + ((memory.read8(cpu.get_hl()) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SRL_13F(cpu: &CPU, memory: &Memory) -> u32 { // 13F SRL A
    let mut t: u16 = (cpu.a >> 1) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    flag += (t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_140(cpu: &CPU, memory: &Memory) -> u32 { // 140 BIT 0,B
    let mut t: u16 = cpu.b & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_141(cpu: &CPU, memory: &Memory) -> u32 { // 141 BIT 0,C
    let mut t: u16 = cpu.c & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_142(cpu: &CPU, memory: &Memory) -> u32 { // 142 BIT 0,D
    let mut t: u16 = cpu.d & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_143(cpu: &CPU, memory: &Memory) -> u32 { // 143 BIT 0,E
    let mut t: u16 = cpu.e & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_144(cpu: &CPU, memory: &Memory) -> u32 { // 144 BIT 0,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_145(cpu: &CPU, memory: &Memory) -> u32 { // 145 BIT 0,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_146(cpu: &CPU, memory: &Memory) -> u32 { // 146 BIT 0,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_147(cpu: &CPU, memory: &Memory) -> u32 { // 147 BIT 0,A
    let mut t: u16 = cpu.a & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_148(cpu: &CPU, memory: &Memory) -> u32 { // 148 BIT 1,B
    let mut t: u16 = cpu.b & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_149(cpu: &CPU, memory: &Memory) -> u32 { // 149 BIT 1,C
    let mut t: u16 = cpu.c & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14A(cpu: &CPU, memory: &Memory) -> u32 { // 14A BIT 1,D
    let mut t: u16 = cpu.d & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14B(cpu: &CPU, memory: &Memory) -> u32 { // 14B BIT 1,E
    let mut t: u16 = cpu.e & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14C(cpu: &CPU, memory: &Memory) -> u32 { // 14C BIT 1,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14D(cpu: &CPU, memory: &Memory) -> u32 { // 14D BIT 1,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14E(cpu: &CPU, memory: &Memory) -> u32 { // 14E BIT 1,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_14F(cpu: &CPU, memory: &Memory) -> u32 { // 14F BIT 1,A
    let mut t: u16 = cpu.a & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_150(cpu: &CPU, memory: &Memory) -> u32 { // 150 BIT 2,B
    let mut t: u16 = cpu.b & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_151(cpu: &CPU, memory: &Memory) -> u32 { // 151 BIT 2,C
    let mut t: u16 = cpu.c & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_152(cpu: &CPU, memory: &Memory) -> u32 { // 152 BIT 2,D
    let mut t: u16 = cpu.d & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_153(cpu: &CPU, memory: &Memory) -> u32 { // 153 BIT 2,E
    let mut t: u16 = cpu.e & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_154(cpu: &CPU, memory: &Memory) -> u32 { // 154 BIT 2,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_155(cpu: &CPU, memory: &Memory) -> u32 { // 155 BIT 2,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_156(cpu: &CPU, memory: &Memory) -> u32 { // 156 BIT 2,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_157(cpu: &CPU, memory: &Memory) -> u32 { // 157 BIT 2,A
    let mut t: u16 = cpu.a & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_158(cpu: &CPU, memory: &Memory) -> u32 { // 158 BIT 3,B
    let mut t: u16 = cpu.b & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_159(cpu: &CPU, memory: &Memory) -> u32 { // 159 BIT 3,C
    let mut t: u16 = cpu.c & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15A(cpu: &CPU, memory: &Memory) -> u32 { // 15A BIT 3,D
    let mut t: u16 = cpu.d & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15B(cpu: &CPU, memory: &Memory) -> u32 { // 15B BIT 3,E
    let mut t: u16 = cpu.e & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15C(cpu: &CPU, memory: &Memory) -> u32 { // 15C BIT 3,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15D(cpu: &CPU, memory: &Memory) -> u32 { // 15D BIT 3,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15E(cpu: &CPU, memory: &Memory) -> u32 { // 15E BIT 3,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_15F(cpu: &CPU, memory: &Memory) -> u32 { // 15F BIT 3,A
    let mut t: u16 = cpu.a & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_160(cpu: &CPU, memory: &Memory) -> u32 { // 160 BIT 4,B
    let mut t: u16 = cpu.b & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_161(cpu: &CPU, memory: &Memory) -> u32 { // 161 BIT 4,C
    let mut t: u16 = cpu.c & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_162(cpu: &CPU, memory: &Memory) -> u32 { // 162 BIT 4,D
    let mut t: u16 = cpu.d & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_163(cpu: &CPU, memory: &Memory) -> u32 { // 163 BIT 4,E
    let mut t: u16 = cpu.e & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_164(cpu: &CPU, memory: &Memory) -> u32 { // 164 BIT 4,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_165(cpu: &CPU, memory: &Memory) -> u32 { // 165 BIT 4,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_166(cpu: &CPU, memory: &Memory) -> u32 { // 166 BIT 4,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_167(cpu: &CPU, memory: &Memory) -> u32 { // 167 BIT 4,A
    let mut t: u16 = cpu.a & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_168(cpu: &CPU, memory: &Memory) -> u32 { // 168 BIT 5,B
    let mut t: u16 = cpu.b & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_169(cpu: &CPU, memory: &Memory) -> u32 { // 169 BIT 5,C
    let mut t: u16 = cpu.c & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16A(cpu: &CPU, memory: &Memory) -> u32 { // 16A BIT 5,D
    let mut t: u16 = cpu.d & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16B(cpu: &CPU, memory: &Memory) -> u32 { // 16B BIT 5,E
    let mut t: u16 = cpu.e & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16C(cpu: &CPU, memory: &Memory) -> u32 { // 16C BIT 5,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16D(cpu: &CPU, memory: &Memory) -> u32 { // 16D BIT 5,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16E(cpu: &CPU, memory: &Memory) -> u32 { // 16E BIT 5,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_16F(cpu: &CPU, memory: &Memory) -> u32 { // 16F BIT 5,A
    let mut t: u16 = cpu.a & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_170(cpu: &CPU, memory: &Memory) -> u32 { // 170 BIT 6,B
    let mut t: u16 = cpu.b & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_171(cpu: &CPU, memory: &Memory) -> u32 { // 171 BIT 6,C
    let mut t: u16 = cpu.c & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_172(cpu: &CPU, memory: &Memory) -> u32 { // 172 BIT 6,D
    let mut t: u16 = cpu.d & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_173(cpu: &CPU, memory: &Memory) -> u32 { // 173 BIT 6,E
    let mut t: u16 = cpu.e & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_174(cpu: &CPU, memory: &Memory) -> u32 { // 174 BIT 6,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_175(cpu: &CPU, memory: &Memory) -> u32 { // 175 BIT 6,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_176(cpu: &CPU, memory: &Memory) -> u32 { // 176 BIT 6,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_177(cpu: &CPU, memory: &Memory) -> u32 { // 177 BIT 6,A
    let mut t: u16 = cpu.a & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_178(cpu: &CPU, memory: &Memory) -> u32 { // 178 BIT 7,B
    let mut t: u16 = cpu.b & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_179(cpu: &CPU, memory: &Memory) -> u32 { // 179 BIT 7,C
    let mut t: u16 = cpu.c & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17A(cpu: &CPU, memory: &Memory) -> u32 { // 17A BIT 7,D
    let mut t: u16 = cpu.d & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17B(cpu: &CPU, memory: &Memory) -> u32 { // 17B BIT 7,E
    let mut t: u16 = cpu.e & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17C(cpu: &CPU, memory: &Memory) -> u32 { // 17C BIT 7,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17D(cpu: &CPU, memory: &Memory) -> u32 { // 17D BIT 7,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17E(cpu: &CPU, memory: &Memory) -> u32 { // 17E BIT 7,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_17F(cpu: &CPU, memory: &Memory) -> u32 { // 17F BIT 7,A
    let mut t: u16 = cpu.a & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += ((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_180(cpu: &CPU, memory: &Memory) -> u32 { // 180 RES 0,B
    let mut t: u16 = cpu.b & !(1 << 0);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_181(cpu: &CPU, memory: &Memory) -> u32 { // 181 RES 0,C
    let mut t: u16 = cpu.c & !(1 << 0);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_182(cpu: &CPU, memory: &Memory) -> u32 { // 182 RES 0,D
    let mut t: u16 = cpu.d & !(1 << 0);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_183(cpu: &CPU, memory: &Memory) -> u32 { // 183 RES 0,E
    let mut t: u16 = cpu.e & !(1 << 0);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_184(cpu: &CPU, memory: &Memory) -> u32 { // 184 RES 0,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 0);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_185(cpu: &CPU, memory: &Memory) -> u32 { // 185 RES 0,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 0);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_186(cpu: &CPU, memory: &Memory) -> u32 { // 186 RES 0,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 0);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_187(cpu: &CPU, memory: &Memory) -> u32 { // 187 RES 0,A
    let mut t: u16 = cpu.a & !(1 << 0);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_188(cpu: &CPU, memory: &Memory) -> u32 { // 188 RES 1,B
    let mut t: u16 = cpu.b & !(1 << 1);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_189(cpu: &CPU, memory: &Memory) -> u32 { // 189 RES 1,C
    let mut t: u16 = cpu.c & !(1 << 1);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18A(cpu: &CPU, memory: &Memory) -> u32 { // 18A RES 1,D
    let mut t: u16 = cpu.d & !(1 << 1);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18B(cpu: &CPU, memory: &Memory) -> u32 { // 18B RES 1,E
    let mut t: u16 = cpu.e & !(1 << 1);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18C(cpu: &CPU, memory: &Memory) -> u32 { // 18C RES 1,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 1);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18D(cpu: &CPU, memory: &Memory) -> u32 { // 18D RES 1,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 1);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18E(cpu: &CPU, memory: &Memory) -> u32 { // 18E RES 1,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 1);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_18F(cpu: &CPU, memory: &Memory) -> u32 { // 18F RES 1,A
    let mut t: u16 = cpu.a & !(1 << 1);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_190(cpu: &CPU, memory: &Memory) -> u32 { // 190 RES 2,B
    let mut t: u16 = cpu.b & !(1 << 2);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_191(cpu: &CPU, memory: &Memory) -> u32 { // 191 RES 2,C
    let mut t: u16 = cpu.c & !(1 << 2);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_192(cpu: &CPU, memory: &Memory) -> u32 { // 192 RES 2,D
    let mut t: u16 = cpu.d & !(1 << 2);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_193(cpu: &CPU, memory: &Memory) -> u32 { // 193 RES 2,E
    let mut t: u16 = cpu.e & !(1 << 2);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_194(cpu: &CPU, memory: &Memory) -> u32 { // 194 RES 2,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 2);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_195(cpu: &CPU, memory: &Memory) -> u32 { // 195 RES 2,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 2);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_196(cpu: &CPU, memory: &Memory) -> u32 { // 196 RES 2,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 2);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_197(cpu: &CPU, memory: &Memory) -> u32 { // 197 RES 2,A
    let mut t: u16 = cpu.a & !(1 << 2);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_198(cpu: &CPU, memory: &Memory) -> u32 { // 198 RES 3,B
    let mut t: u16 = cpu.b & !(1 << 3);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_199(cpu: &CPU, memory: &Memory) -> u32 { // 199 RES 3,C
    let mut t: u16 = cpu.c & !(1 << 3);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19A(cpu: &CPU, memory: &Memory) -> u32 { // 19A RES 3,D
    let mut t: u16 = cpu.d & !(1 << 3);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19B(cpu: &CPU, memory: &Memory) -> u32 { // 19B RES 3,E
    let mut t: u16 = cpu.e & !(1 << 3);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19C(cpu: &CPU, memory: &Memory) -> u32 { // 19C RES 3,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 3);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19D(cpu: &CPU, memory: &Memory) -> u32 { // 19D RES 3,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 3);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19E(cpu: &CPU, memory: &Memory) -> u32 { // 19E RES 3,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 3);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_19F(cpu: &CPU, memory: &Memory) -> u32 { // 19F RES 3,A
    let mut t: u16 = cpu.a & !(1 << 3);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A0(cpu: &CPU, memory: &Memory) -> u32 { // 1A0 RES 4,B
    let mut t: u16 = cpu.b & !(1 << 4);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A1(cpu: &CPU, memory: &Memory) -> u32 { // 1A1 RES 4,C
    let mut t: u16 = cpu.c & !(1 << 4);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A2(cpu: &CPU, memory: &Memory) -> u32 { // 1A2 RES 4,D
    let mut t: u16 = cpu.d & !(1 << 4);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A3(cpu: &CPU, memory: &Memory) -> u32 { // 1A3 RES 4,E
    let mut t: u16 = cpu.e & !(1 << 4);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A4(cpu: &CPU, memory: &Memory) -> u32 { // 1A4 RES 4,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 4);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A5(cpu: &CPU, memory: &Memory) -> u32 { // 1A5 RES 4,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 4);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A6(cpu: &CPU, memory: &Memory) -> u32 { // 1A6 RES 4,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 4);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1A7(cpu: &CPU, memory: &Memory) -> u32 { // 1A7 RES 4,A
    let mut t: u16 = cpu.a & !(1 << 4);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A8(cpu: &CPU, memory: &Memory) -> u32 { // 1A8 RES 5,B
    let mut t: u16 = cpu.b & !(1 << 5);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A9(cpu: &CPU, memory: &Memory) -> u32 { // 1A9 RES 5,C
    let mut t: u16 = cpu.c & !(1 << 5);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AA(cpu: &CPU, memory: &Memory) -> u32 { // 1AA RES 5,D
    let mut t: u16 = cpu.d & !(1 << 5);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AB(cpu: &CPU, memory: &Memory) -> u32 { // 1AB RES 5,E
    let mut t: u16 = cpu.e & !(1 << 5);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AC(cpu: &CPU, memory: &Memory) -> u32 { // 1AC RES 5,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 5);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AD(cpu: &CPU, memory: &Memory) -> u32 { // 1AD RES 5,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 5);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AE(cpu: &CPU, memory: &Memory) -> u32 { // 1AE RES 5,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 5);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1AF(cpu: &CPU, memory: &Memory) -> u32 { // 1AF RES 5,A
    let mut t: u16 = cpu.a & !(1 << 5);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B0(cpu: &CPU, memory: &Memory) -> u32 { // 1B0 RES 6,B
    let mut t: u16 = cpu.b & !(1 << 6);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B1(cpu: &CPU, memory: &Memory) -> u32 { // 1B1 RES 6,C
    let mut t: u16 = cpu.c & !(1 << 6);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B2(cpu: &CPU, memory: &Memory) -> u32 { // 1B2 RES 6,D
    let mut t: u16 = cpu.d & !(1 << 6);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B3(cpu: &CPU, memory: &Memory) -> u32 { // 1B3 RES 6,E
    let mut t: u16 = cpu.e & !(1 << 6);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B4(cpu: &CPU, memory: &Memory) -> u32 { // 1B4 RES 6,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 6);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B5(cpu: &CPU, memory: &Memory) -> u32 { // 1B5 RES 6,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 6);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B6(cpu: &CPU, memory: &Memory) -> u32 { // 1B6 RES 6,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 6);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1B7(cpu: &CPU, memory: &Memory) -> u32 { // 1B7 RES 6,A
    let mut t: u16 = cpu.a & !(1 << 6);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B8(cpu: &CPU, memory: &Memory) -> u32 { // 1B8 RES 7,B
    let mut t: u16 = cpu.b & !(1 << 7);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B9(cpu: &CPU, memory: &Memory) -> u32 { // 1B9 RES 7,C
    let mut t: u16 = cpu.c & !(1 << 7);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BA(cpu: &CPU, memory: &Memory) -> u32 { // 1BA RES 7,D
    let mut t: u16 = cpu.d & !(1 << 7);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BB(cpu: &CPU, memory: &Memory) -> u32 { // 1BB RES 7,E
    let mut t: u16 = cpu.e & !(1 << 7);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BC(cpu: &CPU, memory: &Memory) -> u32 { // 1BC RES 7,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 7);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BD(cpu: &CPU, memory: &Memory) -> u32 { // 1BD RES 7,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 7);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BE(cpu: &CPU, memory: &Memory) -> u32 { // 1BE RES 7,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) & !(1 << 7);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1BF(cpu: &CPU, memory: &Memory) -> u32 { // 1BF RES 7,A
    let mut t: u16 = cpu.a & !(1 << 7);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C0(cpu: &CPU, memory: &Memory) -> u32 { // 1C0 SET 0,B
    let mut t: u16 = cpu.b | (1 << 0);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C1(cpu: &CPU, memory: &Memory) -> u32 { // 1C1 SET 0,C
    let mut t: u16 = cpu.c | (1 << 0);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C2(cpu: &CPU, memory: &Memory) -> u32 { // 1C2 SET 0,D
    let mut t: u16 = cpu.d | (1 << 0);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C3(cpu: &CPU, memory: &Memory) -> u32 { // 1C3 SET 0,E
    let mut t: u16 = cpu.e | (1 << 0);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C4(cpu: &CPU, memory: &Memory) -> u32 { // 1C4 SET 0,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 0);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C5(cpu: &CPU, memory: &Memory) -> u32 { // 1C5 SET 0,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 0);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C6(cpu: &CPU, memory: &Memory) -> u32 { // 1C6 SET 0,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 0);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1C7(cpu: &CPU, memory: &Memory) -> u32 { // 1C7 SET 0,A
    let mut t: u16 = cpu.a | (1 << 0);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C8(cpu: &CPU, memory: &Memory) -> u32 { // 1C8 SET 1,B
    let mut t: u16 = cpu.b | (1 << 1);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C9(cpu: &CPU, memory: &Memory) -> u32 { // 1C9 SET 1,C
    let mut t: u16 = cpu.c | (1 << 1);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CA(cpu: &CPU, memory: &Memory) -> u32 { // 1CA SET 1,D
    let mut t: u16 = cpu.d | (1 << 1);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CB(cpu: &CPU, memory: &Memory) -> u32 { // 1CB SET 1,E
    let mut t: u16 = cpu.e | (1 << 1);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CC(cpu: &CPU, memory: &Memory) -> u32 { // 1CC SET 1,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 1);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CD(cpu: &CPU, memory: &Memory) -> u32 { // 1CD SET 1,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 1);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CE(cpu: &CPU, memory: &Memory) -> u32 { // 1CE SET 1,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 1);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1CF(cpu: &CPU, memory: &Memory) -> u32 { // 1CF SET 1,A
    let mut t: u16 = cpu.a | (1 << 1);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D0(cpu: &CPU, memory: &Memory) -> u32 { // 1D0 SET 2,B
    let mut t: u16 = cpu.b | (1 << 2);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D1(cpu: &CPU, memory: &Memory) -> u32 { // 1D1 SET 2,C
    let mut t: u16 = cpu.c | (1 << 2);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D2(cpu: &CPU, memory: &Memory) -> u32 { // 1D2 SET 2,D
    let mut t: u16 = cpu.d | (1 << 2);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D3(cpu: &CPU, memory: &Memory) -> u32 { // 1D3 SET 2,E
    let mut t: u16 = cpu.e | (1 << 2);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D4(cpu: &CPU, memory: &Memory) -> u32 { // 1D4 SET 2,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 2);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D5(cpu: &CPU, memory: &Memory) -> u32 { // 1D5 SET 2,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 2);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D6(cpu: &CPU, memory: &Memory) -> u32 { // 1D6 SET 2,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 2);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1D7(cpu: &CPU, memory: &Memory) -> u32 { // 1D7 SET 2,A
    let mut t: u16 = cpu.a | (1 << 2);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D8(cpu: &CPU, memory: &Memory) -> u32 { // 1D8 SET 3,B
    let mut t: u16 = cpu.b | (1 << 3);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D9(cpu: &CPU, memory: &Memory) -> u32 { // 1D9 SET 3,C
    let mut t: u16 = cpu.c | (1 << 3);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DA(cpu: &CPU, memory: &Memory) -> u32 { // 1DA SET 3,D
    let mut t: u16 = cpu.d | (1 << 3);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DB(cpu: &CPU, memory: &Memory) -> u32 { // 1DB SET 3,E
    let mut t: u16 = cpu.e | (1 << 3);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DC(cpu: &CPU, memory: &Memory) -> u32 { // 1DC SET 3,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 3);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DD(cpu: &CPU, memory: &Memory) -> u32 { // 1DD SET 3,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 3);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DE(cpu: &CPU, memory: &Memory) -> u32 { // 1DE SET 3,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 3);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1DF(cpu: &CPU, memory: &Memory) -> u32 { // 1DF SET 3,A
    let mut t: u16 = cpu.a | (1 << 3);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E0(cpu: &CPU, memory: &Memory) -> u32 { // 1E0 SET 4,B
    let mut t: u16 = cpu.b | (1 << 4);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E1(cpu: &CPU, memory: &Memory) -> u32 { // 1E1 SET 4,C
    let mut t: u16 = cpu.c | (1 << 4);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E2(cpu: &CPU, memory: &Memory) -> u32 { // 1E2 SET 4,D
    let mut t: u16 = cpu.d | (1 << 4);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E3(cpu: &CPU, memory: &Memory) -> u32 { // 1E3 SET 4,E
    let mut t: u16 = cpu.e | (1 << 4);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E4(cpu: &CPU, memory: &Memory) -> u32 { // 1E4 SET 4,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 4);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E5(cpu: &CPU, memory: &Memory) -> u32 { // 1E5 SET 4,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 4);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E6(cpu: &CPU, memory: &Memory) -> u32 { // 1E6 SET 4,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 4);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1E7(cpu: &CPU, memory: &Memory) -> u32 { // 1E7 SET 4,A
    let mut t: u16 = cpu.a | (1 << 4);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E8(cpu: &CPU, memory: &Memory) -> u32 { // 1E8 SET 5,B
    let mut t: u16 = cpu.b | (1 << 5);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E9(cpu: &CPU, memory: &Memory) -> u32 { // 1E9 SET 5,C
    let mut t: u16 = cpu.c | (1 << 5);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EA(cpu: &CPU, memory: &Memory) -> u32 { // 1EA SET 5,D
    let mut t: u16 = cpu.d | (1 << 5);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EB(cpu: &CPU, memory: &Memory) -> u32 { // 1EB SET 5,E
    let mut t: u16 = cpu.e | (1 << 5);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EC(cpu: &CPU, memory: &Memory) -> u32 { // 1EC SET 5,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 5);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1ED(cpu: &CPU, memory: &Memory) -> u32 { // 1ED SET 5,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 5);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EE(cpu: &CPU, memory: &Memory) -> u32 { // 1EE SET 5,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 5);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1EF(cpu: &CPU, memory: &Memory) -> u32 { // 1EF SET 5,A
    let mut t: u16 = cpu.a | (1 << 5);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F0(cpu: &CPU, memory: &Memory) -> u32 { // 1F0 SET 6,B
    let mut t: u16 = cpu.b | (1 << 6);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F1(cpu: &CPU, memory: &Memory) -> u32 { // 1F1 SET 6,C
    let mut t: u16 = cpu.c | (1 << 6);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F2(cpu: &CPU, memory: &Memory) -> u32 { // 1F2 SET 6,D
    let mut t: u16 = cpu.d | (1 << 6);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F3(cpu: &CPU, memory: &Memory) -> u32 { // 1F3 SET 6,E
    let mut t: u16 = cpu.e | (1 << 6);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F4(cpu: &CPU, memory: &Memory) -> u32 { // 1F4 SET 6,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 6);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F5(cpu: &CPU, memory: &Memory) -> u32 { // 1F5 SET 6,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 6);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F6(cpu: &CPU, memory: &Memory) -> u32 { // 1F6 SET 6,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 6);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1F7(cpu: &CPU, memory: &Memory) -> u32 { // 1F7 SET 6,A
    let mut t: u16 = cpu.a | (1 << 6);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F8(cpu: &CPU, memory: &Memory) -> u32 { // 1F8 SET 7,B
    let mut t: u16 = cpu.b | (1 << 7);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F9(cpu: &CPU, memory: &Memory) -> u32 { // 1F9 SET 7,C
    let mut t: u16 = cpu.c | (1 << 7);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FA(cpu: &CPU, memory: &Memory) -> u32 { // 1FA SET 7,D
    let mut t: u16 = cpu.d | (1 << 7);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FB(cpu: &CPU, memory: &Memory) -> u32 { // 1FB SET 7,E
    let mut t: u16 = cpu.e | (1 << 7);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FC(cpu: &CPU, memory: &Memory) -> u32 { // 1FC SET 7,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 7);
    cpu.set_hl() =(cpu.get_hl() & 0x00FF) | (t << 8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FD(cpu: &CPU, memory: &Memory) -> u32 { // 1FD SET 7,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 7);
    cpu.set_hl() =(cpu.get_hl() & 0xFF00) | (t & 0xFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FE(cpu: &CPU, memory: &Memory) -> u32 { // 1FE SET 7,(HL)
    let mut t: u16 = memory.read8(cpu.get_hl()) | (1 << 7);
    memory.write8(cpu.get_hl(), t);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1FF(cpu: &CPU, memory: &Memory) -> u32 { // 1FF SET 7,A
    let mut t: u16 = cpu.a | (1 << 7);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;