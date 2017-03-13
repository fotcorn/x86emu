use std::fs::File;
use std::io::Read;
use std::env;

extern crate x86emu;
use x86emu::cpu::print::PrintCPU;
use x86emu::machine_state::MachineState;
use x86emu::decoder::Decoder;

const SETUP_SECT_POSITION: usize = 0x1F1;
const BIT64_OFFSET: usize = 0x200;


// see <linux kernel source>/Documentation/x86/boot.txt for documentation of the boot protocol
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

    let setup_sect = buffer[SETUP_SECT_POSITION];

    let mut offset = (setup_sect + 1) as usize * 512;
    offset += BIT64_OFFSET;

    let main_code = &buffer[offset as usize..(offset + 0x10000) as usize];

    let mut cpu = PrintCPU{};
    let mut machine_state = MachineState::new(main_code.to_vec());
    let mut decoder = Decoder::new(&mut cpu, &mut machine_state);
    decoder.execute();
}
