use std::fmt;
use std::io::prelude::*;
use std::fs::File;

use fnv::FnvHashMap;

use instruction_set::{Flags, ArgumentSize};

use bincode::{serialize, deserialize, Infinite};

#[derive(Serialize, Deserialize)]
pub struct MachineState {
    pub rip: i64,

    pub rax: i64,
    pub rbx: i64,
    pub rcx: i64,
    pub rdx: i64,
    pub rsp: i64,
    pub rbp: i64,
    pub rsi: i64,
    pub rdi: i64,

    pub r8: i64,
    pub r9: i64,
    pub r10: i64,
    pub r11: i64,
    pub r12: i64,
    pub r13: i64,
    pub r14: i64,
    pub r15: i64,

    pub rflags: i64,

    pub cr0: i64,
    pub cr2: i64,
    pub cr3: i64,
    pub cr4: i64,
    pub cr8: i64,

    pub gdt: i64,

    pub print_instructions: bool,
    pub print_registers: bool,

    pub memory: FnvHashMap<u64, Vec<u8>>,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            rip: 0,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsp: 0,
            rbp: 0,
            rsi: 0,
            rdi: 0,

            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,

            rflags: 0,

            cr0: 0,
            cr2: 0,
            cr3: 0,
            cr4: 0,
            cr8: 0,

            gdt: 0,

            print_instructions: false,
            print_registers: false,

            memory: FnvHashMap::default(),
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        let f = flag as i64;
        self.rflags & f == f
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            self.rflags |= flag as i64;
        } else {
            self.rflags &= !(flag as i64);
        }
    }

    pub fn compute_flags(&mut self, result: i64, argument_size: ArgumentSize) {
        self.set_flag(Flags::Zero, result == 0);
        let sign = match argument_size {
            ArgumentSize::Bit8 => (result as u64) & 0x80 != 0,
            ArgumentSize::Bit16 => (result as u64) & 0x8000 != 0,
            ArgumentSize::Bit32 => (result as u64) & 0x80000000 != 0,
            ArgumentSize::Bit64 => (result as u64) & 0x8000000000000000 != 0,
        };
        self.set_flag(Flags::Sign, sign);


        let byte = result as u8;
        let mut parity = 0;
        for i in 0..7 {
            parity ^= (byte >> i) & 0b1
        }
        self.set_flag(Flags::Parity, parity != 0b1)
    }
}

impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "rax            {:#x}\n\
                rbx            {:#x}\n\
                rcx            {:#x}\n\
                rdx            {:#x}\n\
                rsi            {:#x}\n\
                rdi            {:#x}\n\
                rbp            {:#x}\n\
                rsp            {:#x}\n\
                r8             {:#x}\n\
                r9             {:#x}\n\
                r10            {:#x}\n\
                r11            {:#x}\n\
                r12            {:#x}\n\
                r13            {:#x}\n\
                r14            {:#x}\n\
                r15            {:#x}\n\
                rip            {:#x}",
               self.rax,
               self.rbx,
               self.rcx,
               self.rdx,
               self.rsi,
               self.rdi,
               self.rbp,
               self.rsp,
               self.r8,
               self.r9,
               self.r10,
               self.r11,
               self.r12,
               self.r13,
               self.r14,
               self.r15,
               self.rip,
               )
    }
}

pub fn load_machine_state(file_path: &str) -> MachineState {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    deserialize(&buffer).unwrap()
}

/// Use like code inside a Decoder instruction block:
///    let cache_entry = (Instruction::Mov, Some(argument));
///    let cache_entry = InstructionCache {
///        instruction: cache_entry.0,
///        arguments: cache_entry.1,
///        size: 0,
///    };
///    // execute instruction, if we do not do this rip would already be increased
///    // but the actual instruciton would not have been executed yet.
///    self.execute_instruction(&cache_entry);
///    save_machine_state(self.machine_state, "machine_state.bin");
///    panic!("Dumped!");
/// In other places (e.g. in emu_instructions after an instruction has been executed)
/// just use the function directly.
#[allow(dead_code)]
pub fn save_machine_state(machine_state: &MachineState, file_path: &str) {
    let encoded: Vec<u8> = serialize(machine_state, Infinite).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write(&encoded).unwrap();
}
