extern crate minifb;
use rand::rngs::ThreadRng;
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
    rng: ThreadRng,
    prev: SystemTime,
    pixel_width: usize,
    pixel_height: usize,
    width: usize,
    height: usize,
    title_time: SystemTime,
    key_buffer: Vec<Key>,
}

impl Screen {
    pub fn new(
        rows: usize,
        columns: usize,
        pixel_width: usize,
        pixel_height: usize,
        scale_factor: minifb::Scale,
    ) -> Self {
        let width = columns * pixel_width;
        let height = rows * pixel_height;
        let mut window_options = WindowOptions::default();
        window_options.scale = scale_factor;
        let ppu = Self {
            width: width,
            height: height,
            buffer: vec![0; width * height],
            window: Window::new("Test - ESC to exit", width, height, window_options).unwrap_or_else(|e| {
                panic!("{}", e);
            }),
            rng: rand::thread_rng(),
            prev: SystemTime::now(),
            pixel_width,
            pixel_height,
            title_time: SystemTime::now(),
            key_buffer: Vec::new(),
        };
        // ppu.window
        //     .limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        ppu
    }

    pub fn set_frame_buffer(&mut self, frame_buffer: &[u32]) {
        self.buffer = frame_buffer.to_vec();
    }

    pub fn update(&mut self) -> (bool, bool) {
        // let offset: u128 = self
        //     .prev
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .expect("asdf")
        //     .as_millis()
        //     % u32::MAX as u128;
        // for (i, item) in self.buffer.iter_mut().enumerate() {
        //     if i % 3 == 0 {
        //         // break;
        //         *item = (offset as u32).wrapping_add(i as u32 + 0xA00);
        //     }
        // }

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
        )
    }

    pub fn get_keys(&mut self) -> &Vec<Key> {
        // self.key_buffer = self.window.get_keys();
        &self.key_buffer
    }
}
