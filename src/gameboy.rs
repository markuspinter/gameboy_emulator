use std::{fmt::Error, time::SystemTime};

pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod interrupts;
pub mod joypad;
pub mod memory;
pub mod ppu;
pub mod timer;

use apu::APU;
use cpu::CPU;
use memory::Memory;
use minifb::Key;
use ppu::PPU;

use crate::screen::Screen;

use self::{cartridge::Cartridge, joypad::Joypad, timer::Timer};

trait MemoryInterface {
    fn read8(&self, addr: u16) -> Option<u8>;
    fn write8(&mut self, addr: u16, value: u8) -> Option<()>;
    fn read16(&self, addr: u16) -> Option<u16> {
        let high_opt = self.read8(addr + 1);
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
        if let (Some(_high), Some(_low)) = (high_opt, low_opt) {
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
        if self.dma_active {
            if addr < memory::HRAM.begin || addr > memory::HRAM.end {
                log::warn!(
                    "trying to access address {:#06X} during dma transfer, returning 0xFF",
                    addr
                );
                return 0xFF;
            }
        }
        if let Some(res) = self.apu.read8(addr) {
            return res;
        }
        if let Some(res) = self.ppu.read8(addr) {
            return res;
        }
        if let Some(res) = self.cpu.read8(addr) {
            return res;
        }
        if let Some(res) = self.joypad.read8(addr) {
            return res;
        }
        if let Some(res) = self.timer.read8(addr) {
            return res;
        }
        if let Some(res) = self.cartridge.read8(addr) {
            return res;
        }
        if let Some(res) = self.memory.read8(addr) {
            return res;
        }
        panic!("read8 address {:#06X} not found", addr);
    }

    fn write8(&mut self, addr: u16, value: u8) {
        if let Some(()) = self.apu.write8(addr, value) {
            return;
        }
        if let Some(()) = self.ppu.write8(addr, value) {
            return;
        }
        if let Some(()) = self.cpu.write8(addr, value) {
            return;
        }
        if let Some(()) = self.joypad.write8(addr, value) {
            return;
        }
        if let Some(()) = self.timer.write8(addr, value) {
            return;
        }
        if let Some(()) = self.cartridge.write8(addr, value) {
            return;
        }
        if let Some(()) = self.memory.write8(addr, value) {
            return;
        }
        panic!("write8 address {:#06X} not found", addr);
    }

    fn read8_unlocked(&self, addr: u16) -> u8 {
        if let Some(res) = self.apu.read8(addr) {
            return res;
        }
        if let Some(res) = self.ppu.read8(addr) {
            return res;
        }
        if let Some(res) = self.cpu.read8(addr) {
            return res;
        }
        if let Some(res) = self.joypad.read8(addr) {
            return res;
        }
        if let Some(res) = self.timer.read8(addr) {
            return res;
        }
        if let Some(res) = self.cartridge.read8(addr) {
            return res;
        }
        if let Some(res) = self.memory.read8(addr) {
            return res;
        }
        panic!("read8_unlocked address {:#06X} not found", addr);
    }

    fn _read16(&self, addr: u16) -> u16 {
        let high = self.read8(addr + 1);
        let low = self.read8(addr);
        (high as u16) << 8 | low as u16
    }
    fn _write16(&mut self, addr: u16, value: u16) {
        self.write8(addr + 1, (value >> 8) as u8);
        self.write8(addr, value as u8);
    }
}

pub struct Gameboy {
    cartridge: Cartridge,
    cpu: CPU,
    ppu: PPU,
    screen: Screen,
    apu: APU,
    memory: Memory,
    joypad: Joypad,
    timer: Timer,

    dma_active: bool,

    running: bool,
    _cgb_mode: bool,

    pub vblank: bool,
}

impl Gameboy {
    const TILE_DATA_ROWS: usize = 192;
    const TILE_DATA_COLUMNS: usize = 128;
    const SCREEN_ROWS: usize = 144;
    const SCREEN_COLUMNS: usize = 160;
    const TILE_MAP_ROWS: usize = 256;
    const TILE_MAP_COLUMNS: usize = 256;

    pub fn new(bootrom_path: String, rom_path: String) -> Self {
        let gb = Self {
            cartridge: Cartridge::new(bootrom_path.clone(), rom_path.clone()),
            cpu: CPU::new(),
            ppu: PPU::new(),
            joypad: Joypad::new(),
            timer: Timer::new(),
            screen: Screen::new(Self::SCREEN_ROWS, Self::SCREEN_COLUMNS, minifb::Scale::X4),
            apu: APU::new(),
            memory: Memory::new(),

            dma_active: false,

            running: true,
            _cgb_mode: false,
            vblank: false,
        };
        gb.cartridge.debug_print();
        gb
    }

    pub unsafe fn run(&mut self, debug_windows: bool) -> Result<(), Error> {
        let mut prev = SystemTime::now();
        let mut pause_pressed: bool;
        let mut paused: bool = false;

        let self_ptr = self as *mut Self;

        let mut tile_data_screen: Option<Screen> = None;
        let mut tile_map_screen: Option<Screen> = None;
        if debug_windows {
            tile_data_screen = Some(Screen::new(
                Self::TILE_DATA_ROWS,
                Self::TILE_DATA_COLUMNS,
                minifb::Scale::X4,
            ));

            tile_map_screen = Some(Screen::new(
                Self::TILE_MAP_ROWS,
                Self::TILE_MAP_COLUMNS,
                minifb::Scale::X4,
            ));
        }
        let mut debug_counter = 0;

        let mut frame_ready = false;

        while self.running {
            if debug_windows {
                self.cpu.tick(self_ptr)?;
                self.ppu.tick(self_ptr)?;
                self.timer.tick(self_ptr)?;

                if paused {
                    std::process::Command::new("clear").status().unwrap();
                    self.ppu.print_state_machine();
                }
            } else if !paused {
                // if ticks < 70224 {
                self.cpu.tick(self_ptr)?;
                self.ppu.tick(self_ptr)?;
                self.timer.tick(self_ptr)?;
                self.apu.tick(self_ptr)?;
                // }
            }
            if let Some(frame_buffer) = self.ppu.get_frame_buffer() {
                frame_ready = true;
                self.screen.set_frame_buffer(frame_buffer);
            }
            if frame_ready {
                // println!("frame ready");
                if debug_windows {
                    debug_counter += 1;
                    if debug_counter >= 60 {
                        self.ppu.process_tile_data();

                        if let Some(ref mut screen) = tile_data_screen {
                            screen.set_frame_buffer(&self.ppu.get_tile_data_frame_buffer(16));
                            screen.update();
                        }
                        if let Some(ref mut screen) = tile_map_screen {
                            screen.set_frame_buffer(&self.ppu.get_bg_frame_buffer());
                            screen.update();
                        }
                        debug_counter = 0;
                    }
                }

                (self.running, pause_pressed) = self.screen.update();

                self.joypad.tick(self_ptr)?;

                if pause_pressed {
                    // paused = !paused;
                    self.apu.shall_clear_audio_queue = !self.apu.shall_clear_audio_queue;
                }

                let mut diff = SystemTime::now()
                    .duration_since(prev)
                    .expect("system time failed")
                    .as_micros();

                if diff < 16742 {
                    //16742 {
                    //59.720 fps = 16742 us {

                    std::thread::sleep(std::time::Duration::from_micros(16742 - diff as u64));
                } else {
                    log::warn!("frame time: {}us, sleeping {}us", diff, 16742 as i128 - diff as i128);
                    log::warn!("skipped one frame, clearing audio buffer");
                    // self.apu.shall_clear_audio_queue = true; // clear audio queue since skipped one frame
                }

                diff = SystemTime::now()
                    .duration_since(prev)
                    .expect("system time failed")
                    .as_nanos();
                prev = SystemTime::now();
                self.apu.sync(self_ptr, diff);
                diff = SystemTime::now()
                    .duration_since(prev)
                    .expect("system time failed")
                    .as_micros();
                // log::warn!("audio sync took {}us", diff);

                prev = SystemTime::now();
                frame_ready = false;
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
