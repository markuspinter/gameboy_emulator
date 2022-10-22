use super::{Register8, Register16, CPU};
use crate::gameboy::memory::Memory;
use crate::gameboy::MemoryInterface;
use super::{FLAGC, FLAGH, FLAGN, FLAGZ};

fn no_opcode(cpu: &mut CPU, memory: &mut Memory) -> u32 {
    return 0;
}

fn opcode_length(opcode: u16) -> u8 {
    return OPCODE_LENGTHS[usize::from(opcode)];
}

pub fn execute_opcode(opcode: u16, cpu: &mut CPU, memory: &mut Memory) -> u32 {
    println!("executing opcode: {:#06X}", opcode);
    let oplen = opcode_length(opcode);
    let mut v: u16 = 0;
    let pc = cpu.get_reg16(Register16::PC);
    if oplen == 2 {
        // 8-bit immediate
        v = memory.read8(pc+1).unwrap() as u16;
    } else if oplen == 3 {
        // 16-bit immediate
        // Flips order of values due to big-endian
        v = memory.read16(pc+1).unwrap();
    }
    if opcode == 0x00 {
        return NOP_00(cpu, memory);
    } else if opcode == 0x01 {
        return LD_01(cpu, memory, v);
    } else if opcode == 0x02 {
        return LD_02(cpu, memory);
    } else if opcode == 0x03 {
        return INC_03(cpu, memory);
    } else if opcode == 0x04 {
        return INC_04(cpu, memory);
    } else if opcode == 0x05 {
        return DEC_05(cpu, memory);
    } else if opcode == 0x06 {
        return LD_06(cpu, memory, v);
    } else if opcode == 0x07 {
        return RLCA_07(cpu, memory);
    } else if opcode == 0x08 {
        return LD_08(cpu, memory, v);
    } else if opcode == 0x09 {
        return ADD_09(cpu, memory);
    } else if opcode == 0x0A {
        return LD_0A(cpu, memory);
    } else if opcode == 0x0B {
        return DEC_0B(cpu, memory);
    } else if opcode == 0x0C {
        return INC_0C(cpu, memory);
    } else if opcode == 0x0D {
        return DEC_0D(cpu, memory);
    } else if opcode == 0x0E {
        return LD_0E(cpu, memory, v);
    } else if opcode == 0x0F {
        return RRCA_0F(cpu, memory);
    } else if opcode == 0x10 {
        return STOP_10(cpu, memory, v);
    } else if opcode == 0x11 {
        return LD_11(cpu, memory, v);
    } else if opcode == 0x12 {
        return LD_12(cpu, memory);
    } else if opcode == 0x13 {
        return INC_13(cpu, memory);
    } else if opcode == 0x14 {
        return INC_14(cpu, memory);
    } else if opcode == 0x15 {
        return DEC_15(cpu, memory);
    } else if opcode == 0x16 {
        return LD_16(cpu, memory, v);
    } else if opcode == 0x17 {
        return RLA_17(cpu, memory);
    } else if opcode == 0x18 {
        return JR_18(cpu, memory, v);
    } else if opcode == 0x19 {
        return ADD_19(cpu, memory);
    } else if opcode == 0x1A {
        return LD_1A(cpu, memory);
    } else if opcode == 0x1B {
        return DEC_1B(cpu, memory);
    } else if opcode == 0x1C {
        return INC_1C(cpu, memory);
    } else if opcode == 0x1D {
        return DEC_1D(cpu, memory);
    } else if opcode == 0x1E {
        return LD_1E(cpu, memory, v);
    } else if opcode == 0x1F {
        return RRA_1F(cpu, memory);
    } else if opcode == 0x20 {
        return JR_20(cpu, memory, v);
    } else if opcode == 0x21 {
        return LD_21(cpu, memory, v);
    } else if opcode == 0x22 {
        return LD_22(cpu, memory);
    } else if opcode == 0x23 {
        return INC_23(cpu, memory);
    } else if opcode == 0x24 {
        return INC_24(cpu, memory);
    } else if opcode == 0x25 {
        return DEC_25(cpu, memory);
    } else if opcode == 0x26 {
        return LD_26(cpu, memory, v);
    } else if opcode == 0x27 {
        return DAA_27(cpu, memory);
    } else if opcode == 0x28 {
        return JR_28(cpu, memory, v);
    } else if opcode == 0x29 {
        return ADD_29(cpu, memory);
    } else if opcode == 0x2A {
        return LD_2A(cpu, memory);
    } else if opcode == 0x2B {
        return DEC_2B(cpu, memory);
    } else if opcode == 0x2C {
        return INC_2C(cpu, memory);
    } else if opcode == 0x2D {
        return DEC_2D(cpu, memory);
    } else if opcode == 0x2E {
        return LD_2E(cpu, memory, v);
    } else if opcode == 0x2F {
        return CPL_2F(cpu, memory);
    } else if opcode == 0x30 {
        return JR_30(cpu, memory, v);
    } else if opcode == 0x31 {
        return LD_31(cpu, memory, v);
    } else if opcode == 0x32 {
        return LD_32(cpu, memory);
    } else if opcode == 0x33 {
        return INC_33(cpu, memory);
    } else if opcode == 0x34 {
        return INC_34(cpu, memory);
    } else if opcode == 0x35 {
        return DEC_35(cpu, memory);
    } else if opcode == 0x36 {
        return LD_36(cpu, memory, v);
    } else if opcode == 0x37 {
        return SCF_37(cpu, memory);
    } else if opcode == 0x38 {
        return JR_38(cpu, memory, v);
    } else if opcode == 0x39 {
        return ADD_39(cpu, memory);
    } else if opcode == 0x3A {
        return LD_3A(cpu, memory);
    } else if opcode == 0x3B {
        return DEC_3B(cpu, memory);
    } else if opcode == 0x3C {
        return INC_3C(cpu, memory);
    } else if opcode == 0x3D {
        return DEC_3D(cpu, memory);
    } else if opcode == 0x3E {
        return LD_3E(cpu, memory, v);
    } else if opcode == 0x3F {
        return CCF_3F(cpu, memory);
    } else if opcode == 0x40 {
        return LD_40(cpu, memory);
    } else if opcode == 0x41 {
        return LD_41(cpu, memory);
    } else if opcode == 0x42 {
        return LD_42(cpu, memory);
    } else if opcode == 0x43 {
        return LD_43(cpu, memory);
    } else if opcode == 0x44 {
        return LD_44(cpu, memory);
    } else if opcode == 0x45 {
        return LD_45(cpu, memory);
    } else if opcode == 0x46 {
        return LD_46(cpu, memory);
    } else if opcode == 0x47 {
        return LD_47(cpu, memory);
    } else if opcode == 0x48 {
        return LD_48(cpu, memory);
    } else if opcode == 0x49 {
        return LD_49(cpu, memory);
    } else if opcode == 0x4A {
        return LD_4A(cpu, memory);
    } else if opcode == 0x4B {
        return LD_4B(cpu, memory);
    } else if opcode == 0x4C {
        return LD_4C(cpu, memory);
    } else if opcode == 0x4D {
        return LD_4D(cpu, memory);
    } else if opcode == 0x4E {
        return LD_4E(cpu, memory);
    } else if opcode == 0x4F {
        return LD_4F(cpu, memory);
    } else if opcode == 0x50 {
        return LD_50(cpu, memory);
    } else if opcode == 0x51 {
        return LD_51(cpu, memory);
    } else if opcode == 0x52 {
        return LD_52(cpu, memory);
    } else if opcode == 0x53 {
        return LD_53(cpu, memory);
    } else if opcode == 0x54 {
        return LD_54(cpu, memory);
    } else if opcode == 0x55 {
        return LD_55(cpu, memory);
    } else if opcode == 0x56 {
        return LD_56(cpu, memory);
    } else if opcode == 0x57 {
        return LD_57(cpu, memory);
    } else if opcode == 0x58 {
        return LD_58(cpu, memory);
    } else if opcode == 0x59 {
        return LD_59(cpu, memory);
    } else if opcode == 0x5A {
        return LD_5A(cpu, memory);
    } else if opcode == 0x5B {
        return LD_5B(cpu, memory);
    } else if opcode == 0x5C {
        return LD_5C(cpu, memory);
    } else if opcode == 0x5D {
        return LD_5D(cpu, memory);
    } else if opcode == 0x5E {
        return LD_5E(cpu, memory);
    } else if opcode == 0x5F {
        return LD_5F(cpu, memory);
    } else if opcode == 0x60 {
        return LD_60(cpu, memory);
    } else if opcode == 0x61 {
        return LD_61(cpu, memory);
    } else if opcode == 0x62 {
        return LD_62(cpu, memory);
    } else if opcode == 0x63 {
        return LD_63(cpu, memory);
    } else if opcode == 0x64 {
        return LD_64(cpu, memory);
    } else if opcode == 0x65 {
        return LD_65(cpu, memory);
    } else if opcode == 0x66 {
        return LD_66(cpu, memory);
    } else if opcode == 0x67 {
        return LD_67(cpu, memory);
    } else if opcode == 0x68 {
        return LD_68(cpu, memory);
    } else if opcode == 0x69 {
        return LD_69(cpu, memory);
    } else if opcode == 0x6A {
        return LD_6A(cpu, memory);
    } else if opcode == 0x6B {
        return LD_6B(cpu, memory);
    } else if opcode == 0x6C {
        return LD_6C(cpu, memory);
    } else if opcode == 0x6D {
        return LD_6D(cpu, memory);
    } else if opcode == 0x6E {
        return LD_6E(cpu, memory);
    } else if opcode == 0x6F {
        return LD_6F(cpu, memory);
    } else if opcode == 0x70 {
        return LD_70(cpu, memory);
    } else if opcode == 0x71 {
        return LD_71(cpu, memory);
    } else if opcode == 0x72 {
        return LD_72(cpu, memory);
    } else if opcode == 0x73 {
        return LD_73(cpu, memory);
    } else if opcode == 0x74 {
        return LD_74(cpu, memory);
    } else if opcode == 0x75 {
        return LD_75(cpu, memory);
    } else if opcode == 0x76 {
        return HALT_76(cpu, memory);
    } else if opcode == 0x77 {
        return LD_77(cpu, memory);
    } else if opcode == 0x78 {
        return LD_78(cpu, memory);
    } else if opcode == 0x79 {
        return LD_79(cpu, memory);
    } else if opcode == 0x7A {
        return LD_7A(cpu, memory);
    } else if opcode == 0x7B {
        return LD_7B(cpu, memory);
    } else if opcode == 0x7C {
        return LD_7C(cpu, memory);
    } else if opcode == 0x7D {
        return LD_7D(cpu, memory);
    } else if opcode == 0x7E {
        return LD_7E(cpu, memory);
    } else if opcode == 0x7F {
        return LD_7F(cpu, memory);
    } else if opcode == 0x80 {
        return ADD_80(cpu, memory);
    } else if opcode == 0x81 {
        return ADD_81(cpu, memory);
    } else if opcode == 0x82 {
        return ADD_82(cpu, memory);
    } else if opcode == 0x83 {
        return ADD_83(cpu, memory);
    } else if opcode == 0x84 {
        return ADD_84(cpu, memory);
    } else if opcode == 0x85 {
        return ADD_85(cpu, memory);
    } else if opcode == 0x86 {
        return ADD_86(cpu, memory);
    } else if opcode == 0x87 {
        return ADD_87(cpu, memory);
    } else if opcode == 0x88 {
        return ADC_88(cpu, memory);
    } else if opcode == 0x89 {
        return ADC_89(cpu, memory);
    } else if opcode == 0x8A {
        return ADC_8A(cpu, memory);
    } else if opcode == 0x8B {
        return ADC_8B(cpu, memory);
    } else if opcode == 0x8C {
        return ADC_8C(cpu, memory);
    } else if opcode == 0x8D {
        return ADC_8D(cpu, memory);
    } else if opcode == 0x8E {
        return ADC_8E(cpu, memory);
    } else if opcode == 0x8F {
        return ADC_8F(cpu, memory);
    } else if opcode == 0x90 {
        return SUB_90(cpu, memory);
    } else if opcode == 0x91 {
        return SUB_91(cpu, memory);
    } else if opcode == 0x92 {
        return SUB_92(cpu, memory);
    } else if opcode == 0x93 {
        return SUB_93(cpu, memory);
    } else if opcode == 0x94 {
        return SUB_94(cpu, memory);
    } else if opcode == 0x95 {
        return SUB_95(cpu, memory);
    } else if opcode == 0x96 {
        return SUB_96(cpu, memory);
    } else if opcode == 0x97 {
        return SUB_97(cpu, memory);
    } else if opcode == 0x98 {
        return SBC_98(cpu, memory);
    } else if opcode == 0x99 {
        return SBC_99(cpu, memory);
    } else if opcode == 0x9A {
        return SBC_9A(cpu, memory);
    } else if opcode == 0x9B {
        return SBC_9B(cpu, memory);
    } else if opcode == 0x9C {
        return SBC_9C(cpu, memory);
    } else if opcode == 0x9D {
        return SBC_9D(cpu, memory);
    } else if opcode == 0x9E {
        return SBC_9E(cpu, memory);
    } else if opcode == 0x9F {
        return SBC_9F(cpu, memory);
    } else if opcode == 0xA0 {
        return AND_A0(cpu, memory);
    } else if opcode == 0xA1 {
        return AND_A1(cpu, memory);
    } else if opcode == 0xA2 {
        return AND_A2(cpu, memory);
    } else if opcode == 0xA3 {
        return AND_A3(cpu, memory);
    } else if opcode == 0xA4 {
        return AND_A4(cpu, memory);
    } else if opcode == 0xA5 {
        return AND_A5(cpu, memory);
    } else if opcode == 0xA6 {
        return AND_A6(cpu, memory);
    } else if opcode == 0xA7 {
        return AND_A7(cpu, memory);
    } else if opcode == 0xA8 {
        return XOR_A8(cpu, memory);
    } else if opcode == 0xA9 {
        return XOR_A9(cpu, memory);
    } else if opcode == 0xAA {
        return XOR_AA(cpu, memory);
    } else if opcode == 0xAB {
        return XOR_AB(cpu, memory);
    } else if opcode == 0xAC {
        return XOR_AC(cpu, memory);
    } else if opcode == 0xAD {
        return XOR_AD(cpu, memory);
    } else if opcode == 0xAE {
        return XOR_AE(cpu, memory);
    } else if opcode == 0xAF {
        return XOR_AF(cpu, memory);
    } else if opcode == 0xB0 {
        return OR_B0(cpu, memory);
    } else if opcode == 0xB1 {
        return OR_B1(cpu, memory);
    } else if opcode == 0xB2 {
        return OR_B2(cpu, memory);
    } else if opcode == 0xB3 {
        return OR_B3(cpu, memory);
    } else if opcode == 0xB4 {
        return OR_B4(cpu, memory);
    } else if opcode == 0xB5 {
        return OR_B5(cpu, memory);
    } else if opcode == 0xB6 {
        return OR_B6(cpu, memory);
    } else if opcode == 0xB7 {
        return OR_B7(cpu, memory);
    } else if opcode == 0xB8 {
        return CP_B8(cpu, memory);
    } else if opcode == 0xB9 {
        return CP_B9(cpu, memory);
    } else if opcode == 0xBA {
        return CP_BA(cpu, memory);
    } else if opcode == 0xBB {
        return CP_BB(cpu, memory);
    } else if opcode == 0xBC {
        return CP_BC(cpu, memory);
    } else if opcode == 0xBD {
        return CP_BD(cpu, memory);
    } else if opcode == 0xBE {
        return CP_BE(cpu, memory);
    } else if opcode == 0xBF {
        return CP_BF(cpu, memory);
    } else if opcode == 0xC0 {
        return RET_C0(cpu, memory);
    } else if opcode == 0xC1 {
        return POP_C1(cpu, memory);
    } else if opcode == 0xC2 {
        return JP_C2(cpu, memory, v);
    } else if opcode == 0xC3 {
        return JP_C3(cpu, memory, v);
    } else if opcode == 0xC4 {
        return CALL_C4(cpu, memory, v);
    } else if opcode == 0xC5 {
        return PUSH_C5(cpu, memory);
    } else if opcode == 0xC6 {
        return ADD_C6(cpu, memory, v);
    } else if opcode == 0xC7 {
        return RST_C7(cpu, memory);
    } else if opcode == 0xC8 {
        return RET_C8(cpu, memory);
    } else if opcode == 0xC9 {
        return RET_C9(cpu, memory);
    } else if opcode == 0xCA {
        return JP_CA(cpu, memory, v);
    } else if opcode == 0xCB {
        return PREFIX_CB(cpu, memory);
    } else if opcode == 0xCC {
        return CALL_CC(cpu, memory, v);
    } else if opcode == 0xCD {
        return CALL_CD(cpu, memory, v);
    } else if opcode == 0xCE {
        return ADC_CE(cpu, memory, v);
    } else if opcode == 0xCF {
        return RST_CF(cpu, memory);
    } else if opcode == 0xD0 {
        return RET_D0(cpu, memory);
    } else if opcode == 0xD1 {
        return POP_D1(cpu, memory);
    } else if opcode == 0xD2 {
        return JP_D2(cpu, memory, v);
    } else if opcode == 0xD3 {
        return no_opcode(cpu, memory);
    } else if opcode == 0xD4 {
        return CALL_D4(cpu, memory, v);
    } else if opcode == 0xD5 {
        return PUSH_D5(cpu, memory);
    } else if opcode == 0xD6 {
        return SUB_D6(cpu, memory, v);
    } else if opcode == 0xD7 {
        return RST_D7(cpu, memory);
    } else if opcode == 0xD8 {
        return RET_D8(cpu, memory);
    } else if opcode == 0xD9 {
        return RETI_D9(cpu, memory);
    } else if opcode == 0xDA {
        return JP_DA(cpu, memory, v);
    } else if opcode == 0xDB {
        return no_opcode(cpu, memory);
    } else if opcode == 0xDC {
        return CALL_DC(cpu, memory, v);
    } else if opcode == 0xDD {
        return no_opcode(cpu, memory);
    } else if opcode == 0xDE {
        return SBC_DE(cpu, memory, v);
    } else if opcode == 0xDF {
        return RST_DF(cpu, memory);
    } else if opcode == 0xE0 {
        return LDH_E0(cpu, memory, v);
    } else if opcode == 0xE1 {
        return POP_E1(cpu, memory);
    } else if opcode == 0xE2 {
        return LD_E2(cpu, memory);
    } else if opcode == 0xE3 {
        return no_opcode(cpu, memory);
    } else if opcode == 0xE4 {
        return no_opcode(cpu, memory);
    } else if opcode == 0xE5 {
        return PUSH_E5(cpu, memory);
    } else if opcode == 0xE6 {
        return AND_E6(cpu, memory, v);
    } else if opcode == 0xE7 {
        return RST_E7(cpu, memory);
    } else if opcode == 0xE8 {
        return ADD_E8(cpu, memory, v);
    } else if opcode == 0xE9 {
        return JP_E9(cpu, memory);
    } else if opcode == 0xEA {
        return LD_EA(cpu, memory, v);
    } else if opcode == 0xEB {
        return no_opcode(cpu, memory);
    } else if opcode == 0xEC {
        return no_opcode(cpu, memory);
    } else if opcode == 0xED {
        return no_opcode(cpu, memory);
    } else if opcode == 0xEE {
        return XOR_EE(cpu, memory, v);
    } else if opcode == 0xEF {
        return RST_EF(cpu, memory);
    } else if opcode == 0xF0 {
        return LDH_F0(cpu, memory, v);
    } else if opcode == 0xF1 {
        return POP_F1(cpu, memory);
    } else if opcode == 0xF2 {
        return LD_F2(cpu, memory);
    } else if opcode == 0xF3 {
        return DI_F3(cpu, memory);
    } else if opcode == 0xF4 {
        return no_opcode(cpu, memory);
    } else if opcode == 0xF5 {
        return PUSH_F5(cpu, memory);
    } else if opcode == 0xF6 {
        return OR_F6(cpu, memory, v);
    } else if opcode == 0xF7 {
        return RST_F7(cpu, memory);
    } else if opcode == 0xF8 {
        return LD_F8(cpu, memory, v);
    } else if opcode == 0xF9 {
        return LD_F9(cpu, memory);
    } else if opcode == 0xFA {
        return LD_FA(cpu, memory, v);
    } else if opcode == 0xFB {
        return EI_FB(cpu, memory);
    } else if opcode == 0xFC {
        return no_opcode(cpu, memory);
    } else if opcode == 0xFD {
        return no_opcode(cpu, memory);
    } else if opcode == 0xFE {
        return CP_FE(cpu, memory, v);
    } else if opcode == 0xFF {
        return RST_FF(cpu, memory);
    } else if opcode == 0x100 {
        return RLC_100(cpu, memory);
    } else if opcode == 0x101 {
        return RLC_101(cpu, memory);
    } else if opcode == 0x102 {
        return RLC_102(cpu, memory);
    } else if opcode == 0x103 {
        return RLC_103(cpu, memory);
    } else if opcode == 0x104 {
        return RLC_104(cpu, memory);
    } else if opcode == 0x105 {
        return RLC_105(cpu, memory);
    } else if opcode == 0x106 {
        return RLC_106(cpu, memory);
    } else if opcode == 0x107 {
        return RLC_107(cpu, memory);
    } else if opcode == 0x108 {
        return RRC_108(cpu, memory);
    } else if opcode == 0x109 {
        return RRC_109(cpu, memory);
    } else if opcode == 0x10A {
        return RRC_10A(cpu, memory);
    } else if opcode == 0x10B {
        return RRC_10B(cpu, memory);
    } else if opcode == 0x10C {
        return RRC_10C(cpu, memory);
    } else if opcode == 0x10D {
        return RRC_10D(cpu, memory);
    } else if opcode == 0x10E {
        return RRC_10E(cpu, memory);
    } else if opcode == 0x10F {
        return RRC_10F(cpu, memory);
    } else if opcode == 0x110 {
        return RL_110(cpu, memory);
    } else if opcode == 0x111 {
        return RL_111(cpu, memory);
    } else if opcode == 0x112 {
        return RL_112(cpu, memory);
    } else if opcode == 0x113 {
        return RL_113(cpu, memory);
    } else if opcode == 0x114 {
        return RL_114(cpu, memory);
    } else if opcode == 0x115 {
        return RL_115(cpu, memory);
    } else if opcode == 0x116 {
        return RL_116(cpu, memory);
    } else if opcode == 0x117 {
        return RL_117(cpu, memory);
    } else if opcode == 0x118 {
        return RR_118(cpu, memory);
    } else if opcode == 0x119 {
        return RR_119(cpu, memory);
    } else if opcode == 0x11A {
        return RR_11A(cpu, memory);
    } else if opcode == 0x11B {
        return RR_11B(cpu, memory);
    } else if opcode == 0x11C {
        return RR_11C(cpu, memory);
    } else if opcode == 0x11D {
        return RR_11D(cpu, memory);
    } else if opcode == 0x11E {
        return RR_11E(cpu, memory);
    } else if opcode == 0x11F {
        return RR_11F(cpu, memory);
    } else if opcode == 0x120 {
        return SLA_120(cpu, memory);
    } else if opcode == 0x121 {
        return SLA_121(cpu, memory);
    } else if opcode == 0x122 {
        return SLA_122(cpu, memory);
    } else if opcode == 0x123 {
        return SLA_123(cpu, memory);
    } else if opcode == 0x124 {
        return SLA_124(cpu, memory);
    } else if opcode == 0x125 {
        return SLA_125(cpu, memory);
    } else if opcode == 0x126 {
        return SLA_126(cpu, memory);
    } else if opcode == 0x127 {
        return SLA_127(cpu, memory);
    } else if opcode == 0x128 {
        return SRA_128(cpu, memory);
    } else if opcode == 0x129 {
        return SRA_129(cpu, memory);
    } else if opcode == 0x12A {
        return SRA_12A(cpu, memory);
    } else if opcode == 0x12B {
        return SRA_12B(cpu, memory);
    } else if opcode == 0x12C {
        return SRA_12C(cpu, memory);
    } else if opcode == 0x12D {
        return SRA_12D(cpu, memory);
    } else if opcode == 0x12E {
        return SRA_12E(cpu, memory);
    } else if opcode == 0x12F {
        return SRA_12F(cpu, memory);
    } else if opcode == 0x130 {
        return SWAP_130(cpu, memory);
    } else if opcode == 0x131 {
        return SWAP_131(cpu, memory);
    } else if opcode == 0x132 {
        return SWAP_132(cpu, memory);
    } else if opcode == 0x133 {
        return SWAP_133(cpu, memory);
    } else if opcode == 0x134 {
        return SWAP_134(cpu, memory);
    } else if opcode == 0x135 {
        return SWAP_135(cpu, memory);
    } else if opcode == 0x136 {
        return SWAP_136(cpu, memory);
    } else if opcode == 0x137 {
        return SWAP_137(cpu, memory);
    } else if opcode == 0x138 {
        return SRL_138(cpu, memory);
    } else if opcode == 0x139 {
        return SRL_139(cpu, memory);
    } else if opcode == 0x13A {
        return SRL_13A(cpu, memory);
    } else if opcode == 0x13B {
        return SRL_13B(cpu, memory);
    } else if opcode == 0x13C {
        return SRL_13C(cpu, memory);
    } else if opcode == 0x13D {
        return SRL_13D(cpu, memory);
    } else if opcode == 0x13E {
        return SRL_13E(cpu, memory);
    } else if opcode == 0x13F {
        return SRL_13F(cpu, memory);
    } else if opcode == 0x140 {
        return BIT_140(cpu, memory);
    } else if opcode == 0x141 {
        return BIT_141(cpu, memory);
    } else if opcode == 0x142 {
        return BIT_142(cpu, memory);
    } else if opcode == 0x143 {
        return BIT_143(cpu, memory);
    } else if opcode == 0x144 {
        return BIT_144(cpu, memory);
    } else if opcode == 0x145 {
        return BIT_145(cpu, memory);
    } else if opcode == 0x146 {
        return BIT_146(cpu, memory);
    } else if opcode == 0x147 {
        return BIT_147(cpu, memory);
    } else if opcode == 0x148 {
        return BIT_148(cpu, memory);
    } else if opcode == 0x149 {
        return BIT_149(cpu, memory);
    } else if opcode == 0x14A {
        return BIT_14A(cpu, memory);
    } else if opcode == 0x14B {
        return BIT_14B(cpu, memory);
    } else if opcode == 0x14C {
        return BIT_14C(cpu, memory);
    } else if opcode == 0x14D {
        return BIT_14D(cpu, memory);
    } else if opcode == 0x14E {
        return BIT_14E(cpu, memory);
    } else if opcode == 0x14F {
        return BIT_14F(cpu, memory);
    } else if opcode == 0x150 {
        return BIT_150(cpu, memory);
    } else if opcode == 0x151 {
        return BIT_151(cpu, memory);
    } else if opcode == 0x152 {
        return BIT_152(cpu, memory);
    } else if opcode == 0x153 {
        return BIT_153(cpu, memory);
    } else if opcode == 0x154 {
        return BIT_154(cpu, memory);
    } else if opcode == 0x155 {
        return BIT_155(cpu, memory);
    } else if opcode == 0x156 {
        return BIT_156(cpu, memory);
    } else if opcode == 0x157 {
        return BIT_157(cpu, memory);
    } else if opcode == 0x158 {
        return BIT_158(cpu, memory);
    } else if opcode == 0x159 {
        return BIT_159(cpu, memory);
    } else if opcode == 0x15A {
        return BIT_15A(cpu, memory);
    } else if opcode == 0x15B {
        return BIT_15B(cpu, memory);
    } else if opcode == 0x15C {
        return BIT_15C(cpu, memory);
    } else if opcode == 0x15D {
        return BIT_15D(cpu, memory);
    } else if opcode == 0x15E {
        return BIT_15E(cpu, memory);
    } else if opcode == 0x15F {
        return BIT_15F(cpu, memory);
    } else if opcode == 0x160 {
        return BIT_160(cpu, memory);
    } else if opcode == 0x161 {
        return BIT_161(cpu, memory);
    } else if opcode == 0x162 {
        return BIT_162(cpu, memory);
    } else if opcode == 0x163 {
        return BIT_163(cpu, memory);
    } else if opcode == 0x164 {
        return BIT_164(cpu, memory);
    } else if opcode == 0x165 {
        return BIT_165(cpu, memory);
    } else if opcode == 0x166 {
        return BIT_166(cpu, memory);
    } else if opcode == 0x167 {
        return BIT_167(cpu, memory);
    } else if opcode == 0x168 {
        return BIT_168(cpu, memory);
    } else if opcode == 0x169 {
        return BIT_169(cpu, memory);
    } else if opcode == 0x16A {
        return BIT_16A(cpu, memory);
    } else if opcode == 0x16B {
        return BIT_16B(cpu, memory);
    } else if opcode == 0x16C {
        return BIT_16C(cpu, memory);
    } else if opcode == 0x16D {
        return BIT_16D(cpu, memory);
    } else if opcode == 0x16E {
        return BIT_16E(cpu, memory);
    } else if opcode == 0x16F {
        return BIT_16F(cpu, memory);
    } else if opcode == 0x170 {
        return BIT_170(cpu, memory);
    } else if opcode == 0x171 {
        return BIT_171(cpu, memory);
    } else if opcode == 0x172 {
        return BIT_172(cpu, memory);
    } else if opcode == 0x173 {
        return BIT_173(cpu, memory);
    } else if opcode == 0x174 {
        return BIT_174(cpu, memory);
    } else if opcode == 0x175 {
        return BIT_175(cpu, memory);
    } else if opcode == 0x176 {
        return BIT_176(cpu, memory);
    } else if opcode == 0x177 {
        return BIT_177(cpu, memory);
    } else if opcode == 0x178 {
        return BIT_178(cpu, memory);
    } else if opcode == 0x179 {
        return BIT_179(cpu, memory);
    } else if opcode == 0x17A {
        return BIT_17A(cpu, memory);
    } else if opcode == 0x17B {
        return BIT_17B(cpu, memory);
    } else if opcode == 0x17C {
        return BIT_17C(cpu, memory);
    } else if opcode == 0x17D {
        return BIT_17D(cpu, memory);
    } else if opcode == 0x17E {
        return BIT_17E(cpu, memory);
    } else if opcode == 0x17F {
        return BIT_17F(cpu, memory);
    } else if opcode == 0x180 {
        return RES_180(cpu, memory);
    } else if opcode == 0x181 {
        return RES_181(cpu, memory);
    } else if opcode == 0x182 {
        return RES_182(cpu, memory);
    } else if opcode == 0x183 {
        return RES_183(cpu, memory);
    } else if opcode == 0x184 {
        return RES_184(cpu, memory);
    } else if opcode == 0x185 {
        return RES_185(cpu, memory);
    } else if opcode == 0x186 {
        return RES_186(cpu, memory);
    } else if opcode == 0x187 {
        return RES_187(cpu, memory);
    } else if opcode == 0x188 {
        return RES_188(cpu, memory);
    } else if opcode == 0x189 {
        return RES_189(cpu, memory);
    } else if opcode == 0x18A {
        return RES_18A(cpu, memory);
    } else if opcode == 0x18B {
        return RES_18B(cpu, memory);
    } else if opcode == 0x18C {
        return RES_18C(cpu, memory);
    } else if opcode == 0x18D {
        return RES_18D(cpu, memory);
    } else if opcode == 0x18E {
        return RES_18E(cpu, memory);
    } else if opcode == 0x18F {
        return RES_18F(cpu, memory);
    } else if opcode == 0x190 {
        return RES_190(cpu, memory);
    } else if opcode == 0x191 {
        return RES_191(cpu, memory);
    } else if opcode == 0x192 {
        return RES_192(cpu, memory);
    } else if opcode == 0x193 {
        return RES_193(cpu, memory);
    } else if opcode == 0x194 {
        return RES_194(cpu, memory);
    } else if opcode == 0x195 {
        return RES_195(cpu, memory);
    } else if opcode == 0x196 {
        return RES_196(cpu, memory);
    } else if opcode == 0x197 {
        return RES_197(cpu, memory);
    } else if opcode == 0x198 {
        return RES_198(cpu, memory);
    } else if opcode == 0x199 {
        return RES_199(cpu, memory);
    } else if opcode == 0x19A {
        return RES_19A(cpu, memory);
    } else if opcode == 0x19B {
        return RES_19B(cpu, memory);
    } else if opcode == 0x19C {
        return RES_19C(cpu, memory);
    } else if opcode == 0x19D {
        return RES_19D(cpu, memory);
    } else if opcode == 0x19E {
        return RES_19E(cpu, memory);
    } else if opcode == 0x19F {
        return RES_19F(cpu, memory);
    } else if opcode == 0x1A0 {
        return RES_1A0(cpu, memory);
    } else if opcode == 0x1A1 {
        return RES_1A1(cpu, memory);
    } else if opcode == 0x1A2 {
        return RES_1A2(cpu, memory);
    } else if opcode == 0x1A3 {
        return RES_1A3(cpu, memory);
    } else if opcode == 0x1A4 {
        return RES_1A4(cpu, memory);
    } else if opcode == 0x1A5 {
        return RES_1A5(cpu, memory);
    } else if opcode == 0x1A6 {
        return RES_1A6(cpu, memory);
    } else if opcode == 0x1A7 {
        return RES_1A7(cpu, memory);
    } else if opcode == 0x1A8 {
        return RES_1A8(cpu, memory);
    } else if opcode == 0x1A9 {
        return RES_1A9(cpu, memory);
    } else if opcode == 0x1AA {
        return RES_1AA(cpu, memory);
    } else if opcode == 0x1AB {
        return RES_1AB(cpu, memory);
    } else if opcode == 0x1AC {
        return RES_1AC(cpu, memory);
    } else if opcode == 0x1AD {
        return RES_1AD(cpu, memory);
    } else if opcode == 0x1AE {
        return RES_1AE(cpu, memory);
    } else if opcode == 0x1AF {
        return RES_1AF(cpu, memory);
    } else if opcode == 0x1B0 {
        return RES_1B0(cpu, memory);
    } else if opcode == 0x1B1 {
        return RES_1B1(cpu, memory);
    } else if opcode == 0x1B2 {
        return RES_1B2(cpu, memory);
    } else if opcode == 0x1B3 {
        return RES_1B3(cpu, memory);
    } else if opcode == 0x1B4 {
        return RES_1B4(cpu, memory);
    } else if opcode == 0x1B5 {
        return RES_1B5(cpu, memory);
    } else if opcode == 0x1B6 {
        return RES_1B6(cpu, memory);
    } else if opcode == 0x1B7 {
        return RES_1B7(cpu, memory);
    } else if opcode == 0x1B8 {
        return RES_1B8(cpu, memory);
    } else if opcode == 0x1B9 {
        return RES_1B9(cpu, memory);
    } else if opcode == 0x1BA {
        return RES_1BA(cpu, memory);
    } else if opcode == 0x1BB {
        return RES_1BB(cpu, memory);
    } else if opcode == 0x1BC {
        return RES_1BC(cpu, memory);
    } else if opcode == 0x1BD {
        return RES_1BD(cpu, memory);
    } else if opcode == 0x1BE {
        return RES_1BE(cpu, memory);
    } else if opcode == 0x1BF {
        return RES_1BF(cpu, memory);
    } else if opcode == 0x1C0 {
        return SET_1C0(cpu, memory);
    } else if opcode == 0x1C1 {
        return SET_1C1(cpu, memory);
    } else if opcode == 0x1C2 {
        return SET_1C2(cpu, memory);
    } else if opcode == 0x1C3 {
        return SET_1C3(cpu, memory);
    } else if opcode == 0x1C4 {
        return SET_1C4(cpu, memory);
    } else if opcode == 0x1C5 {
        return SET_1C5(cpu, memory);
    } else if opcode == 0x1C6 {
        return SET_1C6(cpu, memory);
    } else if opcode == 0x1C7 {
        return SET_1C7(cpu, memory);
    } else if opcode == 0x1C8 {
        return SET_1C8(cpu, memory);
    } else if opcode == 0x1C9 {
        return SET_1C9(cpu, memory);
    } else if opcode == 0x1CA {
        return SET_1CA(cpu, memory);
    } else if opcode == 0x1CB {
        return SET_1CB(cpu, memory);
    } else if opcode == 0x1CC {
        return SET_1CC(cpu, memory);
    } else if opcode == 0x1CD {
        return SET_1CD(cpu, memory);
    } else if opcode == 0x1CE {
        return SET_1CE(cpu, memory);
    } else if opcode == 0x1CF {
        return SET_1CF(cpu, memory);
    } else if opcode == 0x1D0 {
        return SET_1D0(cpu, memory);
    } else if opcode == 0x1D1 {
        return SET_1D1(cpu, memory);
    } else if opcode == 0x1D2 {
        return SET_1D2(cpu, memory);
    } else if opcode == 0x1D3 {
        return SET_1D3(cpu, memory);
    } else if opcode == 0x1D4 {
        return SET_1D4(cpu, memory);
    } else if opcode == 0x1D5 {
        return SET_1D5(cpu, memory);
    } else if opcode == 0x1D6 {
        return SET_1D6(cpu, memory);
    } else if opcode == 0x1D7 {
        return SET_1D7(cpu, memory);
    } else if opcode == 0x1D8 {
        return SET_1D8(cpu, memory);
    } else if opcode == 0x1D9 {
        return SET_1D9(cpu, memory);
    } else if opcode == 0x1DA {
        return SET_1DA(cpu, memory);
    } else if opcode == 0x1DB {
        return SET_1DB(cpu, memory);
    } else if opcode == 0x1DC {
        return SET_1DC(cpu, memory);
    } else if opcode == 0x1DD {
        return SET_1DD(cpu, memory);
    } else if opcode == 0x1DE {
        return SET_1DE(cpu, memory);
    } else if opcode == 0x1DF {
        return SET_1DF(cpu, memory);
    } else if opcode == 0x1E0 {
        return SET_1E0(cpu, memory);
    } else if opcode == 0x1E1 {
        return SET_1E1(cpu, memory);
    } else if opcode == 0x1E2 {
        return SET_1E2(cpu, memory);
    } else if opcode == 0x1E3 {
        return SET_1E3(cpu, memory);
    } else if opcode == 0x1E4 {
        return SET_1E4(cpu, memory);
    } else if opcode == 0x1E5 {
        return SET_1E5(cpu, memory);
    } else if opcode == 0x1E6 {
        return SET_1E6(cpu, memory);
    } else if opcode == 0x1E7 {
        return SET_1E7(cpu, memory);
    } else if opcode == 0x1E8 {
        return SET_1E8(cpu, memory);
    } else if opcode == 0x1E9 {
        return SET_1E9(cpu, memory);
    } else if opcode == 0x1EA {
        return SET_1EA(cpu, memory);
    } else if opcode == 0x1EB {
        return SET_1EB(cpu, memory);
    } else if opcode == 0x1EC {
        return SET_1EC(cpu, memory);
    } else if opcode == 0x1ED {
        return SET_1ED(cpu, memory);
    } else if opcode == 0x1EE {
        return SET_1EE(cpu, memory);
    } else if opcode == 0x1EF {
        return SET_1EF(cpu, memory);
    } else if opcode == 0x1F0 {
        return SET_1F0(cpu, memory);
    } else if opcode == 0x1F1 {
        return SET_1F1(cpu, memory);
    } else if opcode == 0x1F2 {
        return SET_1F2(cpu, memory);
    } else if opcode == 0x1F3 {
        return SET_1F3(cpu, memory);
    } else if opcode == 0x1F4 {
        return SET_1F4(cpu, memory);
    } else if opcode == 0x1F5 {
        return SET_1F5(cpu, memory);
    } else if opcode == 0x1F6 {
        return SET_1F6(cpu, memory);
    } else if opcode == 0x1F7 {
        return SET_1F7(cpu, memory);
    } else if opcode == 0x1F8 {
        return SET_1F8(cpu, memory);
    } else if opcode == 0x1F9 {
        return SET_1F9(cpu, memory);
    } else if opcode == 0x1FA {
        return SET_1FA(cpu, memory);
    } else if opcode == 0x1FB {
        return SET_1FB(cpu, memory);
    } else if opcode == 0x1FC {
        return SET_1FC(cpu, memory);
    } else if opcode == 0x1FD {
        return SET_1FD(cpu, memory);
    } else if opcode == 0x1FE {
        return SET_1FE(cpu, memory);
    } else if opcode == 0x1FF {
        return SET_1FF(cpu, memory);
    } else
    {
        return 0;
    }
}

fn NOP_00(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 00 NOP
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_01(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 01 LD BC,d16
    cpu.set_bc(v);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_02(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 02 LD (BC),A
    memory.write8(((cpu.b << 8) + cpu.c), (cpu.a) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_03(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 03 INC BC
    let mut t: u16 = ((cpu.b << 8) + cpu.c) + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_bc(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_04(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 04 INC B
    let mut t: u16 = cpu.b + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.b & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_05(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 05 DEC B
    let mut t: u16 = cpu.b - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.b & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_06(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 06 LD B,d8
    cpu.b = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLCA_07(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 07 RLCA
    let mut t: u16 = (cpu.a << 1) + (cpu.a >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_08(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 08 LD (a16),SP
    memory.write8(v, (cpu.sp & 0xFF) as u8);
    memory.write8(v+1, (cpu.sp >> 8) as u8);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 20;
}

fn ADD_09(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 09 ADD HL,BC
    let mut t: u16 = cpu.get_hl() + ((cpu.b << 8) + cpu.c);
    let mut flag: u16 = 0b00000000;
    flag += u16::from(((cpu.get_hl() & 0xFFF) + (((cpu.b << 8) + cpu.c) & 0xFFF)) > 0xFFF) << FLAGH;
    flag += u16::from(t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl( t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_0A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 0A LD A,(BC)
    cpu.a = memory.read8((cpu.b << 8) + cpu.c).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_0B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 0B DEC BC
    let mut t: u16 = ((cpu.b << 8) + cpu.c) - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_bc(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_0C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 0C INC C
    let mut t: u16 = cpu.c + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.c & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_0D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 0D DEC C
    let mut t: u16 = cpu.c - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.c & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_0E(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 0E LD C,d8
    cpu.c = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRCA_0F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 0F RRCA
    let mut t: u16 = (cpu.a >> 1) + ((cpu.a & 1) << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn STOP_10(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 10 STOP 0
    if memory.cgb_mode {
        memory.switch_speed();
        memory.write8(0xFF04, (0) as u8);
    }
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_11(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 11 LD DE,d16
    cpu.set_de(v);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_12(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 12 LD (DE),A
    memory.write8(((cpu.d << 8) + cpu.e), (cpu.a) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_13(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 13 INC DE
    let mut t: u16 = ((cpu.d << 8) + cpu.e) + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_de(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_14(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 14 INC D
    let mut t: u16 = cpu.d + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.d & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_15(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 15 DEC D
    let mut t: u16 = cpu.d - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.d & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_16(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 16 LD D,d8
    cpu.d = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLA_17(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 17 RLA
    let mut t: u16 = (cpu.a << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_18(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 18 JR r8
    cpu.pc += 2 + ((v ^ 0x80) - 0x80);
    cpu.pc &= 0xFFFF;
    return 12;
}

fn ADD_19(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 19 ADD HL,DE
    let mut t: u16 = cpu.get_hl() + ((cpu.d << 8) + cpu.e);
    let mut flag: u16 = 0b00000000;
    flag += u16::from(((cpu.get_hl() & 0xFFF) + (((cpu.d << 8) + cpu.e) & 0xFFF)) > 0xFFF) << FLAGH;
    flag += u16::from(t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl( t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_1A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A LD A,(DE)
    cpu.a = memory.read8((cpu.d << 8) + cpu.e).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_1B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B DEC DE
    let mut t: u16 = ((cpu.d << 8) + cpu.e) - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_de(t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_1C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C INC E
    let mut t: u16 = cpu.e + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.e & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_1D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D DEC E
    let mut t: u16 = cpu.e - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.e & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_1E(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 1E LD E,d8
    cpu.e = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRA_1F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F RRA
    let mut t: u16 = (cpu.a >> 1) + ((cpu.f_c() as u16) << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_20(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 20 JR NZ,r8
    cpu.pc += 2;
    if cpu.f_nz() {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn LD_21(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 21 LD HL,d16
    cpu.set_hl( v);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_22(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 22 LD (HL+),A
    memory.write8(cpu.get_hl(), (cpu.a) as u8);
    cpu.set_hl( cpu.get_hl() +1);
    cpu.set_hl( cpu.get_hl() & 0xFFFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_23(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 23 INC HL
    let mut t: u16 = cpu.get_hl() + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_hl( t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_24(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 24 INC H
    let mut t: u16 = (cpu.get_hl() >> 8) + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from((((cpu.get_hl() >> 8) & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_25(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 25 DEC H
    let mut t: u16 = (cpu.get_hl() >> 8) - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from((((cpu.get_hl() >> 8) & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_26(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 26 LD H,d8
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (v << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DAA_27(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 27 DAA
    let mut t: u16 = cpu.a;
    let mut corr: u16 = 0;
    corr |= if cpu.f_h() {0x06} else { 0x00 };
    corr |= if cpu.f_c() {0x60} else { 0x00 };
    if cpu.f_n() {
        t -= corr;
    } else {
        corr |= if (t & 0x0F) > 0x09 {0x06} else { 0x00 };
        corr |= if t > 0x99 {0x60} else { 0x00 };
        t += corr;
    }
    let mut flag: u16 = 0;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(corr & 0x60 != 0) << FLAGC;
    cpu.f &= 0b01000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_28(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 28 JR Z,r8
    cpu.pc += 2;
    if cpu.f_z() {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn ADD_29(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 29 ADD HL,HL
    let mut t: u16 = cpu.get_hl() + cpu.get_hl();
    let mut flag: u16 = 0b00000000;
    flag += u16::from(((cpu.get_hl() & 0xFFF) + (cpu.get_hl() & 0xFFF)) > 0xFFF) << FLAGH;
    flag += u16::from(t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl( t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_2A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 2A LD A,(HL+)
    cpu.a = memory.read8(cpu.get_hl()).unwrap() as u16;
    cpu.set_hl( cpu.get_hl() +1);
    cpu.set_hl( cpu.get_hl() & 0xFFFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_2B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 2B DEC HL
    let mut t: u16 = cpu.get_hl() - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.set_hl( t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_2C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 2C INC L
    let mut t: u16 = (cpu.get_hl() & 0xFF) + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from((((cpu.get_hl() & 0xFF) & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_2D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 2D DEC L
    let mut t: u16 = (cpu.get_hl() & 0xFF) - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from((((cpu.get_hl() & 0xFF) & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_2E(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 2E LD L,d8
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (v & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn CPL_2F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 2F CPL
    cpu.a = (!cpu.a) & 0xFF;
    let mut flag: u16 = 0b01100000;
    cpu.f &= 0b10010000;
    cpu.f |= flag;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_30(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 30 JR NC,r8
    cpu.pc += 2;
    if cpu.f_nc() {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn LD_31(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 31 LD SP,d16
    cpu.sp = v;
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_32(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 32 LD (HL-),A
    memory.write8(cpu.get_hl(), (cpu.a) as u8);
    cpu.set_hl( cpu.get_hl() - 1);
    cpu.set_hl( cpu.get_hl() & 0xFFFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_33(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 33 INC SP
    let mut t: u16 = cpu.sp + 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.sp = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_34(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 34 INC (HL)
    let mut t: u16 = memory.read8(cpu.get_hl()).unwrap() as u16 + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF) + (1 & 0xF)) > 0xF)) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn DEC_35(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 35 DEC (HL)
    let mut t: u16 = memory.read8(cpu.get_hl()).unwrap() as u16 - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF) - (1 & 0xF)) > 0)) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_36(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 36 LD (HL),d8
    memory.write8(cpu.get_hl(), (v) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn SCF_37(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 37 SCF
    let mut flag: u16 = 0b00010000;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn JR_38(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 38 JR C,r8
    cpu.pc += 2;
    if cpu.f_c() {
        cpu.pc += ((v ^ 0x80) - 0x80);
        cpu.pc &= 0xFFFF;
        return 12;
    } else {
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn ADD_39(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 39 ADD HL,SP
    let mut t: u16 = cpu.get_hl() + cpu.sp;
    let mut flag: u16 = 0b00000000;
    flag += u16::from(((cpu.get_hl() & 0xFFF) + (cpu.sp & 0xFFF)) > 0xFFF) << FLAGH;
    flag += u16::from(t > 0xFFFF) << FLAGC;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.set_hl( t);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_3A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 3A LD A,(HL-)
    cpu.a = memory.read8(cpu.get_hl()).unwrap() as u16;
    cpu.set_hl( cpu.get_hl() - 1);
    cpu.set_hl( cpu.get_hl() &  0xFFFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DEC_3B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 3B DEC SP
    let mut t: u16 = cpu.sp - 1;
    // No flag operations;
    t &= 0xFFFF;
    cpu.sp = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn INC_3C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 3C INC A
    let mut t: u16 = cpu.a + 1;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (1 & 0xF)) > 0xF) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn DEC_3D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 3D DEC A
    let mut t: u16 = cpu.a - 1;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (1 & 0xF)) < 0) << FLAGH;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_3E(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // 3E LD A,d8
    cpu.a = v;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn CCF_3F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 3F CCF
    let mut flag: u16 = (cpu.f & 0b00010000) ^ 0b00010000;
    cpu.f &= 0b10000000;
    cpu.f |= flag;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_40(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 40 LD B,B
    cpu.b = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_41(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 41 LD B,C
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_42(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 42 LD B,D
    cpu.b = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_43(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 43 LD B,E
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_44(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 44 LD B,H
    cpu.b = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_45(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 45 LD B,L
    cpu.b = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_46(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 46 LD B,(HL)
    cpu.b = memory.read8(cpu.get_hl()).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_47(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 47 LD B,A
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_48(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 48 LD C,B
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_49(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 49 LD C,C
    cpu.c = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 4A LD C,D
    cpu.c = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 4B LD C,E
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 4C LD C,H
    cpu.c = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 4D LD C,L
    cpu.c = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_4E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 4E LD C,(HL)
    cpu.c = memory.read8(cpu.get_hl()).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_4F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 4F LD C,A
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_50(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 50 LD D,B
    cpu.d = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_51(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 51 LD D,C
    cpu.d = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_52(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 52 LD D,D
    cpu.d = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_53(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 53 LD D,E
    cpu.d = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_54(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 54 LD D,H
    cpu.d = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_55(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 55 LD D,L
    cpu.d = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_56(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 56 LD D,(HL)
    cpu.d = memory.read8(cpu.get_hl()).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_57(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 57 LD D,A
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_58(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 58 LD E,B
    cpu.e = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_59(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 59 LD E,C
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 5A LD E,D
    cpu.e = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 5B LD E,E
    cpu.e = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 5C LD E,H
    cpu.e = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 5D LD E,L
    cpu.e = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_5E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 5E LD E,(HL)
    cpu.e = memory.read8(cpu.get_hl()).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_5F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 5F LD E,A
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_60(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 60 LD H,B
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (cpu.b << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_61(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 61 LD H,C
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (cpu.c << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_62(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 62 LD H,D
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (cpu.d << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_63(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 63 LD H,E
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (cpu.e << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_64(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 64 LD H,H
    cpu.set_hl((cpu.get_hl() & 0x00FF) | ((cpu.get_hl() >> 8) << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_65(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 65 LD H,L
    cpu.set_hl((cpu.get_hl() & 0x00FF) | ((cpu.get_hl() & 0xFF) << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_66(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 66 LD H,(HL)
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (memory.read8(cpu.get_hl()).unwrap() as u16) << 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_67(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 67 LD H,A
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (cpu.a << 8));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_68(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 68 LD L,B
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (cpu.b & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_69(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 69 LD L,C
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (cpu.c & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 6A LD L,D
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (cpu.d & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 6B LD L,E
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (cpu.e & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 6C LD L,H
    cpu.set_hl((cpu.get_hl() & 0xFF00) | ((cpu.get_hl() >> 8) & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 6D LD L,L
    cpu.set_hl((cpu.get_hl() & 0xFF00) | ((cpu.get_hl() & 0xFF) & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_6E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 6E LD L,(HL)
    cpu.set_hl((cpu.get_hl() & 0xFF00) | ((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_6F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 6F LD L,A
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (cpu.a & 0xFF));
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_70(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 70 LD (HL),B
    memory.write8(cpu.get_hl(), (cpu.b) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_71(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 71 LD (HL),C
    memory.write8(cpu.get_hl(), (cpu.c) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_72(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 72 LD (HL),D
    memory.write8(cpu.get_hl(), (cpu.d) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_73(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 73 LD (HL),E
    memory.write8(cpu.get_hl(), (cpu.e) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_74(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 74 LD (HL),H
    memory.write8(cpu.get_hl(), ((cpu.get_hl() >> 8)) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_75(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 75 LD (HL),L
    memory.write8(cpu.get_hl(), ((cpu.get_hl() & 0xFF)) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn HALT_76(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 76 HALT
    cpu.halted = true;
    return 4;
}

fn LD_77(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 77 LD (HL),A
    memory.write8(cpu.get_hl(), (cpu.a) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_78(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 78 LD A,B
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_79(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 79 LD A,C
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 7A LD A,D
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 7B LD A,E
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 7C LD A,H
    cpu.a = (cpu.get_hl() >> 8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 7D LD A,L
    cpu.a = (cpu.get_hl() & 0xFF);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn LD_7E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 7E LD A,(HL)
    cpu.a = memory.read8(cpu.get_hl()).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_7F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 7F LD A,A
    cpu.a = cpu.a;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_80(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 80 ADD A,B
    let mut t: u16 = cpu.a + cpu.b;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.b & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_81(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 81 ADD A,C
    let mut t: u16 = cpu.a + cpu.c;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.c & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_82(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 82 ADD A,D
    let mut t: u16 = cpu.a + cpu.d;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.d & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_83(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 83 ADD A,E
    let mut t: u16 = cpu.a + cpu.e;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.e & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_84(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 84 ADD A,H
    let mut t: u16 = cpu.a + (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + ((cpu.get_hl() >> 8) & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_85(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 85 ADD A,L
    let mut t: u16 = cpu.a + (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + ((cpu.get_hl() & 0xFF) & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADD_86(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 86 ADD A,(HL)
    let mut t: u16 = cpu.a + memory.read8(cpu.get_hl()).unwrap() as u16;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF)) > 0xF)) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn ADD_87(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 87 ADD A,A
    let mut t: u16 = cpu.a + cpu.a;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.a & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_88(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 88 ADC A,B
    let mut t: u16 = cpu.a + cpu.b + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.b & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_89(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 89 ADC A,C
    let mut t: u16 = cpu.a + cpu.c + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.c & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 8A ADC A,D
    let mut t: u16 = cpu.a + cpu.d + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.d & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 8B ADC A,E
    let mut t: u16 = cpu.a + cpu.e + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.e & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 8C ADC A,H
    let mut t: u16 = cpu.a + (cpu.get_hl() >> 8) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + ((cpu.get_hl() >> 8) & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 8D ADC A,L
    let mut t: u16 = cpu.a + (cpu.get_hl() & 0xFF) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + ((cpu.get_hl() & 0xFF) & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn ADC_8E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 8E ADC A,(HL)
    let mut t: u16 = cpu.a + (memory.read8(cpu.get_hl()).unwrap() as u16) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF) + u16::from(cpu.f_c())) > 0xF)) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn ADC_8F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 8F ADC A,A
    let mut t: u16 = cpu.a + cpu.a + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (cpu.a & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_90(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 90 SUB B
    let mut t: u16 = cpu.a - cpu.b;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.b & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_91(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 91 SUB C
    let mut t: u16 = cpu.a - cpu.c;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.c & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_92(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 92 SUB D
    let mut t: u16 = cpu.a - cpu.d;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.d & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_93(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 93 SUB E
    let mut t: u16 = cpu.a - cpu.e;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.e & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_94(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 94 SUB H
    let mut t: u16 = cpu.a - (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - ((cpu.get_hl() >> 8) & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_95(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 95 SUB L
    let mut t: u16 = cpu.a - (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - ((cpu.get_hl() & 0xFF) & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SUB_96(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 96 SUB (HL)
    let mut t: u16 = cpu.a - memory.read8(cpu.get_hl()).unwrap() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF)) > 0)) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SUB_97(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 97 SUB A
    let mut t: u16 = cpu.a - cpu.a;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.a & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_98(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 98 SBC A,B
    let mut t: u16 = cpu.a - cpu.b - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.b & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_99(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 99 SBC A,C
    let mut t: u16 = cpu.a - cpu.c - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.c & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 9A SBC A,D
    let mut t: u16 = cpu.a - cpu.d - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.d & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 9B SBC A,E
    let mut t: u16 = cpu.a - cpu.e - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.e & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 9C SBC A,H
    let mut t: u16 = cpu.a - (cpu.get_hl() >> 8) - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - ((cpu.get_hl() >> 8) & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 9D SBC A,L
    let mut t: u16 = cpu.a - (cpu.get_hl() & 0xFF) - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - ((cpu.get_hl() & 0xFF) & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn SBC_9E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 9E SBC A,(HL)
    let mut t: u16 = cpu.a - (memory.read8(cpu.get_hl()).unwrap() as u16) - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF) - cpu.f_c() as u16) > 0)) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SBC_9F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 9F SBC A,A
    let mut t: u16 = cpu.a - cpu.a - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.a & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A0 AND B
    let mut t: u16 = cpu.a & cpu.b;
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A1 AND C
    let mut t: u16 = cpu.a & cpu.c;
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A2 AND D
    let mut t: u16 = cpu.a & cpu.d;
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A3 AND E
    let mut t: u16 = cpu.a & cpu.e;
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A4 AND H
    let mut t: u16 = cpu.a & (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A5 AND L
    let mut t: u16 = cpu.a & (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn AND_A6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A6 AND (HL)
    let mut t: u16 = cpu.a & memory.read8(cpu.get_hl()).unwrap() as u16;
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn AND_A7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A7 AND A
    let mut t: u16 = cpu.a & cpu.a;
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_A8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A8 XOR B
    let mut t: u16 = cpu.a ^ cpu.b;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_A9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // A9 XOR C
    let mut t: u16 = cpu.a ^ cpu.c;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // AA XOR D
    let mut t: u16 = cpu.a ^ cpu.d;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // AB XOR E
    let mut t: u16 = cpu.a ^ cpu.e;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // AC XOR H
    let mut t: u16 = cpu.a ^ (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AD(cpu: &mut CPU, memory: &mut Memory) -> u32 { // AD XOR L
    let mut t: u16 = cpu.a ^ (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn XOR_AE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // AE XOR (HL)
    let mut t: u16 = cpu.a ^ memory.read8(cpu.get_hl()).unwrap() as u16;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn XOR_AF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // AF XOR A
    let mut t: u16 = cpu.a ^ cpu.a;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B0 OR B
    let mut t: u16 = cpu.a | cpu.b;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B1 OR C
    let mut t: u16 = cpu.a | cpu.c;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B2 OR D
    let mut t: u16 = cpu.a | cpu.d;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B3 OR E
    let mut t: u16 = cpu.a | cpu.e;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B4 OR H
    let mut t: u16 = cpu.a | (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B5 OR L
    let mut t: u16 = cpu.a | (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn OR_B6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B6 OR (HL)
    let mut t: u16 = cpu.a | memory.read8(cpu.get_hl()).unwrap() as u16;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn OR_B7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B7 OR A
    let mut t: u16 = cpu.a | cpu.a;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_B8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B8 CP B
    let mut t: u16 = cpu.a - cpu.b;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.b & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_B9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // B9 CP C
    let mut t: u16 = cpu.a - cpu.c;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.c & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // BA CP D
    let mut t: u16 = cpu.a - cpu.d;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.d & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // BB CP E
    let mut t: u16 = cpu.a - cpu.e;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.e & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // BC CP H
    let mut t: u16 = cpu.a - (cpu.get_hl() >> 8);
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - ((cpu.get_hl() >> 8) & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BD(cpu: &mut CPU, memory: &mut Memory) -> u32 { // BD CP L
    let mut t: u16 = cpu.a - (cpu.get_hl() & 0xFF);
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - ((cpu.get_hl() & 0xFF) & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_BE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // BE CP (HL)
    let mut t: u16 = cpu.a - memory.read8(cpu.get_hl()).unwrap() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF)) > 0)) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn CP_BF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // BF CP A
    let mut t: u16 = cpu.a - cpu.a;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (cpu.a & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn RET_C0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // C0 RET NZ
    if cpu.f_nz() {
        cpu.pc = (memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16) << 8; // High;
        cpu.pc |= memory.read8(cpu.sp).unwrap() as u16; // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn POP_C1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // C1 POP BC
    cpu.b = memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16; // High;
    cpu.c = memory.read8(cpu.sp).unwrap() as u16; // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn JP_C2(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // C2 JP NZ,a16
    if cpu.f_nz() {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn JP_C3(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // C3 JP a16
    cpu.pc = v;
    return 16;
}

fn CALL_C4(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // C4 CALL NZ,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_nz() {
        memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn PUSH_C5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // C5 PUSH BC
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.b) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.c) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn ADD_C6(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // C6 ADD A,d8
    let mut t: u16 = cpu.a + v;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (v & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_C7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // C7 RST 00H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 0;
    return 16;
}

fn RET_C8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // C8 RET Z
    if cpu.f_z() {
        cpu.pc = (memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16) << 8; // High;
        cpu.pc |= memory.read8(cpu.sp).unwrap() as u16; // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn RET_C9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // C9 RET
    cpu.pc = (memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16) << 8; // High;
    cpu.pc |= memory.read8(cpu.sp).unwrap() as u16; // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    return 16;
}

fn JP_CA(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // CA JP Z,a16
    if cpu.f_z() {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn PREFIX_CB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // CB PREFIX CB
    log::error!("CB cannot be called!");
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CALL_CC(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // CC CALL Z,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_z() {
        memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn CALL_CD(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // CD CALL a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = v;
    return 24;
}

fn ADC_CE(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // CE ADC A,d8
    let mut t: u16 = cpu.a + v + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) + (v & 0xF) + u16::from(cpu.f_c())) > 0xF) << FLAGH;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_CF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // CF RST 08H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 8;
    return 16;
}

fn RET_D0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // D0 RET NC
    if cpu.f_nc() {
        cpu.pc = (memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16) << 8; // High;
        cpu.pc |= memory.read8(cpu.sp).unwrap() as u16; // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn POP_D1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // D1 POP DE
    cpu.d = memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16; // High;
    cpu.e = memory.read8(cpu.sp).unwrap() as u16; // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn JP_D2(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // D2 JP NC,a16
    if cpu.f_nc() {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn CALL_D4(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // D4 CALL NC,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_nc() {
        memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn PUSH_D5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // D5 PUSH DE
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.d) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.e) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SUB_D6(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // D6 SUB d8
    let mut t: u16 = cpu.a - v;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (v & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_D7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // D7 RST 10H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 16;
    return 16;
}

fn RET_D8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // D8 RET C
    if cpu.f_c() {
        cpu.pc = (memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16) << 8; // High;
        cpu.pc |= memory.read8(cpu.sp).unwrap() as u16; // Low;
        cpu.sp += 2;
        cpu.sp &= 0xFFFF;
        return 20;
    } else {
        cpu.pc += 1;
        cpu.pc &= 0xFFFF;
        return 8;
    }
}

fn RETI_D9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // D9 RETI
    cpu.interrupt_master_enable = true;
    cpu.pc = (memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16) << 8; // High;
    cpu.pc |= memory.read8(cpu.sp).unwrap() as u16; // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    return 16;
}

fn JP_DA(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // DA JP C,a16
    if cpu.f_c() {
        cpu.pc = v;
        return 16;
    } else {
        cpu.pc += 3;
        cpu.pc &= 0xFFFF;
        return 12;
    }
}

fn CALL_DC(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // DC CALL C,a16
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    if cpu.f_c() {
        memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
        memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
        cpu.sp -= 2;
        cpu.sp &= 0xFFFF;
        cpu.pc = v;
        return 24;
    } else {
        return 12;
    }
}

fn SBC_DE(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // DE SBC A,d8
    let mut t: u16 = cpu.a - v - cpu.f_c() as u16;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (v & 0xF) - cpu.f_c() as u16) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_DF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // DF RST 18H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 24;
    return 16;
}

fn LDH_E0(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // E0 LDH (a8),A
    memory.write8(v + 0xFF00, (cpu.a) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn POP_E1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // E1 POP HL
    cpu.set_hl(((memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16) << 8) + memory.read8(cpu.sp).unwrap() as u16); // High);
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_E2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // E2 LD (C),A
    memory.write8(0xFF00 + cpu.c, (cpu.a) as u8);
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn PUSH_E5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // E5 PUSH HL
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.get_hl() >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.get_hl() & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn AND_E6(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // E6 AND d8
    let mut t: u16 = cpu.a & v;
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_E7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // E7 RST 20H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 32;
    return 16;
}

fn ADD_E8(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // E8 ADD SP,r8
    let mut t: u16 = cpu.sp + ((v ^ 0x80) - 0x80);
    let mut flag: u16 = 0b00000000;
    flag += u16::from(((cpu.sp & 0xF) + (v & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(((cpu.sp & 0xFF) + (v & 0xFF)) > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFFFF;
    cpu.sp = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn JP_E9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // E9 JP (HL)
    cpu.pc = cpu.get_hl();
    return 4;
}

fn LD_EA(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // EA LD (a16),A
    memory.write8(v, (cpu.a) as u8);
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn XOR_EE(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // EE XOR d8
    let mut t: u16 = cpu.a ^ v;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_EF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // EF RST 28H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 40;
    return 16;
}

fn LDH_F0(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // F0 LDH A,(a8)
    cpu.a = memory.read8(v + 0xFF00).unwrap() as u16;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn POP_F1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // F1 POP AF
    cpu.a = memory.read8((cpu.sp + 1) & 0xFFFF).unwrap() as u16; // High;
    cpu.f = memory.read8(cpu.sp).unwrap() as u16 & 0xF0 & 0xF0; // Low;
    cpu.sp += 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_F2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // F2 LD A,(C)
    cpu.a = memory.read8(0xFF00 + cpu.c).unwrap() as u16;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn DI_F3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // F3 DI
    cpu.interrupt_master_enable = false;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn PUSH_F5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // F5 PUSH AF
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.a) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.f & 0xF0) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn OR_F6(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // F6 OR d8
    let mut t: u16 = cpu.a | v;
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_F7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // F7 RST 30H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 48;
    return 16;
}

fn LD_F8(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // F8 LD HL,SP+r8
    cpu.set_hl(cpu.sp + ((v ^ 0x80) - 0x80));
    let mut t: u16 = cpu.get_hl();
    let mut flag: u16 = 0b00000000;
    flag += u16::from(((cpu.sp & 0xF) + (v & 0xF)) > 0xF) << FLAGH;
    flag += u16::from(((cpu.sp & 0xFF) + (v & 0xFF)) > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    cpu.set_hl( cpu.get_hl() & 0xFFFF);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 12;
}

fn LD_F9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // F9 LD SP,HL
    cpu.sp = cpu.get_hl();
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn LD_FA(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // FA LD A,(a16)
    cpu.a = memory.read8(v).unwrap() as u16;
    cpu.pc += 3;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn EI_FB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // FB EI
    cpu.interrupt_master_enable = true;
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    return 4;
}

fn CP_FE(cpu: &mut CPU, memory: &mut Memory, v: u16) -> u32 { // FE CP d8
    let mut t: u16 = cpu.a - v;
    let mut flag: u16 = 0b01000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(((cpu.a & 0xF) - (v & 0xF)) < 0) << FLAGH;
    flag += u16::from(t < 0) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RST_FF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // FF RST 38H
    cpu.pc += 1;
    cpu.pc &= 0xFFFF;
    memory.write8((cpu.sp- 1) & 0xFFFF, (cpu.pc >> 8) as u8); // High;
    memory.write8((cpu.sp- 2) & 0xFFFF, (cpu.pc & 0xFF) as u8); // Low;
    cpu.sp -= 2;
    cpu.sp &= 0xFFFF;
    cpu.pc = 56;
    return 16;
}

fn RLC_100(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 100 RLC B
    let mut t: u16 = (cpu.b << 1) + (cpu.b >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_101(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 101 RLC C
    let mut t: u16 = (cpu.c << 1) + (cpu.c >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_102(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 102 RLC D
    let mut t: u16 = (cpu.d << 1) + (cpu.d >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_103(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 103 RLC E
    let mut t: u16 = (cpu.e << 1) + (cpu.e >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_104(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 104 RLC H
    let mut t: u16 = ((cpu.get_hl() >> 8) << 1) + ((cpu.get_hl() >> 8) >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_105(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 105 RLC L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) << 1) + ((cpu.get_hl() & 0xFF) >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RLC_106(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 106 RLC (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()).unwrap() as u16) << 1) + ((memory.read8(cpu.get_hl()).unwrap() as u16) >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RLC_107(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 107 RLC A
    let mut t: u16 = (cpu.a << 1) + (cpu.a >> 7);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_108(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 108 RRC B
    let mut t: u16 = (cpu.b >> 1) + ((cpu.b & 1) << 7) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_109(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 109 RRC C
    let mut t: u16 = (cpu.c >> 1) + ((cpu.c & 1) << 7) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 10A RRC D
    let mut t: u16 = (cpu.d >> 1) + ((cpu.d & 1) << 7) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 10B RRC E
    let mut t: u16 = (cpu.e >> 1) + ((cpu.e & 1) << 7) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 10C RRC H
    let mut t: u16 = ((cpu.get_hl() >> 8) >> 1) + (((cpu.get_hl() >> 8) & 1) << 7) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 10D RRC L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) >> 1) + (((cpu.get_hl() & 0xFF) & 1) << 7) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RRC_10E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 10E RRC (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()).unwrap() as u16) >> 1) + (((memory.read8(cpu.get_hl()).unwrap() as u16) & 1) << 7) + (((memory.read8(cpu.get_hl()).unwrap() as u16) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RRC_10F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 10F RRC A
    let mut t: u16 = (cpu.a >> 1) + ((cpu.a & 1) << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_110(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 110 RL B
    let mut t: u16 = (cpu.b << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_111(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 111 RL C
    let mut t: u16 = (cpu.c << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_112(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 112 RL D
    let mut t: u16 = (cpu.d << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_113(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 113 RL E
    let mut t: u16 = (cpu.e << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_114(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 114 RL H
    let mut t: u16 = ((cpu.get_hl() >> 8) << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_115(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 115 RL L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RL_116(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 116 RL (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()).unwrap() as u16) << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RL_117(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 117 RL A
    let mut t: u16 = (cpu.a << 1) + u16::from(cpu.f_c());
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_118(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 118 RR B
    let mut t: u16 = (cpu.b >> 1) + ((cpu.f_c() as u16) << 7) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_119(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 119 RR C
    let mut t: u16 = (cpu.c >> 1) + ((cpu.f_c() as u16) << 7) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 11A RR D
    let mut t: u16 = (cpu.d >> 1) + ((cpu.f_c() as u16) << 7) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 11B RR E
    let mut t: u16 = (cpu.e >> 1) + ((cpu.f_c() as u16) << 7) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 11C RR H
    let mut t: u16 = ((cpu.get_hl() >> 8) >> 1) + ((cpu.f_c() as u16) << 7) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 11D RR L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) >> 1) + ((cpu.f_c() as u16) << 7) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RR_11E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 11E RR (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()).unwrap() as u16) >> 1) + ((cpu.f_c() as u16) << 7) + (((memory.read8(cpu.get_hl()).unwrap() as u16) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RR_11F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 11F RR A
    let mut t: u16 = (cpu.a >> 1) + ((cpu.f_c() as u16) << 7) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_120(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 120 SLA B
    let mut t: u16 = (cpu.b << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_121(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 121 SLA C
    let mut t: u16 = (cpu.c << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_122(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 122 SLA D
    let mut t: u16 = (cpu.d << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_123(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 123 SLA E
    let mut t: u16 = (cpu.e << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_124(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 124 SLA H
    let mut t: u16 = ((cpu.get_hl() >> 8) << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_125(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 125 SLA L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SLA_126(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 126 SLA (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()).unwrap() as u16) << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SLA_127(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 127 SLA A
    let mut t: u16 = (cpu.a << 1);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_128(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 128 SRA B
    let mut t: u16 = ((cpu.b >> 1) | (cpu.b & 0x80)) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_129(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 129 SRA C
    let mut t: u16 = ((cpu.c >> 1) | (cpu.c & 0x80)) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 12A SRA D
    let mut t: u16 = ((cpu.d >> 1) | (cpu.d & 0x80)) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 12B SRA E
    let mut t: u16 = ((cpu.e >> 1) | (cpu.e & 0x80)) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 12C SRA H
    let mut t: u16 = (((cpu.get_hl() >> 8) >> 1) | ((cpu.get_hl() >> 8) & 0x80)) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 12D SRA L
    let mut t: u16 = (((cpu.get_hl() & 0xFF) >> 1) | ((cpu.get_hl() & 0xFF) & 0x80)) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRA_12E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 12E SRA (HL)
    let mut t: u16 = (((memory.read8(cpu.get_hl()).unwrap() as u16) >> 1) | ((memory.read8(cpu.get_hl()).unwrap() as u16) & 0x80)) + (((memory.read8(cpu.get_hl()).unwrap() as u16) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SRA_12F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 12F SRA A
    let mut t: u16 = ((cpu.a >> 1) | (cpu.a & 0x80)) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_130(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 130 SWAP B
    let mut t: u16 = ((cpu.b & 0xF0) >> 4) | ((cpu.b & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_131(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 131 SWAP C
    let mut t: u16 = ((cpu.c & 0xF0) >> 4) | ((cpu.c & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_132(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 132 SWAP D
    let mut t: u16 = ((cpu.d & 0xF0) >> 4) | ((cpu.d & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_133(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 133 SWAP E
    let mut t: u16 = ((cpu.e & 0xF0) >> 4) | ((cpu.e & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_134(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 134 SWAP H
    let mut t: u16 = (((cpu.get_hl() >> 8) & 0xF0) >> 4) | (((cpu.get_hl() >> 8) & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_135(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 135 SWAP L
    let mut t: u16 = (((cpu.get_hl() & 0xFF) & 0xF0) >> 4) | (((cpu.get_hl() & 0xFF) & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SWAP_136(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 136 SWAP (HL)
    let mut t: u16 = (((memory.read8(cpu.get_hl()).unwrap() as u16) & 0xF0) >> 4) | (((memory.read8(cpu.get_hl()).unwrap() as u16) & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SWAP_137(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 137 SWAP A
    let mut t: u16 = ((cpu.a & 0xF0) >> 4) | ((cpu.a & 0x0F) << 4);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_138(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 138 SRL B
    let mut t: u16 = (cpu.b >> 1) + ((cpu.b & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_139(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 139 SRL C
    let mut t: u16 = (cpu.c >> 1) + ((cpu.c & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 13A SRL D
    let mut t: u16 = (cpu.d >> 1) + ((cpu.d & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 13B SRL E
    let mut t: u16 = (cpu.e >> 1) + ((cpu.e & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 13C SRL H
    let mut t: u16 = ((cpu.get_hl() >> 8) >> 1) + (((cpu.get_hl() >> 8) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 13D SRL L
    let mut t: u16 = ((cpu.get_hl() & 0xFF) >> 1) + (((cpu.get_hl() & 0xFF) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SRL_13E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 13E SRL (HL)
    let mut t: u16 = ((memory.read8(cpu.get_hl()).unwrap() as u16) >> 1) + (((memory.read8(cpu.get_hl()).unwrap() as u16) & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SRL_13F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 13F SRL A
    let mut t: u16 = (cpu.a >> 1) + ((cpu.a & 1) << 8);
    let mut flag: u16 = 0b00000000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    flag += u16::from(t > 0xFF) << FLAGC;
    cpu.f &= 0b00000000;
    cpu.f |= flag;
    t &= 0xFF;
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_140(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 140 BIT 0,B
    let mut t: u16 = cpu.b & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_141(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 141 BIT 0,C
    let mut t: u16 = cpu.c & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_142(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 142 BIT 0,D
    let mut t: u16 = cpu.d & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_143(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 143 BIT 0,E
    let mut t: u16 = cpu.e & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_144(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 144 BIT 0,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_145(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 145 BIT 0,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_146(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 146 BIT 0,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_147(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 147 BIT 0,A
    let mut t: u16 = cpu.a & (1 << 0);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_148(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 148 BIT 1,B
    let mut t: u16 = cpu.b & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_149(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 149 BIT 1,C
    let mut t: u16 = cpu.c & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 14A BIT 1,D
    let mut t: u16 = cpu.d & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 14B BIT 1,E
    let mut t: u16 = cpu.e & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 14C BIT 1,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 14D BIT 1,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_14E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 14E BIT 1,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_14F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 14F BIT 1,A
    let mut t: u16 = cpu.a & (1 << 1);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_150(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 150 BIT 2,B
    let mut t: u16 = cpu.b & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_151(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 151 BIT 2,C
    let mut t: u16 = cpu.c & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_152(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 152 BIT 2,D
    let mut t: u16 = cpu.d & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_153(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 153 BIT 2,E
    let mut t: u16 = cpu.e & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_154(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 154 BIT 2,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_155(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 155 BIT 2,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_156(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 156 BIT 2,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_157(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 157 BIT 2,A
    let mut t: u16 = cpu.a & (1 << 2);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_158(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 158 BIT 3,B
    let mut t: u16 = cpu.b & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_159(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 159 BIT 3,C
    let mut t: u16 = cpu.c & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 15A BIT 3,D
    let mut t: u16 = cpu.d & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 15B BIT 3,E
    let mut t: u16 = cpu.e & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 15C BIT 3,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 15D BIT 3,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_15E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 15E BIT 3,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_15F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 15F BIT 3,A
    let mut t: u16 = cpu.a & (1 << 3);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_160(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 160 BIT 4,B
    let mut t: u16 = cpu.b & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_161(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 161 BIT 4,C
    let mut t: u16 = cpu.c & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_162(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 162 BIT 4,D
    let mut t: u16 = cpu.d & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_163(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 163 BIT 4,E
    let mut t: u16 = cpu.e & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_164(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 164 BIT 4,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_165(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 165 BIT 4,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_166(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 166 BIT 4,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_167(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 167 BIT 4,A
    let mut t: u16 = cpu.a & (1 << 4);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_168(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 168 BIT 5,B
    let mut t: u16 = cpu.b & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_169(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 169 BIT 5,C
    let mut t: u16 = cpu.c & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 16A BIT 5,D
    let mut t: u16 = cpu.d & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 16B BIT 5,E
    let mut t: u16 = cpu.e & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 16C BIT 5,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 16D BIT 5,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_16E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 16E BIT 5,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_16F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 16F BIT 5,A
    let mut t: u16 = cpu.a & (1 << 5);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_170(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 170 BIT 6,B
    let mut t: u16 = cpu.b & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_171(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 171 BIT 6,C
    let mut t: u16 = cpu.c & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_172(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 172 BIT 6,D
    let mut t: u16 = cpu.d & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_173(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 173 BIT 6,E
    let mut t: u16 = cpu.e & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_174(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 174 BIT 6,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_175(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 175 BIT 6,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_176(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 176 BIT 6,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_177(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 177 BIT 6,A
    let mut t: u16 = cpu.a & (1 << 6);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_178(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 178 BIT 7,B
    let mut t: u16 = cpu.b & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_179(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 179 BIT 7,C
    let mut t: u16 = cpu.c & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 17A BIT 7,D
    let mut t: u16 = cpu.d & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 17B BIT 7,E
    let mut t: u16 = cpu.e & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 17C BIT 7,H
    let mut t: u16 = (cpu.get_hl() >> 8) & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 17D BIT 7,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn BIT_17E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 17E BIT 7,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn BIT_17F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 17F BIT 7,A
    let mut t: u16 = cpu.a & (1 << 7);
    let mut flag: u16 = 0b00100000;
    flag += u16::from((t & 0xFF) == 0) << FLAGZ;
    cpu.f &= 0b00010000;
    cpu.f |= flag;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_180(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 180 RES 0,B
    let mut t: u16 = cpu.b & !(1 << 0);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_181(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 181 RES 0,C
    let mut t: u16 = cpu.c & !(1 << 0);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_182(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 182 RES 0,D
    let mut t: u16 = cpu.d & !(1 << 0);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_183(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 183 RES 0,E
    let mut t: u16 = cpu.e & !(1 << 0);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_184(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 184 RES 0,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 0);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_185(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 185 RES 0,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 0);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_186(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 186 RES 0,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 0);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_187(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 187 RES 0,A
    let mut t: u16 = cpu.a & !(1 << 0);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_188(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 188 RES 1,B
    let mut t: u16 = cpu.b & !(1 << 1);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_189(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 189 RES 1,C
    let mut t: u16 = cpu.c & !(1 << 1);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 18A RES 1,D
    let mut t: u16 = cpu.d & !(1 << 1);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 18B RES 1,E
    let mut t: u16 = cpu.e & !(1 << 1);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 18C RES 1,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 1);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 18D RES 1,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 1);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_18E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 18E RES 1,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 1);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_18F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 18F RES 1,A
    let mut t: u16 = cpu.a & !(1 << 1);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_190(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 190 RES 2,B
    let mut t: u16 = cpu.b & !(1 << 2);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_191(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 191 RES 2,C
    let mut t: u16 = cpu.c & !(1 << 2);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_192(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 192 RES 2,D
    let mut t: u16 = cpu.d & !(1 << 2);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_193(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 193 RES 2,E
    let mut t: u16 = cpu.e & !(1 << 2);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_194(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 194 RES 2,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 2);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_195(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 195 RES 2,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 2);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_196(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 196 RES 2,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 2);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_197(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 197 RES 2,A
    let mut t: u16 = cpu.a & !(1 << 2);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_198(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 198 RES 3,B
    let mut t: u16 = cpu.b & !(1 << 3);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_199(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 199 RES 3,C
    let mut t: u16 = cpu.c & !(1 << 3);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19A(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 19A RES 3,D
    let mut t: u16 = cpu.d & !(1 << 3);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19B(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 19B RES 3,E
    let mut t: u16 = cpu.e & !(1 << 3);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19C(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 19C RES 3,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 3);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19D(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 19D RES 3,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 3);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_19E(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 19E RES 3,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 3);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_19F(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 19F RES 3,A
    let mut t: u16 = cpu.a & !(1 << 3);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A0 RES 4,B
    let mut t: u16 = cpu.b & !(1 << 4);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A1 RES 4,C
    let mut t: u16 = cpu.c & !(1 << 4);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A2 RES 4,D
    let mut t: u16 = cpu.d & !(1 << 4);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A3 RES 4,E
    let mut t: u16 = cpu.e & !(1 << 4);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A4 RES 4,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 4);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A5 RES 4,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 4);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A6 RES 4,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 4);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1A7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A7 RES 4,A
    let mut t: u16 = cpu.a & !(1 << 4);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A8 RES 5,B
    let mut t: u16 = cpu.b & !(1 << 5);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1A9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1A9 RES 5,C
    let mut t: u16 = cpu.c & !(1 << 5);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1AA RES 5,D
    let mut t: u16 = cpu.d & !(1 << 5);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1AB RES 5,E
    let mut t: u16 = cpu.e & !(1 << 5);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1AC RES 5,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 5);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AD(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1AD RES 5,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 5);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1AE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1AE RES 5,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 5);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1AF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1AF RES 5,A
    let mut t: u16 = cpu.a & !(1 << 5);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B0 RES 6,B
    let mut t: u16 = cpu.b & !(1 << 6);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B1 RES 6,C
    let mut t: u16 = cpu.c & !(1 << 6);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B2 RES 6,D
    let mut t: u16 = cpu.d & !(1 << 6);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B3 RES 6,E
    let mut t: u16 = cpu.e & !(1 << 6);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B4 RES 6,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 6);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B5 RES 6,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 6);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B6 RES 6,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 6);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1B7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B7 RES 6,A
    let mut t: u16 = cpu.a & !(1 << 6);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B8 RES 7,B
    let mut t: u16 = cpu.b & !(1 << 7);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1B9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1B9 RES 7,C
    let mut t: u16 = cpu.c & !(1 << 7);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1BA RES 7,D
    let mut t: u16 = cpu.d & !(1 << 7);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1BB RES 7,E
    let mut t: u16 = cpu.e & !(1 << 7);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1BC RES 7,H
    let mut t: u16 = (cpu.get_hl() >> 8) & !(1 << 7);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BD(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1BD RES 7,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) & !(1 << 7);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn RES_1BE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1BE RES 7,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) & !(1 << 7);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn RES_1BF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1BF RES 7,A
    let mut t: u16 = cpu.a & !(1 << 7);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C0 SET 0,B
    let mut t: u16 = cpu.b | (1 << 0);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C1 SET 0,C
    let mut t: u16 = cpu.c | (1 << 0);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C2 SET 0,D
    let mut t: u16 = cpu.d | (1 << 0);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C3 SET 0,E
    let mut t: u16 = cpu.e | (1 << 0);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C4 SET 0,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 0);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C5 SET 0,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 0);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C6 SET 0,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 0);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1C7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C7 SET 0,A
    let mut t: u16 = cpu.a | (1 << 0);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C8 SET 1,B
    let mut t: u16 = cpu.b | (1 << 1);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1C9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1C9 SET 1,C
    let mut t: u16 = cpu.c | (1 << 1);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1CA SET 1,D
    let mut t: u16 = cpu.d | (1 << 1);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1CB SET 1,E
    let mut t: u16 = cpu.e | (1 << 1);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1CC SET 1,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 1);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CD(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1CD SET 1,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 1);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1CE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1CE SET 1,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 1);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1CF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1CF SET 1,A
    let mut t: u16 = cpu.a | (1 << 1);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D0 SET 2,B
    let mut t: u16 = cpu.b | (1 << 2);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D1 SET 2,C
    let mut t: u16 = cpu.c | (1 << 2);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D2 SET 2,D
    let mut t: u16 = cpu.d | (1 << 2);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D3 SET 2,E
    let mut t: u16 = cpu.e | (1 << 2);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D4 SET 2,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 2);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D5 SET 2,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 2);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D6 SET 2,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 2);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1D7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D7 SET 2,A
    let mut t: u16 = cpu.a | (1 << 2);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D8 SET 3,B
    let mut t: u16 = cpu.b | (1 << 3);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1D9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1D9 SET 3,C
    let mut t: u16 = cpu.c | (1 << 3);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1DA SET 3,D
    let mut t: u16 = cpu.d | (1 << 3);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1DB SET 3,E
    let mut t: u16 = cpu.e | (1 << 3);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1DC SET 3,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 3);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DD(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1DD SET 3,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 3);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1DE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1DE SET 3,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 3);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1DF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1DF SET 3,A
    let mut t: u16 = cpu.a | (1 << 3);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E0 SET 4,B
    let mut t: u16 = cpu.b | (1 << 4);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E1 SET 4,C
    let mut t: u16 = cpu.c | (1 << 4);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E2 SET 4,D
    let mut t: u16 = cpu.d | (1 << 4);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E3 SET 4,E
    let mut t: u16 = cpu.e | (1 << 4);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E4 SET 4,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 4);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E5 SET 4,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 4);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E6 SET 4,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 4);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1E7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E7 SET 4,A
    let mut t: u16 = cpu.a | (1 << 4);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E8 SET 5,B
    let mut t: u16 = cpu.b | (1 << 5);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1E9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1E9 SET 5,C
    let mut t: u16 = cpu.c | (1 << 5);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1EA SET 5,D
    let mut t: u16 = cpu.d | (1 << 5);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1EB SET 5,E
    let mut t: u16 = cpu.e | (1 << 5);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1EC SET 5,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 5);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1ED(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1ED SET 5,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 5);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1EE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1EE SET 5,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 5);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1EF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1EF SET 5,A
    let mut t: u16 = cpu.a | (1 << 5);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F0(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F0 SET 6,B
    let mut t: u16 = cpu.b | (1 << 6);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F1(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F1 SET 6,C
    let mut t: u16 = cpu.c | (1 << 6);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F2(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F2 SET 6,D
    let mut t: u16 = cpu.d | (1 << 6);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F3(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F3 SET 6,E
    let mut t: u16 = cpu.e | (1 << 6);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F4(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F4 SET 6,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 6);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F5(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F5 SET 6,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 6);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F6(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F6 SET 6,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 6);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1F7(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F7 SET 6,A
    let mut t: u16 = cpu.a | (1 << 6);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F8(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F8 SET 7,B
    let mut t: u16 = cpu.b | (1 << 7);
    cpu.b = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1F9(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1F9 SET 7,C
    let mut t: u16 = cpu.c | (1 << 7);
    cpu.c = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FA(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1FA SET 7,D
    let mut t: u16 = cpu.d | (1 << 7);
    cpu.d = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FB(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1FB SET 7,E
    let mut t: u16 = cpu.e | (1 << 7);
    cpu.e = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FC(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1FC SET 7,H
    let mut t: u16 = (cpu.get_hl() >> 8) | (1 << 7);
    cpu.set_hl((cpu.get_hl() & 0x00FF) | (t << 8));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FD(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1FD SET 7,L
    let mut t: u16 = (cpu.get_hl() & 0xFF) | (1 << 7);
    cpu.set_hl((cpu.get_hl() & 0xFF00) | (t & 0xFF));
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

fn SET_1FE(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1FE SET 7,(HL)
    let mut t: u16 = (memory.read8(cpu.get_hl()).unwrap() as u16) | (1 << 7);
    memory.write8(cpu.get_hl(), (t) as u8);
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 16;
}

fn SET_1FF(cpu: &mut CPU, memory: &mut Memory) -> u32 { // 1FF SET 7,A
    let mut t: u16 = cpu.a | (1 << 7);
    cpu.a = t;
    cpu.pc += 2;
    cpu.pc &= 0xFFFF;
    return 8;
}

const OPCODE_LENGTHS: [u8; 512] = [
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1,
    1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1,
    2, 1, 1, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1,
    2, 1, 1, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];


const CPU_COMMANDS: [&str; 512] = [
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
    "SET 7,A"
    ];