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
    Immediate8BitRegister { immediate: u8, register: Register },
}
