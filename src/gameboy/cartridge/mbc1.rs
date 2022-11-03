use crate::gameboy::{memory, MemoryInterface};

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    banking_mode: u8,
    ram_enable: bool,
    selected_rom_bank: u16,
    selected_ram_bank: u8,
}

impl super::MBCInterface for MBC1 {
    fn read8_rom_bank_0(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn read8_rom_bank_n(&self, addr: u16) -> u8 {
        self.rom[(addr - memory::cartridge::ROM_BANK_N.begin) as usize
            + (memory::cartridge::ROM_BANK_N.size * self.selected_rom_bank as usize)]
    }

    fn read8_ram_bank_n(&self, addr: u16) -> u8 {
        self.ram[(addr - memory::cartridge::EXTERNAL_RAM.begin) as usize
            + (memory::cartridge::EXTERNAL_RAM.size * self.selected_ram_bank as usize)]
    }

    fn write8_rom_bank_0(&mut self, addr: u16, value: u8) {
        if addr >= write::RAM_ENABLE.begin && addr <= write::RAM_ENABLE.end {
            if value == 0x0A {
                self.ram_enable = true;
                log::debug!("ram in mbc1 enabled");
            } else {
                self.ram_enable = false;
                log::debug!("ram in mbc1 disabled");
            }
        } else if addr >= write::ROM_BANK_NUMBER.begin && addr <= write::ROM_BANK_NUMBER.end {
            self.selected_rom_bank = (value & 0x1F) as u16;
            if self.selected_rom_bank == 0 {
                self.selected_rom_bank = 1;
            }
            if self.banking_mode != 0 {
                panic!("advanced banking mode not supported");
            }
        }
    }

    fn write8_rom_bank_n(&mut self, addr: u16, value: u8) {
        if addr >= write::RAM_BANK_NUMBER.begin && addr <= write::RAM_BANK_NUMBER.end {
            self.selected_ram_bank = std::cmp::min((self.ram.len() / 0x2000 - 1) as u8, value & 0x11);
            log::debug!("select ram bank: {}", self.selected_ram_bank);
        } else if addr >= write::BANKING_MODE_SELECT.begin && addr <= write::BANKING_MODE_SELECT.end {
            self.banking_mode = value;
            if self.banking_mode != 0 {
                panic!("advanced banking mode not supported");
            }
        }
    }

    fn write8_ram_bank_n(&mut self, addr: u16, value: u8) {
        self.ram[(addr - memory::cartridge::EXTERNAL_RAM.begin) as usize
            + (memory::cartridge::EXTERNAL_RAM.size * self.selected_ram_bank as usize)] = value;
    }

    fn get_rom(&self) -> &Vec<u8> {
        &self.rom
    }

    fn new(rom: Vec<u8>, ram: Vec<u8>) -> Self
    where
        Self: Sized,
    {
        Self {
            rom,
            ram,
            banking_mode: 0,
            ram_enable: false,
            selected_rom_bank: 1,
            selected_ram_bank: 0,
        }
    }
}

mod read {
    use crate::gameboy::memory::MemoryRange;

    pub const ROM_BANK_X0: MemoryRange = MemoryRange {
        begin: 0x0000,
        end: 0x3FFF,
        size: 0x4000,
    };

    pub const ROM_BANK_01_7F: MemoryRange = MemoryRange {
        begin: 0x4000,
        end: 0x7FFF,
        size: 0x4000,
    };

    pub const RAM_BANK_00_03: MemoryRange = MemoryRange {
        begin: 0xA000,
        end: 0xBFFF,
        size: 0x2000,
    };
}

mod write {
    use crate::gameboy::memory::MemoryRange;

    pub const RAM_ENABLE: MemoryRange = MemoryRange {
        begin: 0x0000,
        end: 0x1FFF,
        size: 0x2000,
    };

    pub const ROM_BANK_NUMBER: MemoryRange = MemoryRange {
        begin: 0x2000,
        end: 0x3FFF,
        size: 0x2000,
    };

    pub const RAM_BANK_NUMBER: MemoryRange = MemoryRange {
        begin: 0x4000,
        end: 0x5FFF,
        size: 0x2000,
    };

    pub const BANKING_MODE_SELECT: MemoryRange = MemoryRange {
        begin: 0x6000,
        end: 0x7FFF,
        size: 0x2000,
    };
}
