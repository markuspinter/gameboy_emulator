use core::time;
use std::{fmt::Error, thread};

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
}

trait GameboyModule {
    fn tick(&self, memory: &Memory) -> Result<u32, Error>;
}

pub struct Gameboy {
    cpu: CPU,
    ppu: PPU,
    memory: Memory,
    running: bool,
}

impl Gameboy {
    pub fn new(rom_path: String) -> Self {
        Self {
            cpu: CPU::new(),
            ppu: PPU::new(),
            memory: Memory::new(rom_path),
            running: true,
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        while self.running {
            println!("hi");
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }
}
