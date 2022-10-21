use std::fs::File;
use std::io::{BufReader, Read};

use super::MemoryInterface;
pub struct Memory {
    rom: Vec<u8>,
}

impl MemoryInterface for Memory {
    fn read8(&self, addr: u16) -> super::MemoryResult<u8> {
        Ok(self.rom[usize::from(addr)])
    }

    fn write8(&mut self, addr: u16, value: u8) -> super::MemoryResult<()> {
        self.rom[usize::from(addr)] = value;
        Ok(())
    }
}

impl Memory {
    pub fn new(bootrom_path: String, rom_path: String) -> Self {
        Memory {
            rom: Self::load_rom(bootrom_path, rom_path),
        }
    }

    fn load_rom(bootrom_path: String, rom_path: String) -> Vec<u8> {
        let f = File::open(rom_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        buffer.splice(..0x100, Self::load_boot_rom(bootrom_path));

        // Read.
        print!("      |  ");
        for i in 0..0x10 {
            print!("{:#04X}  |  ", i);
        }
        println!();
        for _i in 0..0x11 {
            print!("{:_<9}", "");
        }
        for (i, value) in buffer.iter_mut().enumerate() {
            if (i % 0x10) == 0 {
                println!();
                print!("{:#04X}  |  ", i);
            }
            print!("{:#04X}  |  ", value);
            if i > 0x1000 {
                break;
            };
        }
        println!("\n\n");
        buffer
    }

    fn load_boot_rom(bootrom_path: String) -> Vec<u8> {
        let f = File::open(bootrom_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        // Read.
        print!("bootrm|  ");
        for i in 0..0x10 {
            print!("{:#04X}  |  ", i);
        }
        println!();
        for _i in 0..0x11 {
            print!("{:_<9}", "");
        }
        for (i, value) in buffer.iter_mut().enumerate() {
            if (i % 0x10) == 0 {
                println!();
                print!("{:#04X}  |  ", i);
            }
            print!("{:#04X}  |  ", value);
        }
        println!("\n\n");
        buffer
    }
}
