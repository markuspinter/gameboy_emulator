# Rusty Gameboy Emulator

A gameboy (currently only DMG) emulator written in pure rust.

## Motivation

This is a hobby project of mine and I started it for 2 reasons:

- Learning the Architecture of the Gameboy
- Learning Rust

It's my first larger rust project, so please feel free to give feedback if you like to.
I'm happy to learn for all of you!

## How to run

```sh
RUST_LOG=error cargo run --release <your_boot_rom> <your_gb_file>
```

