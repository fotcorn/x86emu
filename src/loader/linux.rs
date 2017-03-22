use std::fs::File;
use std::io::Read;

use machine_state::MachineState;
use decoder::Decoder;
use cpu::cpu_trait::CPU;

const SETUP_SECT_POSITION: usize = 0x1F1;
const BIT64_OFFSET: usize = 0x200;


// see <linux kernel source>/Documentation/x86/boot.txt and zero-page.txt for documentation of the boot protocol
pub fn linux(filename: &str, cpu: &CPU) {
    /*
    TODO
    copy setup_header to 0x10.000 + 1f1
    load kernel at 0x100.000
    set %rsi to boot_params (0x10.000)
    jmp to 0x100.000 + 0x200
    */
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let setup_sect = buffer[SETUP_SECT_POSITION];

    let mut offset = (setup_sect + 1) as usize * 512;
    offset += BIT64_OFFSET;

    let main_code = &buffer[offset as usize..(offset + 0x10000) as usize];

    let mut machine_state = MachineState::new(main_code.to_vec());
    let mut decoder = Decoder::new(cpu, &mut machine_state);
    decoder.execute();
}
