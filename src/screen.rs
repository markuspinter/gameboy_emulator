extern crate minifb;
use std::time::SystemTime;

use minifb::{Key, Window, WindowOptions};

pub enum MonochromeColor {
    Off = 0x00CADC9F,
    White = 0x009BBC0F,
    LightGray = 0x008BAC0F,
    DarkGray = 0x00306230,
    Black = 0x000F380F,
}

pub struct Screen {
    buffer: Vec<u32>,
    window: Window,
    prev: SystemTime,
    width: usize,
    height: usize,
    title_time: SystemTime,
    key_buffer: Vec<Key>,
}

impl Screen {
    pub fn new(height: usize, width: usize, scale_factor: minifb::Scale) -> Self {
        let mut window_options = WindowOptions::default();
        window_options.scale = scale_factor;
        let ppu = Self {
            width: width,
            height: height,
            buffer: vec![0; width * height],
            window: Window::new("Test - ESC to exit", width, height, window_options).unwrap_or_else(|e| {
                panic!("{}", e);
            }),
            prev: SystemTime::now(),
            title_time: SystemTime::now(),
            key_buffer: Vec::new(),
        };
        ppu
    }

    pub fn set_frame_buffer(&mut self, frame_buffer: &[u32]) {
        self.buffer = frame_buffer.to_vec();
    }

    pub fn update(&mut self) -> (bool, bool, bool) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();

        let now = SystemTime::now();
        let diff = now.duration_since(self.prev).expect("elapsed clock operation failed");

        let diff_time = now
            .duration_since(self.title_time)
            .expect("elapsed clock operation failed");
        if diff_time.as_micros() > 1e5 as u128 {
            self.window
                .set_title(format!("{:.2} fps", 1e6 / diff.as_micros() as f32).as_str());
            self.title_time = now;
        }

        self.prev = now;

        //update keys
        self.key_buffer = self.window.get_keys();

        (
            self.window.is_open() && !self.window.is_key_down(Key::Escape),
            self.window.is_key_pressed(Key::Space, minifb::KeyRepeat::No),
            self.window.is_key_pressed(Key::LeftShift, minifb::KeyRepeat::No),
        )
    }

    pub fn get_keys(&mut self) -> &Vec<Key> {
        // self.key_buffer = self.window.get_keys();
        &self.key_buffer
    }
}
