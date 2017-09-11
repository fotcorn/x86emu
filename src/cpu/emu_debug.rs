use machine_state::MachineState;
use instruction_set::{InstructionArguments, ArgumentSize};

impl MachineState {
    pub fn print_instr(&self, instruction: &str) {
        if self.print_instructions {
            println!("{:X} {:<6}", self.rip as u64, instruction);
        }
    }

    pub fn print_instr_arg_no_size(&self, instruction: &str, arg: &InstructionArguments) {
        if self.print_instructions {
            println!("{:X} {:<6} {}", self.rip as u64, instruction, arg);
        }
    }

    pub fn print_instr_arg(&self, instruction: &str, arg: &InstructionArguments) {
        if self.print_instructions {
            match arg.explicit_size {
                Some(size) => {
                    match size {
                        ArgumentSize::Bit8 => println!("{:X} {:<6} {}", self.rip as u64, instruction.to_owned() + "b", arg),
                        ArgumentSize::Bit16 => println!("{:X} {:<6} {}", self.rip as u64, instruction.to_owned() + "w", arg),
                        ArgumentSize::Bit32 => println!("{:X} {:<6} {}", self.rip as u64, instruction.to_owned() + "l", arg),
                        ArgumentSize::Bit64 => println!("{:X} {:<6} {}", self.rip as u64, instruction.to_owned() + "q", arg),
                    }
                },
                None => println!("{:X} {:<6} {}", self.rip as u64, instruction, arg),
            }
        }
    }
}
