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
use minifb::Key;
use ppu::PPU;

use crate::{screen::Screen, utils};

use self::joypad::Joypad;

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
    unsafe fn tick(&mut self, gb_ptr: *mut Gameboy) -> Result<u32, Error>;
}

impl MemoryInterface for Gameboy {
    fn read8(&self, addr: u16) -> MemoryResult<u8> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            log::trace!("reading from vram {:#06X}", addr);
            self.ppu.read8(addr)
        } else if addr >= memory::ppu::OAM.begin && addr <= memory::ppu::OAM.end {
            log::trace!("reading from oam {:#06X}", addr);
            self.ppu.read8(addr)
        } else {
            log::trace!("reading to memory {:#06X}", addr);
            self.memory.read8(addr)
        }
    }

    fn write8(&mut self, addr: u16, value: u8) -> MemoryResult<()> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            log::trace!("writing to vram {:#06X}: {:#04X}", addr, value);
            self.ppu.write8(addr, value)
        } else if addr >= memory::ppu::OAM.begin && addr <= memory::ppu::OAM.end {
            log::trace!("writing to oam {:#06X}: {:#04X}", addr, value);
            self.ppu.write8(addr, value)
        } else {
            log::trace!("writing to memory {:#06X}: {:#04X}", addr, value);
            self.memory.write8(addr, value)
        }
    }
}

pub struct Gameboy {
    cpu: CPU,
    ppu: PPU,
    screen: Screen,
    memory: Memory,
    joypad: Joypad,
    running: bool,
    cgb_mode: bool,
}

impl Gameboy {
    const TILE_DATA_ROWS: usize = 192;
    const TILE_DATA_COLUMNS: usize = 128;
    const TILE_MAP_ROWS: usize = 256;
    const TILE_MAP_COLUMNS: usize = 256;

    pub fn new(bootrom_path: String, rom_path: String) -> Self {
        Self {
            cpu: CPU::new(),
            ppu: PPU::new(),
            joypad: Joypad::new(),
            screen: Screen::new(Self::TILE_MAP_ROWS, Self::TILE_MAP_COLUMNS, 1, 1, minifb::Scale::X4),
            memory: Memory::new(bootrom_path, rom_path),
            running: true,
            cgb_mode: false,
        }
    }

    pub unsafe fn run(&mut self) -> Result<(), Error> {
        let mut prev = SystemTime::now();
        let mut shall_print_status: bool;

        let self_ptr = self as *mut Self;

        while self.running {
            self.cpu.tick(self_ptr)?;
            self.joypad.tick(self_ptr)?;

            let diff = SystemTime::now()
                .duration_since(prev)
                .expect("system time failed")
                .as_micros();
            if diff > 33333 {
                //16742 {
                //59.720 fps = 16742 us {
                self.ppu.tick(self_ptr)?;
                self.screen.set_frame_buffer(&self.ppu.get_bg_frame_buffer());
                (self.running, shall_print_status) = self.screen.update();
                if shall_print_status {
                    println!("{:?}", self.cpu);
                }
                prev = SystemTime::now();
            }
        }
        Ok(())
    }

    pub unsafe fn test_run(&mut self) -> Result<(), Error> {
        let mut prev = SystemTime::now();

        let mem = utils::load_bytes("roms/mem_dump".into());
        self.ppu.test_load_memory(mem.as_slice());

        let self_ptr = self as *mut Self;
        self.ppu.tick(self_ptr)?;

        self.ppu.print_tiles(0x10);

        let mut draw_bg: bool = true;

        while self.running {
            self.cpu.tick(self_ptr)?;
            self.ppu.tick(self_ptr)?;
            if draw_bg {
                self.screen.set_frame_buffer(&self.ppu.get_bg_frame_buffer());
            } else {
                // self.screen.set_frame_buffer(&self.ppu.get_window_frame_buffer());
                self.screen.set_frame_buffer(&self.ppu.get_objects_frame_buffer());
            }
            // self.screen.set_frame_buffer(&self.ppu.get_tile_data_frame_buffer(16));
            self.running = self.screen.update().1;
            let diff = SystemTime::now()
                .duration_since(prev)
                .expect("system time failed")
                .as_micros();
            if diff > 1e6 as u128 {
                //59.720 fps = 16742 us {
                log::info!(
                    "{:.2} fps",
                    1e6 / SystemTime::now()
                        .duration_since(prev)
                        .expect("system time failed")
                        .as_micros() as f32
                );
                prev = SystemTime::now();
                draw_bg = !draw_bg;
            }
        }
        Ok(())
    }

    pub fn switch_speed(&self) {
        panic!("switch speed not implemented");
    }

    pub fn get_keys(&self) -> Vec<Key> {
        self.screen.get_keys()
    }
}
