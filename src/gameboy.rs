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

use self::{joypad::Joypad, timer::Timer};

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
    fn read8(&self, addr: u16) -> Option<u8>;
    fn write8(&mut self, addr: u16, value: u8) -> Option<()>;
    fn read16(&self, addr: u16) -> Option<u16> {
        let high_opt = (self.read8(addr + 1));
        let low_opt = self.read8(addr);
        if let (Some(high), Some(low)) = (high_opt, low_opt) {
            Some((high as u16) << 8 | low as u16)
        } else {
            None
        }
    }
    fn write16(&mut self, addr: u16, value: u16) -> Option<()> {
        let high_opt = self.write8(addr + 1, (value >> 8) as u8);
        let low_opt = self.write8(addr, value as u8);
        if let (Some(high), Some(low)) = (high_opt, low_opt) {
            Some(())
        } else {
            None
        }
    }
}

trait GameboyModule {
    unsafe fn tick(&mut self, gb_ptr: *mut Gameboy) -> Result<u32, Error>;
}

impl Gameboy {
    fn read8(&self, addr: u16) -> u8 {
        if let Some(res) = self.ppu.read8(addr) {
            return res;
        }
        if let Some(res) = self.cpu.read8(addr) {
            return res;
        }
        if let Some(res) = self.joypad.read8(addr) {
            return res;
        }
        if let Some(res) = self.memory.read8(addr) {
            return res;
        }
        panic!("read8 address {:#06X} not found", addr);
    }

    fn write8(&mut self, addr: u16, value: u8) {
        if let Some(()) = self.ppu.write8(addr, value) {
            return;
        }
        if let Some(()) = self.cpu.write8(addr, value) {
            return;
        }
        if let Some(()) = self.joypad.write8(addr, value) {
            return;
        }
        if let Some(()) = self.memory.write8(addr, value) {
            return;
        }
        panic!("write8 address {:#06X} not found", addr);
    }

    fn read16(&self, addr: u16) -> u16 {
        let high = (self.read8(addr + 1));
        let low = self.read8(addr);
        (high as u16) << 8 | low as u16
    }
    fn write16(&mut self, addr: u16, value: u16) {
        self.write8(addr + 1, (value >> 8) as u8);
        self.write8(addr, value as u8);
    }
}

pub struct Gameboy {
    cpu: CPU,
    ppu: PPU,
    screen: Screen,
    memory: Memory,
    joypad: Joypad,
    timer: Timer,

    running: bool,
    cgb_mode: bool,
}

impl Gameboy {
    const TILE_DATA_ROWS: usize = 192;
    const TILE_DATA_COLUMNS: usize = 128;
    const SCREEN_ROWS: usize = 144;
    const SCREEN_COLUMNS: usize = 160;
    const TILE_MAP_ROWS: usize = 256;
    const TILE_MAP_COLUMNS: usize = 256;

    pub fn new(bootrom_path: String, rom_path: String) -> Self {
        Self {
            cpu: CPU::new(),
            ppu: PPU::new(),
            joypad: Joypad::new(),
            timer: Timer::new(),
            screen: Screen::new(Self::SCREEN_ROWS, Self::SCREEN_COLUMNS, 1, 1, minifb::Scale::X4),
            memory: Memory::new(bootrom_path, rom_path),
            running: true,
            cgb_mode: false,
        }
    }

    pub unsafe fn run(&mut self, debug_windows: bool) -> Result<(), Error> {
        let mut prev = SystemTime::now();
        let mut pause_pressed: bool;
        let mut paused: bool = false;

        let self_ptr = self as *mut Self;

        let mut tile_data_screen: Option<Screen> = None;
        if debug_windows {
            tile_data_screen = Some(Screen::new(
                Self::TILE_DATA_ROWS,
                Self::TILE_DATA_COLUMNS,
                1,
                1,
                minifb::Scale::X4,
            ));
        }

        while self.running {
            if !paused {
                self.cpu.tick(self_ptr)?;
                self.ppu.tick(self_ptr)?;
                self.timer.tick(self_ptr)?;
            }

            let diff = SystemTime::now()
                .duration_since(prev)
                .expect("system time failed")
                .as_micros();
            if diff > 16742 {
                //16742 {
                //59.720 fps = 16742 us {
                if let Some(ref mut screen) = tile_data_screen {
                    self.ppu.process_tile_data();
                    screen.set_frame_buffer(&self.ppu.get_tile_data_frame_buffer(16));
                    screen.update();
                }

                self.screen.set_frame_buffer(&self.ppu.get_frame_buffer());
                (self.running, pause_pressed) = self.screen.update();
                self.joypad.tick(self_ptr)?;

                if pause_pressed {
                    paused = !paused;
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
        let mut shall_pause: bool = false;

        while self.running {
            if !shall_pause {
                self.cpu.tick(self_ptr)?;
                self.ppu.tick(self_ptr)?;
                self.timer.tick(self_ptr)?;
            }

            // self.screen.set_frame_buffer(&self.ppu.get_tile_data_frame_buffer(16));

            let diff = SystemTime::now()
                .duration_since(prev)
                .expect("system time failed")
                .as_micros();
            if diff > 33333 {
                // self.ppu.tick(self_ptr)?;
                self.screen.set_frame_buffer(&self.ppu.get_frame_buffer());
                // if draw_bg {
                //     self.screen.set_frame_buffer(&self.ppu.get_bg_frame_buffer());
                // } else {
                //     // self.screen.set_frame_buffer(&self.ppu.get_window_frame_buffer());
                //     self.screen.set_frame_buffer(&self.ppu.get_objects_frame_buffer());
                // }
                (self.running, shall_pause) = self.screen.update();
                //59.720 fps = 16742 us {
                log::info!(
                    "{:.2} fps",
                    1e6 / SystemTime::now()
                        .duration_since(prev)
                        .expect("system time failed")
                        .as_micros() as f32
                );
                prev = SystemTime::now();
                // draw_bg = !draw_bg;
            }
        }
        Ok(())
    }

    pub fn switch_speed(&self) {
        panic!("switch speed not implemented");
    }

    pub fn get_keys(&mut self) -> &Vec<Key> {
        &self.screen.get_keys()
    }
}
