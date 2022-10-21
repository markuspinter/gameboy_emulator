pub fn execute_opcode(opcode: u16, cpu: &super::CPU) -> Result<u32, std::fmt::Error> {
    println!("executing opcode: {:#06X}", opcode);
    return Ok(0);
}
