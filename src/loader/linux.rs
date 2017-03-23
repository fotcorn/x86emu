use std::fs::File;
use std::io::Read;

use machine_state::MachineState;
use decoder::Decoder;
use cpu::cpu_trait::CPU;

const SETUP_HEADER_OFFSET: usize = 0x1F1;
const BIT64_OFFSET: i64 = 0x200;
const ZERO_PAGE_ADDRESS: u64 = 0x10000;
const LOAD_ADDRESS: i64 = 0x100000;

/* see <linux kernel source>/Documentation/x86/boot.txt and zero-page.txt
 * for documentation of the 64 bit boot protocol
 */
pub fn linux(filename: &str, cpu: &CPU) {
    // load kernel image from disk
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let mut machine_state = MachineState::new();

    // create zero page and copy setup header into it
    let setup_header_end: usize = 0x202 as usize + buffer[0x201] as usize;
    let setup_header = &buffer[SETUP_HEADER_OFFSET..setup_header_end];
    machine_state.mem_write(ZERO_PAGE_ADDRESS + 0x1F1, setup_header);
    machine_state.rsi = ZERO_PAGE_ADDRESS as i64;

    // load code at 0x100.000
    // count of setup sects is the first value in the setup header struct
    let setup_sect = buffer[SETUP_HEADER_OFFSET];
    let offset = (setup_sect + 1) as usize * 512;

    machine_state.mem_write(LOAD_ADDRESS as u64, &buffer[offset..]);
    machine_state.rip = LOAD_ADDRESS + BIT64_OFFSET;

    // start execution
    let mut decoder = Decoder::new(cpu, &mut machine_state);
    decoder.execute();
}
