pub fn print_memory(mem: &[u8], name: &str) {
    print_memory_bytes(mem, name, mem.len());
}

pub fn print_memory_bytes(mem: &[u8], name: &str, bytes: usize) {
    // Read.
    print!("{:<7}|  ", name);
    for i in 0..0x10 {
        print!("{:#04X}  |  ", i);
    }
    println!();
    for _i in 0..0x11 {
        print!("{:_<9}", "");
    }
    for (i, value) in mem.iter().enumerate() {
        if (i % 0x10) == 0 {
            println!();
            print!("{:#04X}  |  ", i);
        }
        print!("{:#04X}  |  ", value);
        if i >= bytes - 1 {
            break;
        };
    }
    println!("\n\n");
}
