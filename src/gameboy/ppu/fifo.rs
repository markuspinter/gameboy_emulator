use std::collections::VecDeque;

use crate::gameboy::GameboyModule;

use super::{palette::MonochromeColor, PPU};

#[derive(Clone, Debug)]
pub struct FifoElement {
    pub color_id: u8,
    pub palette_nummber: u8,
    pub bg_priority: bool,
    pub is_object: bool,
}

pub struct Fifo {
    pub bg_fifo: VecDeque<FifoElement>,
    object_fifo: VecDeque<FifoElement>,
    pub x: u8,
    is_suspended: bool,
    pub flush: bool,
    popped: bool,
}

impl GameboyModule for Fifo {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        self.popped = false;
        if self.bg_fifo.len() > PPU::TILE_SIZE {
            if let Some(pixel) = self.pop(&gb.ppu) {
                gb.ppu.push_into_frame_buffer(pixel);
            } else {
                return Ok(1);
            }
        } else {
            return Ok(1);
        }
        self.popped = true;
        Ok(0)
    }
}

impl Fifo {
    pub fn new() -> Self {
        Self {
            bg_fifo: VecDeque::new(),
            object_fifo: VecDeque::new(),
            x: 0,
            is_suspended: false,
            flush: false,
            popped: false,
        }
    }

    pub fn is_fifo_pushable(&self) -> bool {
        self.bg_fifo.len() <= PPU::TILE_SIZE
    }

    pub fn _is_fifo_poppable(&self) -> bool {
        self.bg_fifo.len() > PPU::TILE_SIZE
    }

    pub fn push_into_bg_fifo(&mut self, elem: FifoElement) {
        self.bg_fifo.push_back(elem);
    }

    pub fn push_into_object_fifo(&mut self, elem: FifoElement) {
        self.object_fifo.push_back(elem);
        if self.object_fifo.len() == 8 {
            self.mix_tile_line();
            self.object_fifo.clear();
            self.is_suspended = false;
        }
    }

    pub fn mix_tile_line(&mut self) {
        for i in 0..std::cmp::min(self.bg_fifo.len(), 8) {
            if self.object_fifo[i].color_id != 0 {
                if self.object_fifo[i].bg_priority {
                    if self.bg_fifo[i].color_id == 0 {
                        self.bg_fifo[i] = self.object_fifo[i].clone();
                    }
                } else {
                    if !self.bg_fifo[i].is_object {
                        self.bg_fifo[i] = self.object_fifo[i].clone();
                    }
                }
            }
        }
    }

    pub fn suspend_fifo(&mut self) {
        self.is_suspended = true;
    }

    pub fn pop(&mut self, ppu: &PPU) -> Option<u32> {
        if !self.is_suspended {
            self.x += 1;
            let elem = self.bg_fifo.pop_front().unwrap();
            let mut color_id = ppu.bgp.color_map[elem.color_id as usize];
            if elem.is_object {
                if elem.palette_nummber == 0 {
                    color_id = ppu.obp0.color_map[elem.color_id as usize];
                } else {
                    color_id = ppu.obp1.color_map[elem.color_id as usize];
                }
            }
            Some(match color_id {
                0 => MonochromeColor::White as u32,
                1 => MonochromeColor::LightGray as u32,
                2 => MonochromeColor::DarkGray as u32,
                3 => MonochromeColor::Black as u32,
                _ => MonochromeColor::Off as u32,
            })
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.bg_fifo.clear();
    }

    pub fn print_state_machine(&self) {
        println!("Fifo States:");
        println!("\tx: {}", self.x);
        println!("\tflush: {}", self.flush);
        println!("\tis suspended: {}", self.is_suspended);
        println!("\tfifo len: {}", self.bg_fifo.len());
        if self.popped {
            println!("\t\tIDLE");
            println!("\t=>\tPOP");
        } else {
            println!("\t=>\tIDLE");
            println!("\t\tPOP");
        }
        println!("--------");
    }
}
