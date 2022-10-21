use std::env;

pub mod gameboy;
use std::process::Command;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    println!(
        "{}",
        String::from_utf8_lossy(&Command::new("pwd").output().unwrap().stdout)
    );

    let gb = gameboy::Gameboy::new(args[1].clone(), args[2].clone());

    // gb.run();
}
