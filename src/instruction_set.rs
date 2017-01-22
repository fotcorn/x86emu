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
    TwoRegister { register1: Register, register2: Register, displacement: i32 },
    Immediate8BitRegister {immediate: u8, register: Register, opcode: u8, displacement: i32 },
    Immediate32BitRegister {immediate: i32, register: Register, displacement: i32, opcode: u8 },
}
