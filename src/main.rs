use std::env;

pub mod gameboy;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let gb = gameboy::Gameboy::new("".into());

    gb.run();
}
