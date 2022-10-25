use std::collections::HashMap;

use log::debug;
use minifb::Key;

use crate::bit;

use super::{memory, Gameboy, GameboyModule};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Button {
    A,
    B,
    SELECT,
    START,
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

impl std::convert::From<Joypad> for u8 {
    fn from(jp: Joypad) -> u8 {
        let mut byte: u8 = 0x00;
        byte |= (jp.unused_7th_bit as u8) << 7;
        byte |= (jp.unused_6th_bit as u8) << 6;
        byte |= (jp.action_buttons_select as u8) << 5;
        byte |= (jp.direction_buttons_select as u8) << 4;
        if jp.direction_buttons_select {
            byte |= (jp.down as u8) << 3;
            byte |= (jp.up as u8) << 2;
            byte |= (jp.left as u8) << 1;
            byte |= (jp.right as u8);
        } else if jp.action_buttons_select {
            byte |= (jp.start as u8) << 3;
            byte |= (jp.select as u8) << 2;
            byte |= (jp.b as u8) << 1;
            byte |= (jp.a as u8);
        } else {
            panic!("selecting both action and direction buttons at the same time is not intended");
        }
        byte
    }
}

#[derive(Clone, Debug)]
pub struct Joypad {
    key_map: HashMap<String, Button>,

    pub unused_7th_bit: bool,
    pub unused_6th_bit: bool,
    pub action_buttons_select: bool,
    pub direction_buttons_select: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,

    key_pressed: bool,
}

impl super::MemoryInterface for Joypad {
    fn read8(&self, addr: u16) -> super::MemoryResult<u8> {
        if addr == memory::joypad::JOYP {
            return Ok(u8::from(self.clone()));
        }
        return Err(super::MemoryError::UnknownAddress);
    }

    fn write8(&mut self, addr: u16, value: u8) -> super::MemoryResult<()> {
        if addr == memory::joypad::JOYP {
            self.unused_7th_bit = bit!(value, 7) != 0;
            self.unused_6th_bit = bit!(value, 6) != 0;
            self.action_buttons_select = bit!(value, 5) != 0;
            self.direction_buttons_select = bit!(value, 4) != 0;
            if value & 0x0F != 0 {
                log::debug!(
                    "attempting to write to read only section of JOYP {} register",
                    memory::joypad::JOYP
                );
            }
            return Ok(());
        }
        return Err(super::MemoryError::UnknownAddress);
    }
}

impl GameboyModule for Joypad {
    unsafe fn tick(&mut self, gb_ptr: *mut Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        self.handle_int(gb);
        self.handle_keys(gb.get_keys());
        Ok(0)
    }
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            key_map: HashMap::from([
                ("K".to_owned(), Button::A),
                ("L".to_owned(), Button::B),
                ("F".to_owned(), Button::SELECT),
                ("J".to_owned(), Button::START),
                ("W".to_owned(), Button::UP),
                ("S".to_owned(), Button::DOWN),
                ("A".to_owned(), Button::LEFT),
                ("D".to_owned(), Button::RIGHT),
            ]),
            unused_7th_bit: false,
            unused_6th_bit: false,
            action_buttons_select: false,
            direction_buttons_select: false,
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
            key_pressed: false,
        }
    }

    fn handle_keys(&mut self, keys: Vec<Key>) {
        for key in keys {
            let key_string = format!("{:?}", key);
            if self.key_map.contains_key(&key_string) {
                println!("{:?}", self.key_map[&key_string]);
                self.key_pressed = true;
            }
        }
    }

    fn handle_int(&mut self, gb: &mut Gameboy) {
        if gb.cpu.interrupt_master_enable {
            if self.key_pressed {
                gb.cpu.if_register.joypad = true;
            }
        }
        //reset for next interrupt check
        self.key_pressed = false;
    }
}
