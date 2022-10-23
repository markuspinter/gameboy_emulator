use std::{fmt::Error, time::SystemTime};

pub mod cpu;
pub mod interrupts;
pub mod joypad;
pub mod memory;
pub mod ppu;
pub mod sound;
pub mod timer;

use cpu::CPU;
use memory::Memory;
use ppu::PPU;

use crate::{screen::Screen, utils};

type MemoryResult<T> = Result<T, MemoryError>;

#[derive(Debug, Clone)]
enum MemoryError {
    ReservedAddress,
    UnknownAddress,
    ReadOnly,
    WriteOnly,
}

impl std::error::Error for MemoryError {}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait MemoryInterface {
    fn read8(&self, addr: u16) -> MemoryResult<u8>;
    fn write8(&mut self, addr: u16, value: u8) -> MemoryResult<()>;
    fn read16(&self, addr: u16) -> MemoryResult<u16> {
        Ok(self.read8(addr)? as u16 + ((self.read8(addr + 1)? as u16) << 8))
    }
    fn write16(&mut self, addr: u16, value: u16) -> MemoryResult<()> {
        self.write8(addr, value as u8)?;
        self.write8(addr + 1, (value >> 8) as u8)?;
        Ok(())
    }
}

trait GameboyModule {
    fn tick(&mut self, memory: &mut Memory) -> Result<u32, Error>;
}

impl MemoryInterface for Gameboy {
    fn read8(&self, addr: u16) -> MemoryResult<u8> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            self.ppu.read8(addr)
        } else {
            self.memory.read8(addr)
        }
    }

    fn write8(&mut self, addr: u16, value: u8) -> MemoryResult<()> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            self.ppu.write8(addr, value)
        } else {
            self.memory.write8(addr, value)
        }
    }
}

pub struct Gameboy {
    cpu: CPU,
    ppu: PPU,
    screen: Screen,
    memory: Memory,
    running: bool,
}

impl Gameboy {
    pub fn new(bootrom_path: String, rom_path: String) -> Self {
        Self {
            cpu: CPU::new(),
            ppu: PPU::new(),
            screen: Screen::new(144, 160, 1, 1, minifb::Scale::X4),
            memory: Memory::new(bootrom_path, rom_path),
            running: true,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut prev = SystemTime::now();
        // let mem = utils::load_bytes("roms/mem_dump".into());
        // self.ppu.test_load_vram(mem.as_slice());
        while self.running {
            self.cpu.tick(&mut self.memory)?;
            let diff = SystemTime::now()
                .duration_since(prev)
                .expect("system time failed")
                .as_micros();
            if diff > 16742 {
                self.running = self.screen.update();
                // self.ppu.tick(&mut self.memory)?;
                log::info!(
                    "{:.2} fps",
                    1e6 / SystemTime::now()
                        .duration_since(prev)
                        .expect("system time failed")
                        .as_micros() as f32
                );
                prev = SystemTime::now();
            }
        }
        Ok(())
    }
}
