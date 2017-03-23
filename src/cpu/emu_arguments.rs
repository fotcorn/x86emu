use instruction_set::{InstructionArgument, Register, ArgumentSize};
use machine_state::MachineState;
use utils::{convert_i32_to_u8vec, convert_i64_to_u8vec};

impl MachineState {
    pub fn get_value(&mut self, arg: &InstructionArgument, argument_size: ArgumentSize) -> i64 {
        match *arg {
            InstructionArgument::Register { ref register } => self.get_register_value(register),
            InstructionArgument::Immediate { immediate } => immediate,
            InstructionArgument::EffectiveAddress { ref register, displacement } => {
                let mut address = self.get_register_value(register);
                address += displacement as i64;
                match argument_size {
                    ArgumentSize::Bit32 => {
                        let mut value: i32 = 0;
                        let val = self.mem_read(address as u64, 4);

                        for (i, v) in val.iter().enumerate() {
                            value |= (*v as i32) << (i * 8);
                        }
                        value as i64
                    }
                    ArgumentSize::Bit64 => {
                        let mut value: i64 = 0;
                        let val = self.mem_read(address as u64, 8);

                        for (i, v) in val.iter().enumerate() {
                            value |= (*v as i64) << (i * 8);
                        }
                        value
                    }
                    _ => panic!("unsupported argument size in set_value/effective address"),
                }
            }
        }
    }

    fn get_register_value(&self, register: &Register) -> i64 {
        match *register {
            Register::RAX => self.rax,
            Register::RBX => self.rbx,
            Register::RCX => self.rcx,
            Register::RDX => self.rdx,
            Register::RSP => self.rsp,
            Register::RBP => self.rbp,
            Register::RSI => self.rsi,
            Register::RDI => self.rdi,

            Register::RIP => self.rip as i64,

            Register::EAX => self.rax as i32 as i64,
            Register::EBX => self.rbx as i32 as i64,
            Register::ECX => self.rcx as i32 as i64,
            Register::EDX => self.rdx as i32 as i64,
            Register::ESP => self.rsp as i32 as i64,
            Register::EBP => self.rbp as i32 as i64,
            Register::ESI => self.rsi as i32 as i64,
            Register::EDI => self.rdi as i32 as i64,

            Register::ES => 0,
            Register::CS => 0,
            Register::SS => 0,
            Register::DS => 0,
            Register::FS => 0,
            Register::GS => 0,
        }
    }

    fn set_register_value(&mut self, register: &Register, value: i64) {
        match *register {
            Register::RAX => self.rax = value,
            Register::RBX => self.rbx = value,
            Register::RCX => self.rcx = value,
            Register::RDX => self.rdx = value,
            Register::RSP => self.rsp = value,
            Register::RBP => self.rbp = value,
            Register::RSI => self.rsi = value,
            Register::RDI => self.rdi = value,

            Register::RIP => self.rip = value,

            Register::EAX => {
                self.rax = ((self.rax as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }
            Register::EBX => {
                self.rbx = ((self.rbx as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }
            Register::ECX => {
                self.rcx = ((self.rcx as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }
            Register::EDX => {
                self.rdx = ((self.rdx as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }
            Register::ESP => {
                self.rsp = ((self.rsp as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }
            Register::EBP => {
                self.rbp = ((self.rbp as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }
            Register::ESI => {
                self.rsi = ((self.rsi as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }
            Register::EDI => {
                self.rdi = ((self.rdi as u64 & 0xFFFFFFFF00000000) | (value as i32 as u64)) as i64
            }

            Register::ES => (),
            Register::CS => (),
            Register::SS => (),
            Register::DS => (),
            Register::FS => (),
            Register::GS => (),
        }
    }

    // stack operations
    pub fn stack_push(&mut self, data: Vec<u8>) {
        let rsp = self.rsp - data.len() as i64;
        self.mem_write(rsp as u64, data);
        self.rsp = rsp;
    }

    pub fn set_value(&mut self,
                     value: i64,
                     arg: &InstructionArgument,
                     argument_size: ArgumentSize) {
        match *arg {
            InstructionArgument::Register { ref register } => {
                self.set_register_value(register, value)
            }
            InstructionArgument::EffectiveAddress { ref register, displacement } => {
                let mut address = self.get_register_value(register);
                address += displacement as i64;
                let vector = match argument_size {
                    ArgumentSize::Bit32 => convert_i32_to_u8vec(value as i32),
                    ArgumentSize::Bit64 => convert_i64_to_u8vec(value),
                    _ => panic!("unsupported argument size in set_value/effective address"),
                };

                self.mem_write(address as u64, vector);
            }
            InstructionArgument::Immediate { .. } => panic!("Cannot set value on immediate value"),
        }
    }
}
