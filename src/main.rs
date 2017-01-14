use std::fs::File;
use std::io::Read;
use std::env;

const MAGIC_NUMBER: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: cargo run <program>");
            return;
        }
    };

    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file.");

    if &buffer[..4] != MAGIC_NUMBER {
        println!("Not a valid ELF file");
        return;
    }
}
