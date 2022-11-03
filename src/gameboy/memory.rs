use std::fs::File;
use std::io::{BufReader, Read};

use crate::utils;

use super::MemoryInterface;
pub struct Memory {
    rom: Vec<u8>,
    ram: [u8; 0x10000],
}

impl MemoryInterface for Memory {
    fn read8(&self, addr: u16) -> Option<u8> {
        Some(self.ram[usize::from(addr)])
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        self.ram[usize::from(addr)] = value;
        Some(())
    }
}

impl Memory {
    pub fn new(bootrom_path: String, rom_path: String) -> Self {
        let mut mem = Memory {
            rom: Self::load_rom(bootrom_path, rom_path),
            ram: [0; 0x10000],
        };
        mem.ram[0..0x8000].clone_from_slice(mem.rom[0..0x8000].into());

        mem
    }

    fn load_rom(bootrom_path: String, rom_path: String) -> Vec<u8> {
        let f = File::open(rom_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        // buffer.splice(..0x100, Self::load_boot_rom(bootrom_path));

        utils::print_memory_bytes(&buffer, "rom", 0x100);
        buffer
    }

    fn load_boot_rom(bootrom_path: String) -> Vec<u8> {
        let f = File::open(bootrom_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        utils::print_memory(&buffer, "bootrom");
        println!("\n\n");
        buffer
    }
}

pub struct MemoryRange {
    pub begin: u16,
    pub end: u16,
    pub size: usize,
}

pub mod cartridge {
    use super::MemoryRange;

    pub const ROM_BANK_0: MemoryRange = MemoryRange {
        begin: 0x0000,
        end: 0x3FFF,
        size: 0x4000,
    };
    pub const ROM_BANK_N: MemoryRange = MemoryRange {
        begin: 0x4000,
        end: 0x7FFF,
        size: 0x4000,
    };

    pub const EXTERNAL_RAM: MemoryRange = MemoryRange {
        begin: 0xA000,
        end: 0xBFFF,
        size: 0x2000,
    };

    pub const BOOTROM_FLAG: u16 = 0xFF40;
}

pub mod ppu {
    use super::MemoryRange;

    pub const VRAM: MemoryRange = MemoryRange {
        begin: 0x8000,
        end: 0x9FFF,
        size: 0x2000,
    };
    pub const OAM: MemoryRange = MemoryRange {
        begin: 0xFE00,
        end: 0xFE9F,
        size: 0x00A0,
    };
    pub const CONTROL: MemoryRange = MemoryRange {
        begin: 0xFF40,
        end: 0xFF4B,
        size: 0x000B,
    };
    pub const TILE_DATA: MemoryRange = MemoryRange {
        begin: 0x8000,
        end: 0x97FF,
        size: 0x1800,
    };
    pub const TILE_DATA_VRAM: MemoryRange = MemoryRange {
        begin: 0x0000,
        end: 0x17FF,
        size: 0x1800,
    };
    pub const TILE_MAP: MemoryRange = MemoryRange {
        begin: 0x9800,
        end: 0x9FFF,
        size: 0x0800,
    };
    pub const TILE_MAP_VRAM: MemoryRange = MemoryRange {
        begin: 0x1800,
        end: 0x1FFF,
        size: 0x0800,
    };
    pub const TILE_MAP_AREA_9800: MemoryRange = MemoryRange {
        begin: 0x9800,
        end: 0x9BFF,
        size: 0x400,
    };
    pub const TILE_MAP_AREA_9C00: MemoryRange = MemoryRange {
        begin: 0x9C00,
        end: 0x9FFF,
        size: 0x400,
    };
    pub const TILE_MAP_AREA_9800_VRAM: MemoryRange = MemoryRange {
        begin: 0x1800,
        end: 0x1BFF,
        size: 0x400,
    };
    pub const TILE_MAP_AREA_9C00_VRAM: MemoryRange = MemoryRange {
        begin: 0x1C00,
        end: 0x1FFF,
        size: 0x400,
    };
    pub const TILE_DATA_AREA_8000: MemoryRange = MemoryRange {
        begin: 0x8000,
        end: 0x8FFF,
        size: 0x1000,
    };
    pub const TILE_DATA_AREA_8800: MemoryRange = MemoryRange {
        begin: 0x8800,
        end: 0x97FF,
        size: 0x1000,
    };
    pub const LCDC: u16 = 0xFF40;
    pub const STAT: u16 = 0xFF41;
    pub const SCY: u16 = 0xFF42;
    pub const SCX: u16 = 0xFF43;
    pub const LY: u16 = 0xFF44;
    pub const LYC: u16 = 0xFF45;
    pub const DMA: u16 = 0xFF46;
    pub const BGP: u16 = 0xFF47;
    pub const OBP0: u16 = 0xFF48;
    pub const OBP1: u16 = 0xFF49;
    pub const WY: u16 = 0xFF4A;
    pub const WX: u16 = 0xFF4B;
}

pub mod joypad {
    pub const JOYP: u16 = 0xFF00;
}

pub mod serial {
    pub const SB: u16 = 0xFF01;
    pub const SC: u16 = 0xFF02;
}

pub mod timer {
    pub const DIV: u16 = 0xFF04;
    pub const TIMA: u16 = 0xFF05;
    pub const TMA: u16 = 0xFF06;
    pub const TAC: u16 = 0xFF07;
}

pub mod interrupt {
    pub const IF: u16 = 0xFF0F;
    pub const IE: u16 = 0xFFFF;
}
