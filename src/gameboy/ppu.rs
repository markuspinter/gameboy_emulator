mod lcdc;
mod palette;
mod sprite;
mod stat;

use std::fmt::format;

use crate::{bit, gameboy::memory, utils};
use colored::Colorize;

use super::{memory::Memory, GameboyModule};

pub struct PPU {
    frame_buffer: [u32; Self::ROWS * Self::COLUMNS],
    vram: [u8; memory::ppu::VRAM.size],
    tiles: [[[u8; Self::TILE_SIZE]; Self::TILE_SIZE]; Self::TILES],
}

impl GameboyModule for PPU {
    fn tick(&mut self, memory: &mut Memory) -> Result<u32, std::fmt::Error> {
        self.process_tile_data();
        self.print_tiles(10);
        Ok((0))
    }
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
    const ROWS: usize = 160;
    const COLUMNS: usize = 144;
    const TILES: usize = 0x180; //sanity check: 0x97FF+1 - 0x8000 / 16
    const TILE_SIZE: usize = 8;
    const BYTES_PER_TILE: usize = 16;

    pub fn new() -> Self {
        let vram: [u8; memory::ppu::VRAM.size] = [0; memory::ppu::VRAM.size];
        let ppu = Self {
            frame_buffer: [0; Self::ROWS * Self::COLUMNS],
            vram: vram,
            tiles: [[[0; Self::TILE_SIZE]; Self::TILE_SIZE]; Self::TILES],
        };

        ppu
    }

    fn process_tile_data(&mut self) {
        let tile_data = &self.vram
            [memory::ppu::TILE_DATA_VRAM.begin as usize..=memory::ppu::TILE_DATA_VRAM.end as usize];
        for addr in (0..Self::TILES * Self::BYTES_PER_TILE).step_by(Self::BYTES_PER_TILE) {
            let tile_id: usize = addr / (Self::BYTES_PER_TILE);
            let mut tile: [[u8; 8]; 8] = [[0; Self::TILE_SIZE]; Self::TILE_SIZE];
            for line in (0..Self::BYTES_PER_TILE).step_by(2) {
                let byte1: u8 = tile_data[addr + line];
                let byte2: u8 = tile_data[addr + line + 1];

                //pixel conversion
                let mut line_pixels: [u8; Self::TILE_SIZE] = [0; Self::TILE_SIZE];
                for i in (0..=7).rev() {
                    line_pixels[i] = bit!(byte2, i) << 1 | bit!(byte1, i);
                }

                // println!("{:?}", line_pixels);
                tile[line >> 1] = line_pixels;
            }
            self.tiles[tile_id] = tile;
        }
    }

    fn process_tile(&mut self, addr: u16) {}

    pub fn get_frame_buffer(&mut self) -> &[u32] {
        &self.frame_buffer
    }

    pub fn test_load_vram(&mut self, mem: &[u8]) {
        self.vram[..memory::ppu::VRAM.size].clone_from_slice(
            mem[memory::ppu::VRAM.begin as usize..=memory::ppu::VRAM.end as usize].into(),
        );
    }

    pub fn print_vram(&self) {
        utils::print_memory_bytes(&self.vram, "vram", 0x100);
    }

    pub fn print_tiles(&self, count: usize) {
        for (i, tile) in self.tiles.iter().enumerate() {
            // let pixel_color = "\u{25A0}";
            println!("Tile {}: ", i);

            for line in tile {
                for pixel in line {
                    let string = match pixel {
                        0 => "0".truecolor(0x9B, 0xBC, 0x0F),
                        1 => "1".truecolor(0x8B, 0xAC, 0x0F),
                        2 => "2".truecolor(0x30, 0x62, 0x30),
                        3 => "3".truecolor(0x0F, 0x38, 0x0F),
                        _ => "X".truecolor(127, 0, 127), //string.truecolor(0, 0, 0),
                    };
                    // pub enum MonochromeColor {
                    //     Off = 0x00CADC9F,
                    //     White = 0x009BBC0F,
                    //     LightGray = 0x008BAC0F,
                    //     DarkGray = 0x00306230,
                    //     Black = 0x000F380F,
                    // }

                    print!("{}", string);
                }
                println!();
            }

            println!();
            if i >= count {
                break;
            }
        }
    }
}
