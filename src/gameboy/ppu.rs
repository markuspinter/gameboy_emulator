mod fetcher;
mod fifo;
mod lcdc;
mod palette;
mod sprite;
mod stat;

use crate::{bit, gameboy::memory, screen::MonochromeColor, utils};
use colored::Colorize;

use self::{
    fetcher::Fetcher,
    fifo::Fifo,
    lcdc::LCDControl,
    palette::PaletteData,
    stat::{LCDModeFlag, LCDStatus},
};

use super::{Gameboy, GameboyModule};

pub struct PPU {
    frame_buffer: [u32; PPU::ROWS * PPU::COLUMNS],
    back_buffer: [u32; PPU::ROWS * PPU::COLUMNS],
    back_buffer_index: usize,
    vram: [u8; memory::ppu::VRAM.size],
    oam: [u8; memory::ppu::OAM.size],
    lcdc: lcdc::LCDControl,
    stat: stat::LCDStatus,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    dma_cycles: u8,
    bgp: palette::PaletteData,
    obp0: palette::PaletteData,
    obp1: palette::PaletteData,
    wy: u8,
    wx: u8,

    fetcher: Fetcher,
    fifo: Fifo,
    dots: u16,

    frame_ready: bool,

    ppu_debug: PPUDebug,
}

impl GameboyModule for PPU {
    unsafe fn tick(&mut self, gb_ptr: *mut Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;

        if self.dma_cycles > 0 {
            gb.dma_active = true;
            self.handle_dma(gb);
        } else {
            gb.dma_active = false;
        }
        if self.lcdc.lcd_ppu_enable {
            self.handle_int(gb);

            match self.stat.mode_flag {
                LCDModeFlag::HBlank => self.handle_hblank(gb),
                LCDModeFlag::VBlank => self.handle_vblank(gb),
                LCDModeFlag::SearchingOAM => self.handle_oam_search(),
                LCDModeFlag::TransferringDataToLCD => {
                    self.fetcher.tick(gb_ptr)?;
                    let popped = self.fifo.tick(gb_ptr)?;
                    self.handle_pixel_transfer(gb, popped);
                }
            }
            if self.dots > 0 && !matches!(self.stat.mode_flag, LCDModeFlag::TransferringDataToLCD) {
                self.dots -= 1;
            }
        }

        Ok(0)
    }
}

impl super::MemoryInterface for PPU {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            if matches!(self.stat.mode_flag, LCDModeFlag::TransferringDataToLCD) {
                log::warn!(
                    "VRAM is inaccessible during mode 3; address {:#06x}, returning garbage (0xFF)",
                    addr
                );
                return Some(0xFF);
            }
            return Some(self.vram[usize::from(addr - memory::ppu::VRAM.begin)]);
        } else if addr >= memory::ppu::OAM.begin && addr <= memory::ppu::OAM.end {
            if matches!(self.stat.mode_flag, LCDModeFlag::SearchingOAM)
                || matches!(self.stat.mode_flag, LCDModeFlag::TransferringDataToLCD)
            {
                log::warn!(
                    "OAM is inaccessible during mode 2 and 3 (currently mode {}); address {:#06x}, returning garbage (0xFF)",
                    self.stat.mode_flag as u8,
                    addr
                );
                return Some(0xFF);
            }
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
            return Some(self.wx);
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            if matches!(self.stat.mode_flag, LCDModeFlag::TransferringDataToLCD) {
                log::warn!(
                    "VRAM is inaccessible during mode 3; address {:#06x}, ignoring write",
                    addr
                );
                return Some(());
            }
            self.vram[usize::from(addr - memory::ppu::VRAM.begin)] = value;
        } else if addr >= memory::ppu::OAM.begin && addr <= memory::ppu::OAM.end {
            if matches!(self.stat.mode_flag, LCDModeFlag::SearchingOAM)
                || matches!(self.stat.mode_flag, LCDModeFlag::TransferringDataToLCD)
            {
                log::warn!(
                    "OAM is inaccessible during mode 2 and 3 (currently mode {}); address {:#06x}, ignoring write",
                    self.stat.mode_flag as u8,
                    addr
                );
                return Some(());
            }
            self.oam[usize::from(addr - memory::ppu::OAM.begin)] = value;
        } else if addr == memory::ppu::LCDC {
            log::info!("lcdc changed: {:#010b}", u8::from(value.clone()));
            self.lcdc = value.into();
            if !self.lcdc.lcd_ppu_enable {
                self.back_buffer_index = 0;
                self.frame_ready = false;
                self.dots = 0;
                self.ly = 144;
                self.stat.mode_flag = LCDModeFlag::VBlank;
                self.fifo.reset();
                self.fetcher.reset();
            }
        } else if addr == memory::ppu::STAT {
            self.stat = value.into();
        } else if addr == memory::ppu::SCY {
            self.scy = value;
        } else if addr == memory::ppu::SCX {
            self.scx = value;
        } else if addr == memory::ppu::LY {
            log::warn!("LY is read only at address {:#06x}, ignoring write", addr);
            // self.ly = value;
        } else if addr == memory::ppu::LYC {
            self.lyc = value;
        } else if addr == memory::ppu::DMA {
            self.dma = value;
            self.dma_cycles = 160;
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
            self.wx = value;
        } else {
            return None;
        }
        return Some(());
    }
}

impl PPU {
    const ROWS: usize = 144;
    const COLUMNS: usize = 160;
    const TILES: usize = 0x180; //sanity check: 0x97FF+1 - 0x8000 / 16
    const TILE_SIZE: usize = 8; //this is one line i.e. size*size=total pixels
    const BYTES_PER_TILE: usize = 16;
    const TILE_MAP_SIZE: usize = 32; //this is one line of tiles i.e. size*size=total tiles

    pub fn new() -> Self {
        let mut ppu = Self {
            frame_buffer: [0; PPU::ROWS * PPU::COLUMNS],
            back_buffer: [0; PPU::ROWS * PPU::COLUMNS],
            back_buffer_index: 0,
            vram: [0; memory::ppu::VRAM.size],
            oam: [0; memory::ppu::OAM.size],
            lcdc: lcdc::LCDControl::from(0),
            stat: stat::LCDStatus::from(0),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            dma_cycles: 0,
            bgp: palette::PaletteData::from(0),
            obp0: palette::PaletteData::from(0),
            obp1: palette::PaletteData::from(0),
            wy: 0,
            wx: 0,
            fetcher: Fetcher::new(),
            fifo: Fifo::new(),
            dots: 0,
            frame_ready: false,
            ppu_debug: PPUDebug::new(),
        };
        ppu.stat.mode_flag = LCDModeFlag::VBlank;
        ppu
    }

    fn handle_int(&mut self, gb: &mut Gameboy) {
        if self.ly == self.lyc {
            self.stat.lyc_flag = true;
        } else {
            self.stat.lyc_flag = false;
        }
        if gb.cpu.interrupt_master_enable {
            if self.stat.lyc_interrupt_enable {
                if self.ly == self.lyc {
                    gb.cpu.if_register.lcd_stat = true;
                }
            }
            if self.ly == 144 {
                gb.cpu.if_register.vblank = true;
            }
        }
    }

    fn handle_dma(&mut self, gb: &Gameboy) {
        let oam_addr = 0x00A0 - self.dma_cycles;
        let src_addr = ((self.dma & 0xDF) as u16) << 8 | oam_addr as u16;
        log::trace!("dma oam addr: {:#06X}, src addr: {:#06X}", oam_addr, src_addr);
        self.oam[oam_addr as usize] = gb.read8_unlocked(src_addr);

        self.dma_cycles -= 1;
    }

    fn handle_hblank(&mut self, gb: &mut Gameboy) {
        if self.dots == 0 {
            log::trace!("hblank fifo {}", self.fifo.bg_fifo.len());

            if self.back_buffer_index == 0 {
                self.stat.mode_flag = LCDModeFlag::VBlank;
                self.dots = 4560;
                if gb.cpu.interrupt_master_enable {
                    if self.stat.mode1_vblank_interrupt_enable {
                        gb.cpu.if_register.lcd_stat = true;
                    }
                }
            } else {
                self.stat.mode_flag = LCDModeFlag::SearchingOAM;
                self.dots = 80;
                if gb.cpu.interrupt_master_enable {
                    if self.stat.mode2_oam_interrupt_enable {
                        gb.cpu.if_register.lcd_stat = true;
                    }
                }
            }
            self.ly += 1;
        }
    }

    fn handle_vblank(&mut self, gb: &mut Gameboy) {
        if self.dots == 0 {
            self.frame_ready = true;
            log::trace!("---vblank fifo {}", self.fifo.bg_fifo.len());
            self.stat.mode_flag = LCDModeFlag::SearchingOAM;
            self.dots = 80;
            self.ly = 0;
            if gb.cpu.interrupt_master_enable {
                if self.stat.mode2_oam_interrupt_enable {
                    gb.cpu.if_register.lcd_stat = true;
                }
            }
            // for (i) in 0..4 {
            //     self.bgp.color_map[i] = self.bgp.color_map[(i + 1) % 4];
            // }
        } else if self.dots % 456 == 0 {
            self.ly += 1;
        }
    }

    fn handle_oam_search(&mut self) {
        if self.dots == 0 {
            self.stat.mode_flag = LCDModeFlag::TransferringDataToLCD;
        } else if self.dots % 2 == 0 {
            //content takes 2 dots to complete
            let addr: usize = (40 - (self.dots as usize / 2 + 1)) * 4;
            let y_pos: u8 = self.oam[addr];
            let x_pos: u8 = self.oam[addr + 1];
            if (x_pos != 0)
                && ((self.ly + 16) >= y_pos)
                && ((self.ly + 16) < (y_pos.wrapping_add(PPU::TILE_SIZE as u8)))
            {
                self.fetcher
                    .add_visible_object(addr as u16 + memory::ppu::OAM.begin, x_pos, y_pos);
            }
        }
    }

    fn handle_pixel_transfer(&mut self, gb: &mut Gameboy, popped: u32) {
        self.dots = self.dots.wrapping_add(1);
        if self.dots > 4000 {
            log::info!(
                "mode 3 ongoing, dots taken {}, {}, pushed {}",
                self.dots,
                self.back_buffer_index,
                self.fifo.x
            );
        }
        if popped == 0 && self.back_buffer_index % (PPU::COLUMNS) == 0 {
            log::info!(
                "mode 3 done, dots taken {}, {}, pushed {}",
                self.dots,
                self.back_buffer_index,
                self.fifo.x
            );
            self.fetcher.clear_visible_objects();
            self.fetcher.reset();
            self.fifo.reset();
            // self.fifo.reset(); //doesnt work
            self.stat.mode_flag = LCDModeFlag::HBlank;
            if gb.cpu.interrupt_master_enable {
                if self.stat.mode0_hblank_interrupt_enable {
                    gb.cpu.if_register.lcd_stat = true;
                }
            }
            self.dots = 456 - 80 - 172; // last one needs to be modifyable
        }
    }

    pub fn get_frame_buffer(&mut self) -> Option<&[u32]> {
        if self.frame_ready {
            self.frame_ready = false;
            Some(&self.frame_buffer)
        } else {
            None
        }
    }

    pub fn test_load_memory(&mut self, mem: &[u8]) {
        self.vram[..memory::ppu::VRAM.size]
            .clone_from_slice(mem[memory::ppu::VRAM.begin as usize..=memory::ppu::VRAM.end as usize].into());
        self.oam[..memory::ppu::OAM.size]
            .clone_from_slice(mem[memory::ppu::OAM.begin as usize..=memory::ppu::OAM.end as usize].into());

        self.lcdc = LCDControl::from(mem[memory::ppu::LCDC as usize]);
        self.stat = LCDStatus::from(mem[memory::ppu::STAT as usize]);
        self.scy = mem[memory::ppu::SCY as usize];
        self.scx = mem[memory::ppu::SCX as usize];
        // self.ly = mem[memory::ppu::LY as usize];
        self.ly = 0;
        self.lyc = mem[memory::ppu::LYC as usize];
        self.dma = mem[memory::ppu::DMA as usize];
        self.bgp = PaletteData::from(mem[memory::ppu::BGP as usize]);
        self.obp0 = PaletteData::from(mem[memory::ppu::OBP0 as usize]);
        self.obp1 = PaletteData::from(mem[memory::ppu::OBP1 as usize]);
        self.wy = mem[memory::ppu::WY as usize];
        self.wx = mem[memory::ppu::WX as usize];
    }

    pub fn push_into_frame_buffer(&mut self, pixel: u32) {
        self.back_buffer[self.back_buffer_index] = pixel; // cgb correction: pixel * 3 / 4 + 0x08;
        self.back_buffer_index += 1;
        if self.back_buffer_index >= self.back_buffer.len() {
            self.frame_buffer = self.back_buffer.clone();
            self.back_buffer = [0; PPU::ROWS * PPU::COLUMNS];
            self.back_buffer_index = 0;
        }
    }

    pub fn print_state_machine(&self) {
        self.fetcher.print_state_machine();
        self.fifo.print_state_machine();
        println!("PPU States:");
        println!("\tLY: {}", self.ly);
        println!("\tbuffer index: {}", self.back_buffer_index);
        match self.stat.mode_flag {
            LCDModeFlag::HBlank => {
                println!("\t\tSEARCHING_OAM");
                println!("\t\tTRANSFERRING_DATA_TO_LCD");
                println!("\t=>\tHBLANK");
                println!("\t\tVBLANK");
            }
            LCDModeFlag::VBlank => {
                println!("\t\tSEARCHING_OAM");
                println!("\t\tTRANSFERRING_DATA_TO_LCD");
                println!("\t\tHBLANK");
                println!("\t=>\tVBLANK");
            }
            LCDModeFlag::SearchingOAM => {
                println!("\t=>\tSEARCHING_OAM");
                println!("\t\tTRANSFERRING_DATA_TO_LCD");
                println!("\t\tHBLANK");
                println!("\t\tVBLANK");
            }
            LCDModeFlag::TransferringDataToLCD => {
                println!("\t\tSEARCHING_OAM");
                println!("\t=>\tTRANSFERRING_DATA_TO_LCD");
                println!("\t\tHBLANK");
                println!("\t\tVBLANK");
            }
        }
        println!("--------");
    }

    fn read8_unlocked(&self, addr: u16) -> Option<u8> {
        if addr >= memory::ppu::VRAM.begin && addr <= memory::ppu::VRAM.end {
            return Some(self.vram[usize::from(addr - memory::ppu::VRAM.begin)]);
        } else if addr >= memory::ppu::OAM.begin && addr <= memory::ppu::OAM.end {
            return Some(self.oam[usize::from(addr - memory::ppu::OAM.begin)]);
        }
        return None;
    }

    //---------DEBUG Interface--------
    pub fn process_tile_data(&mut self) {
        self.ppu_debug.process_tile_data(&self.vram);
    }

    pub fn get_tile_data_frame_buffer(
        &self,
        wrap_count: usize,
    ) -> [u32; PPU::TILES * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        self.ppu_debug.get_tile_data_frame_buffer(wrap_count, &self.vram)
    }

    pub fn get_bg_frame_buffer(
        &self,
    ) -> [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        self.ppu_debug.get_bg_frame_buffer(&self.vram, &self.lcdc, &self.bgp)
    }

    pub fn get_window_frame_buffer(
        &self,
    ) -> [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        self.ppu_debug
            .get_window_frame_buffer(&self.vram, &self.lcdc, &self.bgp)
    }

    pub fn get_objects_frame_buffer(
        &self,
    ) -> [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        self.ppu_debug
            .get_objects_frame_buffer(&self.oam, &self.obp0, &self.obp1)
    }
}

struct PPUDebug {
    tiles: [[[u8; PPU::TILE_SIZE]; PPU::TILE_SIZE]; PPU::TILES],
}

impl PPUDebug {
    pub fn new() -> Self {
        Self {
            tiles: [[[0; PPU::TILE_SIZE]; PPU::TILE_SIZE]; PPU::TILES],
        }
    }

    pub fn process_tile_data(&mut self, vram: &[u8; memory::ppu::VRAM.size]) {
        let tile_data = &vram[memory::ppu::TILE_DATA_VRAM.begin as usize..=memory::ppu::TILE_DATA_VRAM.end as usize];
        for addr in (0..PPU::TILES * PPU::BYTES_PER_TILE).step_by(PPU::BYTES_PER_TILE) {
            let tile_id: usize = addr / (PPU::BYTES_PER_TILE);
            let mut tile: [[u8; 8]; 8] = [[0; PPU::TILE_SIZE]; PPU::TILE_SIZE];
            for line in (0..PPU::BYTES_PER_TILE).step_by(2) {
                let low: u8 = tile_data[addr + line];
                let high: u8 = tile_data[addr + line + 1];

                //pixel conversion
                let mut line_pixels: [u8; PPU::TILE_SIZE] = [0; PPU::TILE_SIZE];
                for i in (0..=7).rev() {
                    line_pixels[7 - i] = bit!(high, i) << 1 | bit!(low, i);
                }

                // println!("{:?}", line_pixels);
                tile[line >> 1] = line_pixels;
            }
            self.tiles[tile_id] = tile;
        }
    }

    fn _process_tile(&mut self, _addr: u16) {}

    pub fn get_tile_data_frame_buffer(
        &self,
        wrap_count: usize,
        _vram: &[u8; memory::ppu::VRAM.size],
    ) -> [u32; PPU::TILES * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        let mut frame_buffer: [u32; PPU::TILES * (PPU::TILE_SIZE * PPU::TILE_SIZE)] =
            [0; PPU::TILES * (PPU::TILE_SIZE * PPU::TILE_SIZE)];

        for row in 0..(self.tiles.len() * PPU::TILE_SIZE / wrap_count) {
            let curr_tile_start: usize = (row / PPU::TILE_SIZE) * wrap_count;
            for tile_index in curr_tile_start..curr_tile_start + wrap_count {
                let tile_line: [u32; PPU::TILE_SIZE] = (self.tiles[tile_index][row % PPU::TILE_SIZE])
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

                let fb_start = (tile_index % wrap_count) * PPU::TILE_SIZE + (row * wrap_count * PPU::TILE_SIZE);
                frame_buffer[fb_start..fb_start + PPU::TILE_SIZE].copy_from_slice(tile_line.as_slice());
            }
        }

        frame_buffer
    }

    fn get_tiles_from_tile_map(
        &self,
        tile_map_start: u16,
        vram: &[u8; memory::ppu::VRAM.size],
        lcdc: &LCDControl,
    ) -> [&[[u8; 8]; 8]; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE] {
        let mut map_tiles: [&[[u8; 8]; 8]; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE] =
            [&[[0; 8]; 8]; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE];

        for addr in tile_map_start..tile_map_start + 0x0400 {
            let mut tile_id = vram[addr as usize - memory::ppu::VRAM.begin as usize];
            let mut offset = 0;
            if !lcdc.bg_and_window_tile_data_area {
                tile_id = tile_id.wrapping_sub(128);
                offset = 128;
            }

            map_tiles[addr as usize - tile_map_start as usize] = &self.tiles[tile_id as usize + offset as usize];
        }
        map_tiles
    }

    pub fn get_bg_frame_buffer(
        &self,
        vram: &[u8; memory::ppu::VRAM.size],
        lcdc: &LCDControl,
        bgp: &PaletteData,
    ) -> [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        let tile_map_start = if lcdc.bg_tile_map_area {
            memory::ppu::TILE_MAP_AREA_9C00.begin
        } else {
            memory::ppu::TILE_MAP_AREA_9800.begin
        };

        self.get_tile_map_frame_buffer(self.get_tiles_from_tile_map(tile_map_start, vram, lcdc), bgp)
    }

    pub fn get_window_frame_buffer(
        &self,
        vram: &[u8; memory::ppu::VRAM.size],
        lcdc: &LCDControl,
        bgp: &PaletteData,
    ) -> [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        let tile_map_start = if lcdc.window_tile_map_area {
            memory::ppu::TILE_MAP_AREA_9C00.begin
        } else {
            memory::ppu::TILE_MAP_AREA_9800.begin
        };

        self.get_tile_map_frame_buffer(self.get_tiles_from_tile_map(tile_map_start, vram, lcdc), bgp)
    }

    fn get_tile_map_frame_buffer(
        &self,
        map_tiles: [&[[u8; 8]; 8]; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE],
        bgp: &PaletteData,
    ) -> [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        let mut frame_buffer: [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] =
            [0; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)];

        for row in 0..(PPU::TILE_MAP_SIZE * PPU::TILE_SIZE) {
            let curr_tile_start: usize = (row / PPU::TILE_SIZE) * PPU::TILE_MAP_SIZE;
            for tile_index in curr_tile_start..curr_tile_start + PPU::TILE_MAP_SIZE {
                let tile_line: [u32; PPU::TILE_SIZE] = (map_tiles[tile_index][row % PPU::TILE_SIZE])
                    .iter()
                    .map(|pixel| match bgp.color_map[*pixel as usize] {
                        0 => MonochromeColor::White as u32,
                        1 => MonochromeColor::LightGray as u32,
                        2 => MonochromeColor::DarkGray as u32,
                        3 => MonochromeColor::Black as u32,
                        _ => MonochromeColor::Off as u32,
                    })
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap();

                let fb_start =
                    (tile_index % PPU::TILE_MAP_SIZE) * PPU::TILE_SIZE + (row * PPU::TILE_MAP_SIZE * PPU::TILE_SIZE);
                frame_buffer[fb_start..fb_start + PPU::TILE_SIZE].copy_from_slice(tile_line.as_slice());
            }
        }

        frame_buffer
    }

    pub fn get_objects_frame_buffer(
        &self,
        oam: &[u8; memory::ppu::OAM.size],
        obp0: &PaletteData,
        obp1: &PaletteData,
    ) -> [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] {
        let mut frame_buffer: [u32; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)] =
            [0; PPU::TILE_MAP_SIZE * PPU::TILE_MAP_SIZE * (PPU::TILE_SIZE * PPU::TILE_SIZE)];
        //TODO: if LCDC bit 2: 1 -> 2 tile objects
        for addr in (0..0x00A0).step_by(4) {
            // print(self.vram[addr])
            let entry = sprite::OAMTableEntry::new(oam, addr);
            let curr_tile = self.tiles[entry.tile_index as usize];

            if entry.x_pos <= 0 || entry.x_pos >= 168 || entry.y_pos <= 0 || entry.y_pos >= 160 {
                log::trace!("sprite is offscreen");
            } else {
                for row in entry.y_pos as usize..entry.y_pos as usize + PPU::TILE_SIZE {
                    for col in entry.x_pos as usize..entry.x_pos as usize + PPU::TILE_SIZE {
                        let palette_id = curr_tile[row - entry.y_pos as usize][col - entry.x_pos as usize];
                        let palette: &PaletteData;
                        if entry.attributes.palette_number == 0 {
                            palette = obp0;
                        } else {
                            palette = obp1;
                        }
                        frame_buffer[row * PPU::TILE_MAP_SIZE * PPU::TILE_SIZE + col] =
                            match palette.color_map[palette_id as usize] {
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

    pub fn _print_vram(&self, vram: &[u8; memory::ppu::VRAM.size]) {
        utils::print_memory_bytes(vram, "vram", 0x100);
    }

    pub fn _print_tiles(&self, count: usize) {
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
