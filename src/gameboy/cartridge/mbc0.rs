use crate::gameboy::{memory, MemoryInterface};

pub struct MBC0 {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl super::MBCInterface for MBC0 {
    fn read8_rom_bank_0(&self, addr: u16) -> u8 {
        log::trace!("mbc0 rom0 read");
        self.rom[addr as usize]
    }

    fn read8_rom_bank_n(&self, addr: u16) -> u8 {
        log::trace!("mbc0 rom1 read");
        self.rom[addr as usize]
    }

    fn read8_ram_bank_n(&self, addr: u16) -> u8 {
        log::trace!("mbc0 ram read");
        self.ram[addr as usize]
    }

    fn write8_rom_bank_0(self: &mut MBC0, addr: u16, value: u8) {
        log::error!("mbc0 rom0 is read only, addr {:#06X} - value {:#04X}", addr, value);
    }

    fn write8_rom_bank_n(self: &mut MBC0, addr: u16, value: u8) {
        log::error!("mbc0 rom1 is read only, addr {:#06X} - value {:#04X}", addr, value);
    }

    fn write8_ram_bank_n(self: &mut MBC0, addr: u16, value: u8) {
        log::trace!("mbc0 ram write");
        self.ram[addr as usize] = value;
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
