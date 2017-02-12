use std::fs::File;
use std::io::Read;
use std::env;

extern crate x86emu;
use x86emu::cpu::CPU;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: cargo run --bin elf <program>");
            return;
        }
    };

    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let offset = 0x4800; // TODO calculate from setup_sects

    let main_code = &buffer[offset as usize .. (offset + 0x10000) as usize];

    let mut cpu = CPU::new(main_code.to_vec());
    cpu.execute();
}
