pub mod gameboy;

fn main() {
    println!("Hello, world!");

    let gb = gameboy::Gameboy::new("".into());

    gb.run();
}
