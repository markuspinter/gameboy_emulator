mod lcdc;
mod palette;
mod sprite;
mod stat;

use crate::{bit, gameboy::memory, screen::MonochromeColor, utils};
use colored::Colorize;
use log::warn;

use self::lcdc::LCDControl;

use super::{memory::MemoryRange, Gameboy, GameboyModule, MemoryInterface};

pub struct PPU {
    frame_buffer: [u32; Self::ROWS * Self::COLUMNS],
    vram: [u8; memory::ppu::VRAM.size],
    oam: [u8; memory::ppu::OAM.size],
    tiles: [[[u8; Self::TILE_SIZE]; Self::TILE_SIZE]; Self::TILES],
    lcdc: lcdc::LCDControl,
    stat: stat::LCDStatus,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: palette::PaletteData,
    obp0: palette::PaletteData,
    obp1: palette::PaletteData,
    wy: u8,
    wx: u8,
}

impl GameboyModule for PPU {
    unsafe fn tick(&mut self, gb_ptr: *mut Gameboy) -> Result<u32, std::fmt::Error> {
        self.process_tile_data();
        // self.print_tiles(10);
        Ok((0))
    }
}

impl super::MemoryInterface for PPU {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            return Some(self.vram[usize::from(addr - memory::ppu::VRAM.begin)]);
        } else if addr >= memory::ppu::OAM.begin && addr <= memory::ppu::OAM.end {
            return Some(self.oam[usize::from(addr - memory::ppu::OAM.begin)]);
        } else if addr == memory::ppu::LCDC {
            return Some(self.lcdc.clone().into());
        } else if addr == memory::ppu::STAT {
            return Some(self.stat.clone().into());
        } else if addr == memory::ppu::SCY {
            return Some(self.scy);
        } else if addr == memory::ppu::SCX {
            return Some(self.scx);
        } else if addr == memory::ppu::LY {
            return Some(self.ly);
        } else if addr == memory::ppu::LYC {
            return Some(self.lyc);
        } else if addr == memory::ppu::DMA {
            return Some(self.dma);
        } else if addr == memory::ppu::BGP {
            return Some(self.bgp.clone().into());
        } else if addr == memory::ppu::OBP0 {
            return Some(self.obp0.clone().into());
        } else if addr == memory::ppu::OBP1 {
            return Some(self.obp1.clone().into());
        } else if addr == memory::ppu::WY {
            return Some(self.wy);
        } else if addr == memory::ppu::WX {
            return Some(self.wx + 7);
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            self.vram[usize::from(addr - memory::ppu::VRAM.begin)] = value;
        } else if addr >= memory::ppu::OAM.begin && addr <= memory::ppu::OAM.end {
            self.oam[usize::from(addr - memory::ppu::OAM.begin)] = value;
        } else if addr == memory::ppu::LCDC {
            self.lcdc = value.into();
        } else if addr == memory::ppu::STAT {
            self.stat = value.into();
        } else if addr == memory::ppu::SCY {
            self.scy = value;
        } else if addr == memory::ppu::SCX {
            self.scx = value;
        } else if addr == memory::ppu::LY {
            warn!("LY is read only at address {:#06x}, ignoring write", addr);
        } else if addr == memory::ppu::LYC {
            self.lyc = value;
        } else if addr == memory::ppu::DMA {
            self.dma = value;
            //TODO: start dma routine and prohibit memory access execept for hram
        } else if addr == memory::ppu::BGP {
            self.bgp = value.into();
        } else if addr == memory::ppu::OBP0 {
            self.obp0 = value.into();
        } else if addr == memory::ppu::OBP1 {
            self.obp1 = value.into();
        } else if addr == memory::ppu::WY {
            self.wy = value;
        } else if addr == memory::ppu::WX {
            self.wx = value - 7;
        } else {
            return None;
        }
        return Some(());
    }
}

impl PPU {
    const ROWS: usize = 160;
    const COLUMNS: usize = 144;
    const TILES: usize = 0x180; //sanity check: 0x97FF+1 - 0x8000 / 16
    const TILE_SIZE: usize = 8; //this is one line i.e. size*size=total pixels
    const BYTES_PER_TILE: usize = 16;
    const TILE_MAP_SIZE: usize = 32; //this is one line of tiles i.e. size*size=total tiles
    const TILE_MAP_AREA_9800: MemoryRange = MemoryRange {
        begin: 0x9800,
        end: 0x9BFF,
        size: 0x400,
    };
    const TILE_MAP_AREA_9C00: MemoryRange = MemoryRange {
        begin: 0x9C00,
        end: 0x9FFF,
        size: 0x400,
    };
    const TILE_MAP_AREA_9800_VRAM: MemoryRange = MemoryRange {
        begin: 0x1800,
        end: 0x1BFF,
        size: 0x400,
    };
    const TILE_MAP_AREA_9C00_VRAM: MemoryRange = MemoryRange {
        begin: 0x1C00,
        end: 0x1FFF,
        size: 0x400,
    };

    pub fn new() -> Self {
        let ppu = Self {
            frame_buffer: [0; Self::ROWS * Self::COLUMNS],
            vram: [0; memory::ppu::VRAM.size],
            oam: [0; memory::ppu::OAM.size],
            tiles: [[[0; Self::TILE_SIZE]; Self::TILE_SIZE]; Self::TILES],
            lcdc: lcdc::LCDControl::from(0),
            stat: stat::LCDStatus::from(0),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: palette::PaletteData::from(0),
            obp0: palette::PaletteData::from(0),
            obp1: palette::PaletteData::from(0),
            wy: 0,
            wx: 0,
        };

        ppu
    }

    fn process_tile_data(&mut self) {
        let tile_data =
            &self.vram[memory::ppu::TILE_DATA_VRAM.begin as usize..=memory::ppu::TILE_DATA_VRAM.end as usize];
        for addr in (0..Self::TILES * Self::BYTES_PER_TILE).step_by(Self::BYTES_PER_TILE) {
            let tile_id: usize = addr / (Self::BYTES_PER_TILE);
            let mut tile: [[u8; 8]; 8] = [[0; Self::TILE_SIZE]; Self::TILE_SIZE];
            for line in (0..Self::BYTES_PER_TILE).step_by(2) {
                let byte1: u8 = tile_data[addr + line];
                let byte2: u8 = tile_data[addr + line + 1];

                //pixel conversion
                let mut line_pixels: [u8; Self::TILE_SIZE] = [0; Self::TILE_SIZE];
                for i in (0..=7).rev() {
                    line_pixels[7 - i] = bit!(byte2, i) << 1 | bit!(byte1, i);
                }

                // println!("{:?}", line_pixels);
                tile[line >> 1] = line_pixels;
            }
            self.tiles[tile_id] = tile;
        }
    }

    fn process_tile(&mut self, addr: u16) {}

    pub fn get_tile_data_frame_buffer(
        &self,
        wrap_count: usize,
    ) -> [u32; Self::TILES * (Self::TILE_SIZE * Self::TILE_SIZE)] {
        let mut frame_buffer: [u32; Self::TILES * (Self::TILE_SIZE * Self::TILE_SIZE)] =
            [0; Self::TILES * (Self::TILE_SIZE * Self::TILE_SIZE)];

        for row in 0..(self.tiles.len() * Self::TILE_SIZE / wrap_count) {
            let curr_tile_start: usize = (row / Self::TILE_SIZE) * wrap_count;
            for tile_index in curr_tile_start..curr_tile_start + wrap_count {
                let tile_line: [u32; Self::TILE_SIZE] = (self.tiles[tile_index][row % Self::TILE_SIZE])
                    .iter()
                    .map(|pixel| match pixel {
                        0 => MonochromeColor::White as u32,
                        1 => MonochromeColor::LightGray as u32,
                        2 => MonochromeColor::DarkGray as u32,
                        3 => MonochromeColor::Black as u32,
                        _ => MonochromeColor::Off as u32,
                    })
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap();

                let fb_start = (tile_index % wrap_count) * Self::TILE_SIZE + (row * wrap_count * Self::TILE_SIZE);
                frame_buffer[fb_start..fb_start + Self::TILE_SIZE].copy_from_slice(tile_line.as_slice());
            }
            println!();
        }

        frame_buffer
    }

    fn get_tiles_from_tile_map(
        &self,
        tile_map_start: u16,
    ) -> [&[[u8; 8]; 8]; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE] {
        let mut map_tiles: [&[[u8; 8]; 8]; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE] =
            [&[[0; 8]; 8]; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE];

        for addr in tile_map_start..tile_map_start + 0x0400 {
            let mut tile_id = self.read8(addr).unwrap();
            if self.lcdc.bg_and_window_tile_data_area {
                tile_id = tile_id.wrapping_sub(128);
            }

            map_tiles[addr as usize - tile_map_start as usize] = &self.tiles[tile_id as usize];
        }
        map_tiles
    }

    pub fn get_bg_frame_buffer(
        &self,
    ) -> [u32; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)] {
        let tile_map_start = if self.lcdc.bg_tile_map_area {
            Self::TILE_MAP_AREA_9C00.begin
        } else {
            Self::TILE_MAP_AREA_9800.begin
        };

        self.get_tile_map_frame_buffer(self.get_tiles_from_tile_map(tile_map_start))
    }

    pub fn get_window_frame_buffer(
        &self,
    ) -> [u32; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)] {
        let tile_map_start = if self.lcdc.window_tile_map_area {
            Self::TILE_MAP_AREA_9C00.begin
        } else {
            Self::TILE_MAP_AREA_9800.begin
        };

        self.get_tile_map_frame_buffer(self.get_tiles_from_tile_map(tile_map_start))
    }

    fn get_tile_map_frame_buffer(
        &self,
        map_tiles: [&[[u8; 8]; 8]; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE],
    ) -> [u32; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)] {
        let mut frame_buffer: [u32; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)] =
            [0; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)];

        for row in 0..(Self::TILE_MAP_SIZE * Self::TILE_SIZE) {
            let curr_tile_start: usize = (row / Self::TILE_SIZE) * Self::TILE_MAP_SIZE;
            for tile_index in curr_tile_start..curr_tile_start + Self::TILE_MAP_SIZE {
                let tile_line: [u32; Self::TILE_SIZE] = (map_tiles[tile_index][row % Self::TILE_SIZE])
                    .iter()
                    .map(|pixel| match pixel {
                        0 => MonochromeColor::White as u32,
                        1 => MonochromeColor::LightGray as u32,
                        2 => MonochromeColor::DarkGray as u32,
                        3 => MonochromeColor::Black as u32,
                        _ => MonochromeColor::Off as u32,
                    })
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap();

                let fb_start = (tile_index % Self::TILE_MAP_SIZE) * Self::TILE_SIZE
                    + (row * Self::TILE_MAP_SIZE * Self::TILE_SIZE);
                frame_buffer[fb_start..fb_start + Self::TILE_SIZE].copy_from_slice(tile_line.as_slice());
            }
        }

        frame_buffer
    }

    pub fn get_objects_frame_buffer(
        &self,
    ) -> [u32; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)] {
        let mut frame_buffer: [u32; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)] =
            [0; Self::TILE_MAP_SIZE * Self::TILE_MAP_SIZE * (Self::TILE_SIZE * Self::TILE_SIZE)];
        //TODO: if LCDC bit 2: 1 -> 2 tile objects
        for addr in (0..0x00A0).step_by(4) {
            // print(self.vram[addr])
            let entry = sprite::OAMTableEntry::new(&self.oam, addr);
            let curr_tile = self.tiles[entry.tile_index as usize];

            if entry.x_pos <= 0 || entry.x_pos >= 168 || entry.y_pos <= 0 || entry.y_pos >= 160 {
                log::info!("sprite is offscreen");
            } else {
                for row in entry.y_pos as usize..entry.y_pos as usize + Self::TILE_SIZE {
                    for col in entry.x_pos as usize..entry.x_pos as usize + Self::TILE_SIZE {
                        frame_buffer[row * Self::TILE_MAP_SIZE * Self::TILE_SIZE + col] =
                            match curr_tile[row - entry.y_pos as usize][col - entry.x_pos as usize] {
                                0 => MonochromeColor::White as u32,
                                1 => MonochromeColor::LightGray as u32,
                                2 => MonochromeColor::DarkGray as u32,
                                3 => MonochromeColor::Black as u32,
                                _ => MonochromeColor::Off as u32,
                            };
                    }
                }
            }
        }
        frame_buffer
    }

    pub fn get_frame_buffer(&mut self) -> &[u32] {
        &self.frame_buffer
    }

    pub fn test_load_memory(&mut self, mem: &[u8]) {
        self.vram[..memory::ppu::VRAM.size]
            .clone_from_slice(mem[memory::ppu::VRAM.begin as usize..=memory::ppu::VRAM.end as usize].into());
        self.oam[..memory::ppu::OAM.size]
            .clone_from_slice(mem[memory::ppu::OAM.begin as usize..=memory::ppu::OAM.end as usize].into());

        self.lcdc = LCDControl::from(mem[memory::ppu::LCDC as usize]);
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
                        _ => "X".truecolor(0, 0, 0), //string.truecolor(0, 0, 0),
                    };

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
