extern crate x86emu;
use x86emu::machine_state::MachineState;

fn main() {
    let mut ms = MachineState::new(vec![]);
    ms.mem_write(0x1000, vec![0,1,2,3]);
    println!("{:?}", ms.mem_read(0x1001, 4));
    println!("{:?}", ms.mem_read(0x2000, 4));
}
