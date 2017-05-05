use std::fs::File;
use std::io::Read;

use machine_state::MachineState;
use decoder::Decoder;
use cpu::cpu_trait::CPU;
use utils::convert_i32_to_u8vec;

const SETUP_HEADER_OFFSET: u64 = 0x1F1;
const BIT64_OFFSET: u64 = 0x200;
const ZERO_PAGE_ADDRESS: u64 = 0x140a0;
const COMMAND_LINE_ADDRESS: u64 = 0x20000;
const LOAD_ADDRESS: u64 = 0x100000;

/* see <linux kernel source>/Documentation/x86/boot.txt and zero-page.txt
 * for documentation of the 64 bit boot protocol
 */
pub fn linux(filename: &str, cpu: &CPU, debug: bool) {
    // load kernel image from disk
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let mut machine_state = MachineState::new();

    // create zero page and copy setup header into it
    let setup_header_end: usize = 0x202 + buffer[0x201] as usize;
    let setup_header = &buffer[SETUP_HEADER_OFFSET as usize..setup_header_end];
    machine_state.mem_write(ZERO_PAGE_ADDRESS + SETUP_HEADER_OFFSET, setup_header);
    machine_state.rsi = ZERO_PAGE_ADDRESS as i64;

    // set kernel command line
    machine_state.mem_write(COMMAND_LINE_ADDRESS, b"debug earlyprintk=vga");
    machine_state.mem_write(ZERO_PAGE_ADDRESS + 0x228, &convert_i32_to_u8vec(COMMAND_LINE_ADDRESS as i32));

    // set video mode
    machine_state.mem_write(ZERO_PAGE_ADDRESS + 0x01, &vec![9]); // screeninfo.y
    machine_state.mem_write(ZERO_PAGE_ADDRESS + 0x06, &vec![6]); // screeninfo.orig_video_mode
    machine_state.mem_write(ZERO_PAGE_ADDRESS + 0x07, &vec![80]); // screeninfo.orig_video_cols
    machine_state.mem_write(ZERO_PAGE_ADDRESS + 0x0e, &vec![25]); // screeninfo.orig_video_lines

    // load code at 0x100.000
    // count of setup sects is the first value in the setup header struct
    let setup_sect = buffer[SETUP_HEADER_OFFSET as usize];
    let offset = (setup_sect + 1) as usize * 512;

    machine_state.mem_write(LOAD_ADDRESS as u64, &buffer[offset..]);
    machine_state.rip = (LOAD_ADDRESS + BIT64_OFFSET) as i64;

    // start execution
    let mut decoder = Decoder::new(cpu, &mut machine_state);
    decoder.execute(debug);
}
