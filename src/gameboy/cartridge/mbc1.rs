use crate::gameboy::MemoryInterface;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl super::MBCInterface for MBC1 {
    fn read8_rom_bank_0(&self, addr: u16) -> u8 {
        todo!()
    }

    fn read8_rom_bank_n(&self, addr: u16) -> u8 {
        todo!()
    }

    fn read8_ram_bank_n(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write8_rom_bank_0(&mut self, addr: u16, value: u8) {
        todo!()
    }

    fn write8_rom_bank_n(&mut self, addr: u16, value: u8) {
        todo!()
    }

    fn write8_ram_bank_n(&mut self, addr: u16, value: u8) {
        todo!()
    }

    fn get_rom(&self) -> &Vec<u8> {
        &self.rom
    }

    fn new(rom: Vec<u8>, ram: Vec<u8>) -> Self
    where
        Self: Sized,
    {
        Self { rom, ram }
    }
}