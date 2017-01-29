use instruction_set::InstructionArgument;

use cpu::CPU;

impl CPU {
    pub fn push(&mut self, arg: InstructionArgument) {
        println!("PUSH {:?}", arg);
    }

    pub fn mov(&mut self, arg: InstructionArgument) {
        println!("MOV {:?}", arg);
    }

    pub fn add(&mut self, arg: InstructionArgument) {
        println!("ADD {:?}", arg);
    }

    pub fn or(&mut self, arg: InstructionArgument) {
        println!("OR {:?}", arg);
    }

    pub fn adc(&mut self, arg: InstructionArgument) {
        println!("ADC {:?}", arg);
    }

    pub fn sbb(&mut self, arg: InstructionArgument) {
        println!("SBB {:?}", arg);
    }

    pub fn and(&mut self, arg: InstructionArgument) {
        println!("AND {:?}", arg);
    }

    pub fn sub(&mut self, arg: InstructionArgument) {
        println!("SUB {:?}", arg);
    }

    pub fn xor(&mut self, arg: InstructionArgument) {
        println!("XOR {:?}", arg);
    }

    pub fn cmp(&mut self, arg: InstructionArgument) {
        println!("CMP {:?}", arg);
    }

    pub fn call(&mut self, arg: InstructionArgument) {
        println!("CALL {:?}", arg);
    }

    pub fn lea(&mut self, arg: InstructionArgument) {
        println!("LEA {:?}", arg);
    }

    pub fn test(&mut self, arg: InstructionArgument) {
        println!("TEST {:?}", arg);
    }

    pub fn cmov(&mut self, arg: InstructionArgument) {
        println!("CMOV {:?}", arg);
    }

    pub fn sar(&mut self, arg: InstructionArgument) {
        println!("SAR {:?}", arg);
    }

    pub fn ret(&mut self) {
        println!("RET");
    }

    pub fn leave(&mut self) {
        println!("LEAVE");
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
}
