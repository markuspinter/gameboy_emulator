mod types;

use crate::{gameboy::memory, utils};

const ROWS: usize = 160;
const COLUMNS: usize = 144;

pub enum MonochromeColor {
    Off = 0x00CADC9F,
    White = 0x009BBC0F,
    LightGray = 0x008BAC0F,
    DarkGray = 0x00306230,
    Black = 0x000F380F,
}

pub struct PPU {
    frame_buffer: [u32; ROWS * COLUMNS],
    vram: [u8; memory::ppu::VRAM.size],
}

impl super::MemoryInterface for PPU {
    fn read8(&self, addr: u16) -> super::MemoryResult<u8> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            return Ok(self.vram[usize::from(addr)]);
        }
        return Err(super::MemoryError::UnknownAddress);
    }

    fn write8(&mut self, addr: u16, value: u8) -> super::MemoryResult<()> {
        self.vram[usize::from(addr)] = value;
        Ok(())
    }
}

impl PPU {
    pub fn new() -> Self {
        let vram: [u8; memory::ppu::VRAM.size] = [0; memory::ppu::VRAM.size];
        let mut ppu = Self {
            frame_buffer: [0; ROWS * COLUMNS],
            vram: vram,
        };

        ppu
    }

    pub fn get_frame_buffer(&mut self) -> &[u32] {
        &self.frame_buffer
    }

    pub fn print_vram(self) {
        utils::print_memory_bytes(&self.vram, "vram", 0x100);
    }
}
