use machine_state::load_machine_state;
use cpu::emu_instructions::EmulationCPU;
use decoder::Decoder;

pub fn dump(filename: &str, print_instructions: bool, print_registers: bool) {
    let mut cpu = EmulationCPU {};

    let mut machine_state = load_machine_state(filename);
    machine_state.print_instructions = print_instructions;
    machine_state.print_registers = print_registers;

    let mut decoder = Decoder::new(&mut cpu, &mut machine_state);
    decoder.execute(false);
}
