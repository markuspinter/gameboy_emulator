use std::env;

pub mod gameboy;
pub mod screen;
pub mod utils;

use std::process::Command;

fn main() {
    env_logger::init();

    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    println!(
        "{}",
        String::from_utf8_lossy(&Command::new("pwd").output().unwrap().stdout)
    );

    let mut gb = gameboy::Gameboy::new(args[1].clone(), args[2].clone());

    unsafe {
        if args.len() >= 4 {
            if args[3] == "--test" {
                println!("test run");
                gb.test_run().unwrap();
            } else if args[3] == "--debug" {
                gb.run(true).unwrap();
            } else {
                gb.run(false).unwrap();
            }
        } else {
            gb.run(false).unwrap();
        }
    }
}
