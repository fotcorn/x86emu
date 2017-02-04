#[derive(Clone, Copy, Debug)]
pub enum RegisterSize {
    Bit32,
    Bit64,
}

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

    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,
}

#[derive(Debug)]
pub enum InstructionArgument {
    OneRegister { register: Register },
    TwoRegister { register1: Register, register2: Register, displacement: i32 },
    Immediate32 { immediate: i32 },
    Immediate8BitRegister {immediate: u8, register: Register, opcode: u8, displacement: i32 },
    Immediate32BitRegister {immediate: i32, register: Register, displacement: i32, opcode: u8 },
}
