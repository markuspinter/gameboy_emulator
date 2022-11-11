use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::format};

use std::fs::File;
use std::io::{BufReader, Read};

use crate::gameboy::memory;
use crate::utils;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use self::mbc0::MBC0;
use self::mbc1::MBC1;

use super::MemoryInterface;

pub mod mbc0;
pub mod mbc1;

#[derive(Clone, Debug, FromPrimitive)]
#[allow(non_camel_case_types)]
enum CartridgeType {
    ROM_ONLY = 0x00,
    MBC1 = 0x01,
    MBC1_RAM = 0x02,
    MBC1_RAM_BATTERY = 0x03,
    MBC3_RAM_BATTERY = 0x13,
    UNKNOWN = 0xFF,
}

#[derive(Clone, Debug)]
struct CartridgeHeader {
    title: String,
    cartridge_type: CartridgeType,
    rom_size: usize,
    rom_banks: usize,
    ram_size: usize,
    ram_banks: usize,
}

struct MBC(Box<dyn MBCInterface>);
trait MBCInterface {
    fn read8_rom_bank_0(&self, addr: u16) -> u8;
    fn read8_rom_bank_n(&self, addr: u16) -> u8;
    fn read8_ram_bank_n(&self, addr: u16) -> u8;

    fn write8_rom_bank_0(&mut self, addr: u16, value: u8);
    fn write8_rom_bank_n(&mut self, addr: u16, value: u8);
    fn write8_ram_bank_n(&mut self, addr: u16, value: u8);

    fn get_rom(&self) -> &Vec<u8>;

    fn new(rom: Vec<u8>, ram: Vec<u8>) -> Self
    where
        Self: Sized;
}

impl MemoryInterface for MBC {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr >= memory::cartridge::ROM_BANK_0.begin && addr <= memory::cartridge::ROM_BANK_0.end {
            return Some(self.0.read8_rom_bank_0(addr));
        } else if addr >= memory::cartridge::ROM_BANK_N.begin && addr <= memory::cartridge::ROM_BANK_N.end {
            return Some(self.0.read8_rom_bank_n(addr));
        } else if addr >= memory::cartridge::EXTERNAL_RAM.begin && addr <= memory::cartridge::EXTERNAL_RAM.end {
            return Some(self.0.read8_ram_bank_n(addr));
        } else {
            return None;
        }
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr >= memory::cartridge::ROM_BANK_0.begin && addr <= memory::cartridge::ROM_BANK_0.end {
            self.0.write8_rom_bank_0(addr, value);
        } else if addr >= memory::cartridge::ROM_BANK_N.begin && addr <= memory::cartridge::ROM_BANK_N.end {
            self.0.write8_rom_bank_n(addr, value);
        } else if addr >= memory::cartridge::EXTERNAL_RAM.begin && addr <= memory::cartridge::EXTERNAL_RAM.end {
            self.0.write8_ram_bank_n(addr, value);
        } else {
            return None;
        }
        return Some(());
    }
}

lazy_static! {
    static ref ROM_SIZE_MAP: HashMap<u8, (usize, &'static str)> = vec![
        (0x00, (0x8000, "32 KiByte 	2 banks(No ROM banking)")),
        (0x01, (0x10000, "64 KiByte 	4 banks")),
        (0x02, (0x20000, "128 KiByte 	8 banks")),
        (0x03, (0x40000, "256 KiByte 	16 banks")),
        (0x04, (0x80000, "512 KiByte 	32 banks")),
        (0x05, (0x100000, "1 MiByte 	64 banks")),
        (0x06, (0x200000, "2 MiByte 	128 banks")),
        (0x07, (0x400000, "4 MiByte 	256 banks")),
        (0x08, (0x800000, "8 MiByte 	512 banks")),
    ]
    .iter()
    .copied()
    .collect();

    static ref RAM_SIZE_MAP: HashMap<u8, (usize, &'static str)> = vec![
        (0x00, (0, "None")),
        (0x01, (0, "2 KiBytes (unused)")), //unused
        (0x02, (0x2000, "8 KiBytes")),
        (0x03, (0x8000, "32 KiBytes (4 banks of 8KBytes each)")),
        (0x04, (0x20000, "128 KiBytes (16 banks of 8KBytes each)")),
        (0x05, (0x10000, "64 KiBytes (8 banks of 8KBytes each)")),
    ]
    .iter()
    .copied()
    .collect();
}

impl std::convert::From<&Vec<u8>> for CartridgeHeader {
    fn from(rom: &Vec<u8>) -> CartridgeHeader {
        if rom.len() < 0x150 {
            panic!("rom too small {}", rom.len());
        }

        let mut header: CartridgeHeader = CartridgeHeader {
            title: "".to_string(),
            cartridge_type: CartridgeType::UNKNOWN,
            rom_size: 0,
            rom_banks: 0,
            ram_size: 0,
            ram_banks: 0,
        };
        header.title =
            String::from(std::str::from_utf8(&rom[0x134..=0x143]).unwrap_or("failed to parse cartridge title"));
        header.cartridge_type =
            FromPrimitive::from_u8(rom[0x147]).expect(format!("cartridge type not supported {}", rom[0x147]).as_str());
        header.rom_size = ROM_SIZE_MAP[&rom[0x148]].0;
        header.rom_banks = header.rom_size / 0x4000;
        header.ram_size = std::cmp::max(RAM_SIZE_MAP[&rom[0x149]].0, 0x2000);
        header.ram_banks = header.ram_size / 0x2000;

        if header.rom_size >= 0x200000 {
            panic!("2MiB rom cartridges not supported yet");
        }

        header
    }
}

pub struct Cartridge {
    header: CartridgeHeader,
    mbc: MBC,
    boot_rom: Vec<u8>,
    boot_flag: u8,
}

impl MemoryInterface for Cartridge {
    fn read8(&self, addr: u16) -> Option<u8> {
        if self.boot_flag == 0 {
            if addr >= memory::cartridge::BOOTROM.begin && addr <= memory::cartridge::BOOTROM.end {
                return Some(self.boot_rom[addr as usize]);
            }
        }
        if let Some(res) = self.mbc.read8(addr) {
            return Some(res);
        } else if addr == memory::cartridge::BOOTROM_FLAG {
            return Some(self.boot_flag);
        } else {
            return None;
        }
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if self.boot_flag == 0 {
            if addr >= memory::cartridge::BOOTROM.begin && addr <= memory::cartridge::BOOTROM.end {
                log::error!("trying to write to bootrom, addr {:#06X}", addr);
            }
        }
        if let Some(res) = self.mbc.write8(addr, value) {
        } else if addr == memory::cartridge::BOOTROM_FLAG {
            log::warn!("bootrom flag set to {:#04X}", value);
            self.boot_flag = value;
        } else {
            return None;
        }
        return Some(());
    }
}

impl Cartridge {
    pub fn new(bootrom_path: String, rom_path: String) -> Self {
        let rom = Self::load_rom(rom_path);
        let header = CartridgeHeader::from(&rom);
        let ram = vec![0; header.ram_size];

        let mut mem = Cartridge {
            header: header.clone(),
            boot_rom: Self::load_boot_rom(bootrom_path),
            boot_flag: 0,
            mbc: match header.cartridge_type {
                CartridgeType::ROM_ONLY => MBC(Box::new(MBC0::new(rom, ram))),
                CartridgeType::MBC1 => MBC(Box::new(MBC1::new(rom, ram))),
                CartridgeType::MBC1_RAM => MBC(Box::new(MBC1::new(rom, ram))),
                CartridgeType::MBC1_RAM_BATTERY => MBC(Box::new(MBC1::new(rom, ram))),
                CartridgeType::MBC3_RAM_BATTERY => MBC(Box::new(MBC1::new(rom, ram))),
                CartridgeType::UNKNOWN => panic!("cartridge type is 0xFF"),
            },
        };

        mem
    }

    fn load_rom(rom_path: String) -> Vec<u8> {
        let f = File::open(rom_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        // buffer.splice(..0x100, Self::load_boot_rom(bootrom_path));

        utils::print_memory_bytes(&buffer, "rom", 0x100);
        buffer
    }

    fn load_ram(rom_path: String) -> Vec<u8> {
        Vec::new()
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

    pub fn debug_print(&self) {
        let mut print_str = String::new();

        use std::fmt::Write;
        writeln!(print_str, "Title:\t{}", self.header.title).unwrap();
        writeln!(print_str, "Cartridge Type: {:?}", self.header.cartridge_type).unwrap();
        writeln!(print_str, "ROM Size: {}", ROM_SIZE_MAP[&self.mbc.0.get_rom()[0x148]].1).unwrap();
        writeln!(print_str, "RAM Size: {}", RAM_SIZE_MAP[&self.mbc.0.get_rom()[0x149]].1).unwrap();

        println!("{}", print_str);
    }
}
