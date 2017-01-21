#[derive(Debug)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSP,
    RBP,
    RSI,
    RDI,
}

#[derive(Debug)]
pub enum InstructionArgument {
    OneRegister { register: Register },
    TwoRegister { register1: Register, register2: Register },
    Immediate8BitRegister8BitDisplacement {immediate: i8, register: Register, displacement: i8, opcode: u8 },
    Immediate32BitRegister8BitDisplacement {immediate: i32, register: Register, displacement: i8, opcode: u8 },
    Immediate8BitRegister {immediate: u8, register: Register, opcode: u8 },
}
