use std::fmt::Error;

use super::{memory::Memory, GameboyModule, MemoryInterface};

mod instructions;

pub struct CPU {
    pc: u16,
}

impl GameboyModule for CPU {
    fn tick(&self, memory: &Memory) -> Result<u32, std::fmt::Error> {
        self.decode_execute(memory)
    }
}

impl CPU {
    fn decode_execute(&self, memory: &Memory) -> Result<u32, std::fmt::Error> {
        let mut opcode: u16 = match memory.read8(self.pc) {
            Ok(num) => u16::from(num),
            Err(_) => return Err(Error),
        };
        if opcode == 0xCB {
            opcode = match memory.read8(self.pc + 1) {
                Ok(num) => u16::from(num),
                Err(_) => return Err(Error),
            };
            opcode += 0x100;
        }
        instructions::execute_opcode(opcode, self)
    }
    pub fn new() -> Self {
        Self { pc: 0x00 }
    }
}
