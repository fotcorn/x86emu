use std::fmt;
use std::io::prelude::*;
use std::fs::File;

use fnv::FnvHashMap;
use bincode::{serialize, deserialize, Infinite};
use zero;

use instruction_set::{InstructionArgument, Register, Flags, ArgumentSize};
use utils::{convert_i8_to_u8vec, convert_i16_to_u8vec, convert_i32_to_u8vec, convert_i64_to_u8vec};

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
    pub idt: i64,

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
            idt: 0,

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
        for i in 0..8 {
            parity ^= (byte >> i) & 0b1
        }
        self.set_flag(Flags::Parity, parity != 0b1)
    }

    pub fn get_value(&mut self, arg: &InstructionArgument, argument_size: ArgumentSize) -> i64 {
        match *arg {
            InstructionArgument::Register { ref register } => self.get_register_value(register),
            InstructionArgument::Immediate { immediate } => immediate,
            InstructionArgument::EffectiveAddress { .. } => {
                let address = self.calculate_effective_address(arg);
                match argument_size {
                    ArgumentSize::Bit8 => self.mem_read_byte(address) as i64,
                    ArgumentSize::Bit16 => {
                        let mut value: i16 = 0;
                        let val = self.mem_read(address, 2);

                        for (i, v) in val.iter().enumerate() {
                            value |= (*v as i16) << (i * 8);
                        }
                        value as i64
                    }
                    ArgumentSize::Bit32 => {
                        let mut value: i32 = 0;
                        let val = self.mem_read(address, 4);

                        for (i, v) in val.iter().enumerate() {
                            value |= (*v as i32) << (i * 8);
                        }
                        value as i64
                    }
                    ArgumentSize::Bit64 => {
                        let mut value: i64 = 0;
                        let val = self.mem_read(address, 8);

                        for (i, v) in val.iter().enumerate() {
                            value |= (*v as i64) << (i * 8);
                        }
                        value
                    }
                }
            }
        }
    }

    pub fn get_register_value(&self, register: &Register) -> i64 {
        match *register {
            Register::RAX => self.rax,
            Register::RBX => self.rbx,
            Register::RCX => self.rcx,
            Register::RDX => self.rdx,
            Register::RSP => self.rsp,
            Register::RBP => self.rbp,
            Register::RSI => self.rsi,
            Register::RDI => self.rdi,

            Register::R8 => self.r8,
            Register::R9 => self.r9,
            Register::R10 => self.r10,
            Register::R11 => self.r11,
            Register::R12 => self.r12,
            Register::R13 => self.r13,
            Register::R14 => self.r14,
            Register::R15 => self.r15,

            Register::CR0 => self.cr0,
            Register::CR2 => self.cr2,
            Register::CR3 => self.cr3,
            Register::CR4 => self.cr4,
            Register::CR8 => self.cr8,

            Register::RIP => self.rip as i64,

            // 32 Bit
            Register::EAX => self.rax as i32 as i64,
            Register::EBX => self.rbx as i32 as i64,
            Register::ECX => self.rcx as i32 as i64,
            Register::EDX => self.rdx as i32 as i64,
            Register::ESP => self.rsp as i32 as i64,
            Register::EBP => self.rbp as i32 as i64,
            Register::ESI => self.rsi as i32 as i64,
            Register::EDI => self.rdi as i32 as i64,

            Register::R8D => self.r8 as i32 as i64,
            Register::R9D => self.r9 as i32 as i64,
            Register::R10D => self.r10 as i32 as i64,
            Register::R11D => self.r11 as i32 as i64,
            Register::R12D => self.r12 as i32 as i64,
            Register::R13D => self.r13 as i32 as i64,
            Register::R14D => self.r14 as i32 as i64,
            Register::R15D => self.r15 as i32 as i64,

            // 16 Bit
            Register::AX => self.rax as i16 as i64,
            Register::BX => self.rbx as i16 as i64,
            Register::CX => self.rcx as i16 as i64,
            Register::DX => self.rdx as i16 as i64,
            Register::SP => self.rsp as i16 as i64,
            Register::BP => self.rbp as i16 as i64,
            Register::SI => self.rsi as i16 as i64,
            Register::DI => self.rdi as i16 as i64,

            Register::R8W => self.r8 as i16 as i64,
            Register::R9W => self.r9 as i16 as i64,
            Register::R10W => self.r10 as i16 as i64,
            Register::R11W => self.r11 as i16 as i64,
            Register::R12W => self.r12 as i16 as i64,
            Register::R13W => self.r13 as i16 as i64,
            Register::R14W => self.r14 as i16 as i64,
            Register::R15W => self.r15 as i16 as i64,

            // 8 Bit
            Register::AL => self.rax as i8 as i64,
            Register::CL => self.rcx as i8 as i64,
            Register::DL => self.rdx as i8 as i64,
            Register::BL => self.rbx as i8 as i64,
            Register::AH => (self.rax as i16 >> 8) as i64,
            Register::CH => (self.rcx as i16 >> 8) as i64,
            Register::DH => (self.rdx as i16 >> 8) as i64,
            Register::BH => (self.rbx as i16 >> 8) as i64,

            Register::R8B => self.r8 as i8 as i64,
            Register::R9B => self.r9 as i8 as i64,
            Register::R10B => self.r10 as i8 as i64,
            Register::R11B => self.r11 as i8 as i64,
            Register::R12B => self.r12 as i8 as i64,
            Register::R13B => self.r13 as i8 as i64,
            Register::R14B => self.r14 as i8 as i64,
            Register::R15B => self.r15 as i8 as i64,

            Register::SPL => self.rsp as i8 as i64,
            Register::BPL => self.rbp as i8 as i64,
            Register::SIL => self.rsi as i8 as i64,
            Register::DIL => self.rdi as i8 as i64,

            Register::ES => 0,
            Register::CS => 0,
            Register::SS => 0,
            Register::DS => 0,
            Register::FS => 0,
            Register::GS => 0,
        }
    }

    pub fn set_register_value(&mut self, register: &Register, value: i64) {
        match *register {
            // 64 Bit
            Register::RAX => self.rax = value,
            Register::RBX => self.rbx = value,
            Register::RCX => self.rcx = value,
            Register::RDX => self.rdx = value,
            Register::RSP => self.rsp = value,
            Register::RBP => self.rbp = value,
            Register::RSI => self.rsi = value,
            Register::RDI => self.rdi = value,

            Register::R8 => self.r8 = value,
            Register::R9 => self.r9 = value,
            Register::R10 => self.r10 = value,
            Register::R11 => self.r11 = value,
            Register::R12 => self.r12 = value,
            Register::R13 => self.r13 = value,
            Register::R14 => self.r14 = value,
            Register::R15 => self.r15 = value,

            Register::CR0 => {
                println!("CR0: {:x}", value);
                self.cr0 = value
            },
            Register::CR2 => {
                println!("CR2: {:x}", value);
                self.cr2 = value
            },
            Register::CR3 => {
                println!("CR3: {:x}", value);
                self.cr3 = value
            },
            Register::CR4 => {
                println!("CR4: {:x}", value);
                self.cr4 = value
            },
            Register::CR8 => {
                println!("CR5: {:x}", value);
                self.cr8 = value
            },

            Register::RIP => self.rip = value,

            // 32 Bit
            Register::EAX => self.rax = value as u32 as u64 as i64,
            Register::EBX => self.rbx = value as u32 as u64 as i64,
            Register::ECX => self.rcx = value as u32 as u64 as i64,
            Register::EDX => self.rdx = value as u32 as u64 as i64,
            Register::ESP => self.rsp = value as u32 as u64 as i64,
            Register::EBP => self.rbp = value as u32 as u64 as i64,
            Register::ESI => self.rsi = value as u32 as u64 as i64,
            Register::EDI => self.rdi = value as u32 as u64 as i64,

            Register::R8D => self.r8 = value as u32 as u64 as i64,
            Register::R9D => self.r9 = value as u32 as u64 as i64,
            Register::R10D => self.r10 = value as u32 as u64 as i64,
            Register::R11D => self.r11 = value as u32 as u64 as i64,
            Register::R12D => self.r12 = value as u32 as u64 as i64,
            Register::R13D => self.r13 = value as u32 as u64 as i64,
            Register::R14D => self.r14 = value as u32 as u64 as i64,
            Register::R15D => self.r15 = value as u32 as u64 as i64,

            // 16 Bit
            Register::AX => self.rax = ((self.rax as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::BX => self.rbx = ((self.rbx as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::CX => self.rcx = ((self.rcx as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::DX => self.rdx = ((self.rdx as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::SP => self.rsp = ((self.rsp as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::BP => self.rbp = ((self.rbp as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::SI => self.rsi = ((self.rsi as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::DI => self.rdi = ((self.rdi as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,

            Register::R8W => self.r8 = ((self.r8 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::R9W => self.r9 = ((self.r9 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::R10W => self.r10 = ((self.r10 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::R11W => self.r11 = ((self.r11 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::R12W => self.r12 = ((self.r12 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::R13W => self.r13 = ((self.r13 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::R14W => self.r14 = ((self.r14 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,
            Register::R15W => self.r15 = ((self.r15 as u64 & 0xFFFFFFFFFFFF0000) | (value as u16 as u64)) as i64,

            // 8 Bit
            Register::AL => self.rax = ((self.rax as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::CL => self.rcx = ((self.rcx as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::DL => self.rdx = ((self.rdx as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::BL => self.rbx = ((self.rbx as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::AH => self.rax = ((self.rax as u64 & 0xFFFFFFFFFFFF00FF) |
                            ((value as u8 as u64) << 8)) as i64,
            Register::CH => self.rcx = ((self.rcx as u64 & 0xFFFFFFFFFFFF00FF) |
                            ((value as u8 as u64) << 8)) as i64,
            Register::DH => self.rdx = ((self.rdx as u64 & 0xFFFFFFFFFFFF00FF) |
                            ((value as u8 as u64) << 8)) as i64,
            Register::BH => self.rbx = ((self.rbx as u64 & 0xFFFFFFFFFFFF00FF) |
                            ((value as u8 as u64) << 8)) as i64,

            Register::R8B => self.r8 = ((self.r8 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::R9B => self.r9 = ((self.r9 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::R10B => self.r10 = ((self.r10 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::R11B => self.r11 = ((self.r11 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::R12B => self.r12 = ((self.r12 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::R13B => self.r13 = ((self.r13 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::R14B => self.r14 = ((self.r14 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::R15B => self.r15 = ((self.r15 as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,

            Register::SPL => self.rsp = ((self.rsp as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::BPL => self.rbp = ((self.rbp as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::SIL => self.rsi = ((self.rsi as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,
            Register::DIL => self.rdi = ((self.rdi as u64 & 0xFFFFFFFFFFFFFF00) | (value as u8 as u64)) as i64,

            Register::ES => (),
            Register::CS => (),
            Register::SS => (),
            Register::DS => (),
            Register::FS => (),
            Register::GS => (),
        }
    }

    // stack operations
    pub fn stack_push(&mut self, data: &[u8]) {
        let rsp = self.rsp - data.len() as i64;
        self.mem_write(rsp as u64, data);
        self.rsp = rsp;
    }

    pub fn stack_pop(&mut self) -> i64 {
        let rsp = self.rsp as u64;
        let data = self.mem_read(rsp, 8);
        self.rsp += 8;
        *zero::read::<i64>(&data)
    }

    pub fn set_value(&mut self,
                     value: i64,
                     arg: &InstructionArgument,
                     argument_size: ArgumentSize) {
        match *arg {
            InstructionArgument::Register { ref register } => {
                self.set_register_value(register, value)
            }
            InstructionArgument::EffectiveAddress { .. } => {
                let address = self.calculate_effective_address(arg);
                let vector = match argument_size {
                    ArgumentSize::Bit8 => convert_i8_to_u8vec(value as i8),
                    ArgumentSize::Bit16 => convert_i16_to_u8vec(value as i16),
                    ArgumentSize::Bit32 => convert_i32_to_u8vec(value as i32),
                    ArgumentSize::Bit64 => convert_i64_to_u8vec(value),
                };

                self.mem_write(address, &vector);
            }
            InstructionArgument::Immediate { .. } => panic!("Cannot set value on immediate value"),
        }
    }

    pub fn calculate_effective_address(&self, arg: &InstructionArgument) -> u64 {
        match *arg {
            InstructionArgument::EffectiveAddress { ref base, ref index, scale, displacement} => {
                let mut address = match *base {
                    Some(ref base) => self.get_register_value(&base),
                    None => 0,
                };
                address += match *index {
                    None => 0,
                    Some(ref index) => self.get_register_value(index) * scale.unwrap() as i64,
                };
                address += displacement as i64;
                address as u64
            }
            _ => unreachable!(),
        }
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
