use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum RegisterSize {
    Bit32,
    Bit64,
    Segment,
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

    RIP,

    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,

    ES,
    CS,
    SS,
    DS,
    FS,
    GS,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep = format!("{:?}", self).to_lowercase();
        write!(f, "%{}", rep)
    }
}


#[derive(Debug)]
pub enum InstructionArgument {
    OneRegister { register: Register, opcode: u8 },
    TwoRegister {
        register1: Register,
        register2: Register,
        effective_address_displacement: Option<i32>,
        reverse_direction: bool,
    },
    Immediate8 { immediate: i8 },
    Immediate32 { immediate: i32 },
    Immediate8BitRegister {
        immediate: u8,
        register: Register,
        opcode: u8,
        effective_address_displacement: Option<i32>,
    },
    Immediate32BitRegister {
        immediate: i32,
        register: Register,
        opcode: u8,
        effective_address_displacement: Option<i32>,
    },
}

impl fmt::Display for InstructionArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InstructionArgument::OneRegister { ref register, .. } => write!(f, "{}", register),
            InstructionArgument::TwoRegister { ref register1,
                                               ref register2,
                                               effective_address_displacement,
                                               reverse_direction } => {
                match effective_address_displacement {
                    Some(displacement) if displacement > 0 => {
                        if reverse_direction {
                            write!(f, "{:#x}({}),{}", displacement, register1, register2)
                        } else {
                            write!(f, "{},{:#x}({})", register2, displacement, register1)
                        }
                    }
                    Some(displacement) if displacement < 0 => {
                        if reverse_direction {
                            write!(f, "-{:#x}({}),{}", displacement * -1, register1, register2)
                        } else {
                            write!(f, "{},-{:#x}({})", register2, displacement * -1, register1)
                        }
                    }
                    Some(_) => {
                        if reverse_direction {
                            write!(f, "({}),{}", register1, register2)
                        } else {
                            write!(f, "{},({})", register2, register1)
                        }
                    }
                    None => {
                        if reverse_direction {
                            write!(f, "{},{}", register1, register2)
                        } else {
                            write!(f, "{},{}", register2, register1)
                        }
                    }
                }
            }
            InstructionArgument::Immediate8 { immediate } => write!(f, "{:x}", immediate),
            InstructionArgument::Immediate32 { immediate } => write!(f, "{:x}", immediate),
            InstructionArgument::Immediate32BitRegister { ref register,
                                                          immediate,
                                                          effective_address_displacement,
                                                          .. } => {
                match effective_address_displacement {
                    Some(displacement) if displacement > 0 => {
                        write!(f, "${:#x},{:#x}({})", immediate, displacement, register)
                    }
                    Some(displacement) if displacement < 0 => {
                        write!(f,
                           "${:#x},-{:#x}({})",
                           immediate,
                           displacement * -1,
                           register)
                    }
                    Some(_) => {
                        write!(f,
                           "${:#x},({})",
                           immediate,
                           register)
                    }
                    None => {
                        write!(f, "${:#x},{}", immediate, register)
                    }
                }
            }
            InstructionArgument::Immediate8BitRegister { ref register,
                                                         immediate,
                                                         effective_address_displacement,
                                                         .. } => {
                match effective_address_displacement {
                    Some(displacement) if displacement > 0 => {
                        write!(f, "${:#x},{:#x}({})", immediate, displacement, register)
                    }
                    Some(displacement) if displacement < 0 => {
                        write!(f,
                           "${:#x},-{:#x}({})",
                           immediate,
                           displacement * -1,
                           register)
                    }
                    Some(_) => {
                        write!(f,
                           "${:#x},({})",
                           immediate,
                           register)
                    }
                    None => {
                        write!(f, "${:#x},{}", immediate, register)
                    }
                }
            }
        }
    }
}
