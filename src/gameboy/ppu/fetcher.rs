use core::panic;

use crate::{
    bit,
    gameboy::{memory, Gameboy, GameboyModule, MemoryInterface},
};

use super::{fifo::FifoElement, sprite::SpriteAttributes, PPU};

#[derive(Copy, Clone, Debug)]
pub enum FetcherState {
    GET_TILE,
    GET_DATA_LOW,
    GET_DATA_HIGH,
    SLEEP,
    PUSH,
    WAIT,
}

pub struct Fetcher {
    state: FetcherState,
    prev_state: FetcherState,
    pub x: u8,
    y: u8,
    tile_map_start: u16,
    tile_data_start: u16,
    next_tile_id: u8,
    low: u8,
    high: u8,
    visible_objects: Vec<(u16, u8, u8)>, //addr, x, y
    fetching_object: bool,
    curr_object_index: usize,
    drawing_window: bool,
}

impl GameboyModule for Fetcher {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        self.step(&mut gb.ppu);
        Ok(0)
    }
}

impl Fetcher {
    const MAX_SPRITES_PER_ROW: usize = 10;
    pub fn new() -> Self {
        Self {
            state: FetcherState::GET_TILE,
            prev_state: FetcherState::GET_TILE,
            x: 0,
            y: 0,
            tile_map_start: 0,
            tile_data_start: 0,
            next_tile_id: 0,
            low: 0,
            high: 0,
            visible_objects: Vec::new(),
            fetching_object: false,
            curr_object_index: 0,
            drawing_window: false,
        }
    }

    pub fn add_visible_object(&mut self, addr: u16, x_pos: u8, y_pos: u8) {
        if self.visible_objects.len() < Self::MAX_SPRITES_PER_ROW {
            self.visible_objects.push((addr, x_pos, y_pos));
        }
    }

    pub fn clear_visible_objects(&mut self) {
        self.visible_objects.clear();
    }

    fn step(&mut self, ppu: &mut super::PPU) {
        if ppu.lcdc.window_enable && !self.drawing_window && ppu.fifo.x == ppu.wx && ppu.ly >= ppu.wy {
            ppu.fifo.bg_fifo.clear();
            self.state = FetcherState::GET_TILE;
            self.prev_state = FetcherState::GET_TILE;
            self.drawing_window = true;
        }
        if ppu.lcdc.obj_enable && !self.fetching_object {
            for (i, obj) in self.visible_objects.iter().enumerate() {
                if obj.1 == ppu.fifo.x + 8 {
                    self.fetching_object = true;
                    self.curr_object_index = i;
                    ppu.fifo.suspend_fifo();
                    self.state = FetcherState::GET_TILE;
                    self.prev_state = FetcherState::GET_TILE;
                }
            }
        }
        let next_state = match self.state {
            FetcherState::GET_TILE => self.get_tile(ppu),
            FetcherState::GET_DATA_LOW => self.get_data_low(ppu),
            FetcherState::GET_DATA_HIGH => self.get_data_high(ppu),
            FetcherState::SLEEP => self.sleep(),
            FetcherState::PUSH => self.push(ppu),
            FetcherState::WAIT => self.wait(),
        };
        self.prev_state = self.state.clone();
        self.state = next_state;
    }

    fn get_tile(&mut self, ppu: &mut super::PPU) -> FetcherState {
        self.y = ppu.ly.wrapping_add(ppu.scy);
        if ppu.lcdc.window_enable && self.drawing_window {
            if ppu.lcdc.window_tile_map_area {
                self.tile_map_start = memory::ppu::TILE_MAP_AREA_9C00.begin; //bg 9c00
                                                                             // } else if self.x >= ppu.wx && ppu.lcdc.window_tile_map_area {
                                                                             //     self.tile_map_start = memory::ppu::TILE_MAP_AREA_9C00_VRAM.begin; //window 9c00
            } else {
                self.tile_map_start = memory::ppu::TILE_MAP_AREA_9800.begin;
            }
        } else {
            if ppu.lcdc.bg_tile_map_area {
                self.tile_map_start = memory::ppu::TILE_MAP_AREA_9C00.begin; //bg 9c00
                                                                             // } else if self.x >= ppu.wx && ppu.lcdc.window_tile_map_area {
                                                                             //     self.tile_map_start = memory::ppu::TILE_MAP_AREA_9C00_VRAM.begin; //window 9c00
            } else {
                self.tile_map_start = memory::ppu::TILE_MAP_AREA_9800.begin;
            }
        }

        let addr: u16;
        if self.fetching_object {
            addr = self.visible_objects[self.curr_object_index].0 + 2;
        } else {
            addr =
                self.tile_map_start + ((self.x + (ppu.scx / 8)) & 0x1F) as u16 + (((self.y) / 8) as u16 * (32) as u16);
        }

        self.next_tile_id = match ppu.read8_unlocked(addr) {
            Some(val) => val,
            None => panic!("reached invalid address {:#06X} in fetcher get tile", addr),
        };
        FetcherState::WAIT
    }

    fn get_data_low(&mut self, ppu: &mut super::PPU) -> FetcherState {
        let addr;
        if !self.fetching_object {
            self.tile_data_start = memory::ppu::TILE_DATA_AREA_8800.begin;
            if ppu.lcdc.bg_and_window_tile_data_area {
                self.tile_data_start = memory::ppu::TILE_DATA_AREA_8000.begin;
            } else {
                self.next_tile_id = self.next_tile_id.wrapping_sub(128);
            }
            addr = self.tile_data_start
                + (self.next_tile_id as u16 * PPU::BYTES_PER_TILE as u16)
                + ((self.y % 8) as u16 * 2);
        } else {
            self.tile_data_start = memory::ppu::TILE_DATA_AREA_8000.begin;
            addr = self.tile_data_start
                + (self.next_tile_id as u16 * PPU::BYTES_PER_TILE as u16)
                + ((self
                    .y
                    .wrapping_add(16)
                    .wrapping_sub(self.visible_objects[self.curr_object_index].2)) as u16
                    * 2);
        }

        self.low = match ppu.read8_unlocked(addr) {
            Some(val) => val,
            None => panic!(
                "reached invalid address {:#06X} in fetcher low",
                self.tile_data_start + self.next_tile_id as u16
            ),
        };
        FetcherState::WAIT
    }

    fn get_data_high(&mut self, ppu: &mut super::PPU) -> FetcherState {
        let addr;
        if !self.fetching_object {
            addr = self.tile_data_start
                + (self.next_tile_id as u16 * PPU::BYTES_PER_TILE as u16)
                + ((ppu.ly % 8) as u16 * 2)
                + 1;
        } else {
            addr = self.tile_data_start
                + (self.next_tile_id as u16 * PPU::BYTES_PER_TILE as u16)
                + ((self
                    .y
                    .wrapping_add(16)
                    .wrapping_sub(self.visible_objects[self.curr_object_index].2)) as u16
                    * 2)
                + 1;
        }
        self.high = match ppu.read8_unlocked(addr) {
            Some(val) => val,
            None => panic!(
                "reached invalid address {:#06X} in fetcher high",
                self.tile_data_start + self.next_tile_id as u16 + 1
            ),
        };
        FetcherState::WAIT
    }

    fn sleep(&self) -> FetcherState {
        FetcherState::WAIT
    }

    fn push(&mut self, ppu: &mut super::PPU) -> FetcherState {
        if self.fetching_object {
            let attr: SpriteAttributes = match ppu.read8_unlocked(self.visible_objects[self.curr_object_index].0 + 3) {
                Some(val) => SpriteAttributes::from(val),
                None => panic!(
                    "reached invalid address {:#06X} in fetcher high",
                    self.tile_data_start + self.next_tile_id as u16 + 1
                ),
            };
            if attr.x_flip {
                for i in (0..=7) {
                    ppu.fifo.push_into_object_fifo(FifoElement {
                        color_id: bit!(self.high, i) << 1 | bit!(self.low, i),
                        palette_nummber: attr.palette_number,
                        bg_priority: attr.bg_window_override,
                        is_object: true,
                    });
                }
            } else {
                for i in (0..=7).rev() {
                    ppu.fifo.push_into_object_fifo(FifoElement {
                        color_id: bit!(self.high, i) << 1 | bit!(self.low, i),
                        palette_nummber: attr.palette_number,
                        bg_priority: attr.bg_window_override,
                        is_object: true,
                    });
                }
            }
            self.fetching_object = false;
            self.visible_objects.remove(self.curr_object_index);
            FetcherState::GET_TILE
        } else {
            if ppu.fifo.is_fifo_pushable() {
                //pixel conversion
                for i in (0..=7).rev() {
                    ppu.fifo.push_into_bg_fifo(FifoElement {
                        color_id: bit!(self.high, i) << 1 | bit!(self.low, i),
                        palette_nummber: 0,
                        bg_priority: false,
                        is_object: false,
                    });
                }
                self.x += 1;
                // self.x %= (PPU::COLUMNS / 8) as u8;

                FetcherState::GET_TILE
            } else {
                FetcherState::PUSH
            }
        }
    }

    fn wait(&self) -> FetcherState {
        match self.prev_state {
            FetcherState::GET_TILE => FetcherState::GET_DATA_LOW,
            FetcherState::GET_DATA_LOW => FetcherState::GET_DATA_HIGH,
            FetcherState::GET_DATA_HIGH => FetcherState::SLEEP,
            FetcherState::SLEEP => FetcherState::PUSH,
            FetcherState::PUSH => panic!("changing state from push to wait is not intended"),
            FetcherState::WAIT => panic!("changing state from wait to wait is not intended"),
        }
    }

    pub fn print_state_machine(&self) {
        println!("Fetcher States:");
        println!("\tx: {}", self.x);
        println!("\ty: {}", self.y);
        println!("\tfetching obj: {}", self.fetching_object);
        let mut print_state = self.state;
        if matches!(print_state, FetcherState::WAIT) {
            print_state = self.prev_state;
        }
        match print_state {
            FetcherState::GET_TILE => {
                println!("\t=>\tGET_TILE");
                println!("\t\tGET_DATA_LOW");
                println!("\t\tGET_DATA_HIGH");
                println!("\t\tSLEEP");
                println!("\t\tPUSH");
            }
            FetcherState::GET_DATA_LOW => {
                println!("\t\tGET_TILE");
                println!("\t=>\tGET_DATA_LOW");
                println!("\t\tGET_DATA_HIGH");
                println!("\t\tSLEEP");
                println!("\t\tPUSH");
            }
            FetcherState::GET_DATA_HIGH => {
                println!("\t\tGET_TILE");
                println!("\t\tGET_DATA_LOW");
                println!("\t=>\tGET_DATA_HIGH");
                println!("\t\tSLEEP");
                println!("\t\tPUSH");
            }
            FetcherState::SLEEP => {
                println!("\t\tGET_TILE");
                println!("\t\tGET_DATA_LOW");
                println!("\t\tGET_DATA_HIGH");
                println!("\t=>\tSLEEP");
                println!("\t\tPUSH");
            }
            FetcherState::PUSH => {
                println!("\t\tGET_TILE");
                println!("\t\tGET_DATA_LOW");
                println!("\t\tGET_DATA_HIGH");
                println!("\t\tSLEEP");
                println!("\t=>\tPUSH");
            }
            FetcherState::WAIT => {
                println!("wait should not happen");
            }
        }
        println!("--------");
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.drawing_window = false;
        self.state = FetcherState::GET_TILE;
        self.prev_state = FetcherState::GET_TILE;
    }
}
