use std::fs::File;
use std::io::{BufReader, Read};
pub struct Memory {
    rom: Vec<u8>,
}

impl Memory {
    pub fn new(rom_path: String) -> Self {
        Memory {
            rom: Self::load_rom(rom_path),
        }
    }

    fn load_rom(rom_path: String) -> Vec<u8> {
        let f = File::open(rom_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        // Read.
        for value in buffer.iter_mut() {
            println!("BYTE: {:x}", value);
        }
        buffer
    }
}
