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

pub enum ArgumentSize {
    Bit64,
    Bit32,
    Bit16,
    Bit8,
}

pub fn get_register_size(reg: &Register) -> ArgumentSize {
    match *reg {
        Register::RAX | Register::RBX | Register::RCX | Register::RDX | Register::RSP |
        Register::RBP | Register::RSI | Register::RDI | Register::RIP => ArgumentSize::Bit64,
        Register::EAX | Register::EBX | Register::ECX | Register::EDX | Register::ESP |
        Register::EBP | Register::ESI | Register::EDI => ArgumentSize::Bit32,
        Register::ES | Register::CS | Register::SS | Register::DS | Register::FS | Register::GS => {
            ArgumentSize::Bit16
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep = format!("{:?}", self).to_lowercase();
        write!(f, "%{}", rep)
    }
}


#[derive(Debug)]
pub enum  InstructionArgument {
    Immediate { immediate: i64},
    Register { register: Register},
    EffectiveAddress { register: Register, displacement: i32}
}

#[derive(Debug)]
pub struct InstructionArguments {
    pub first_argument: InstructionArgument,
    pub second_argument: Option<InstructionArgument>,
    pub opcode: Option<u8>,
}

impl InstructionArguments {

    pub fn new_one_argument(argument: InstructionArgument) -> InstructionArguments {
        InstructionArguments {
            first_argument: argument,
            second_argument: None,
            opcode: None,
        }
    }

    pub fn new_one_argument_opcode(argument: InstructionArgument, opcode: u8) -> InstructionArguments {
        InstructionArguments {
            first_argument: argument,
            second_argument: None,
            opcode: Some(opcode),
        }
    }

    pub fn new_two_arguments(first_argument: InstructionArgument, second_argument: InstructionArgument) -> InstructionArguments {
        InstructionArguments {
            first_argument: first_argument,
            second_argument: Some(second_argument),
            opcode: None,
        }
    }

    pub fn new_two_arguments_opcode(first_argument: InstructionArgument,
                                    second_argument: InstructionArgument,
                                    opcode: u8) -> InstructionArguments {
        InstructionArguments {
            first_argument: first_argument,
            second_argument: Some(second_argument),
            opcode: Some(opcode),
        }
    }

    pub fn assert_one_argument(&self) {
        match self.second_argument {
            Some(_) => panic!("Instruction accepts only one argument"),
            None => (),
        }
    }

    pub fn assert_two_arguments(&self) {
        match self.second_argument {
            Some(_) => (),
            None => panic!("Instruction requires two arguments"),
        }
    }
}

impl fmt::Display for InstructionArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        /*
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
                    Some(_) => write!(f, "${:#x},({})", immediate, register),
                    None => write!(f, "${:#x},{}", immediate, register),
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
                    Some(_) => write!(f, "${:#x},({})", immediate, register),
                    None => write!(f, "${:#x},{}", immediate, register),
                }
            }
        }*/
    }
}
