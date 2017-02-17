use instruction_set::InstructionArgument;

use cpu::CPU;

impl CPU {
    pub fn push(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "push", arg);
    }

    pub fn mov(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "mov", arg);
    }

    pub fn add(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "add", arg);
    }

    pub fn or(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "or", arg);
    }

    pub fn adc(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "adc", arg);
    }

    pub fn sbb(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "sbb", arg);
    }

    pub fn and(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "and", arg);
    }

    pub fn sub(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "sub", arg);
    }

    pub fn xor(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "xor", arg);
    }

    pub fn cmp(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "cmp", arg);
    }

    pub fn call(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "call", arg);
    }

    pub fn lea(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "lea", arg);
    }

    pub fn test(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "test", arg);
    }

    pub fn cmov(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "cmov", arg);
    }

    pub fn sar(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "sar", arg);
    }

    pub fn inc(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "inc", arg);
    }

    pub fn dec(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "dec", arg);
    }

    pub fn ret(&mut self) {
        println!("{:<6}", "ret");
    }

    pub fn leave(&mut self) {
        println!("{:<6}", "leave");
    }

    pub fn jmp(&mut self, arg: InstructionArgument) {
        println!("{:<6} {}", "jmp", arg);
        match arg {
            InstructionArgument::Immediate32 { immediate } => {
                self.instruction_pointer += immediate as usize
            }
            _ => panic!("JMP: Unsupported argument."),
        }
    }

    pub fn arithmetic(&mut self, arg: InstructionArgument) {
        let opcode = match arg {
            InstructionArgument::Immediate8BitRegister { opcode, .. } => opcode,
            InstructionArgument::Immediate32BitRegister { opcode, .. } => opcode,
            _ => panic!("Unsupported argument type for arithmetic"),
        };
        match opcode {
            0 => self.add(arg),
            1 => self.or(arg),
            2 => self.adc(arg),
            3 => self.sbb(arg),
            4 => self.and(arg),
            5 => self.sub(arg),
            6 => self.xor(arg),
            7 => self.cmp(arg),
            _ => unreachable!(),
        }
    }

    pub fn register_operation(&mut self, arg: InstructionArgument) {
        let opcode = match arg {
            InstructionArgument::OneRegister { opcode, .. } => opcode,
            _ => panic!("Unsupported argument type for register operation"),
        };
        match opcode {
            0 => self.inc(arg),
            1 => self.dec(arg),
            2 => self.call(arg),
            3 => self.call(arg), // far call
            4 => self.jmp(arg),
            5 => self.jmp(arg), // far jmp
            6 => self.push(arg),
            _ => unreachable!(),
        }
    }
}
