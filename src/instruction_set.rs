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

    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

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

pub enum Flags {
    Carry = 1 << 0,
    Direction = 1 << 10,
}

#[derive(Debug, Copy, Clone)]
pub enum ArgumentSize {
    Bit64,
    Bit32,
    Bit16,
    Bit8,
}

pub fn get_register_size(reg: &Register) -> ArgumentSize {
    match *reg {
        Register::RAX | Register::RBX | Register::RCX | Register::RDX | Register::RSP |
        Register::RBP | Register::RSI | Register::RDI | Register::RIP | Register::R8 |
        Register::R9 | Register::R10 | Register::R11 | Register::R12 | Register::R13 |
        Register::R14 | Register::R15 => ArgumentSize::Bit64,
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
pub enum InstructionArgument {
    Immediate { immediate: i64 },
    Register { register: Register },
    EffectiveAddress {
        register: Register,
        displacement: i32,
    },
}

#[derive(Debug)]
pub struct InstructionArguments {
    pub first_argument: InstructionArgument,
    pub second_argument: Option<InstructionArgument>,
    pub opcode: Option<u8>,
    explicit_size: Option<ArgumentSize>,
}

impl InstructionArguments {
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



    pub fn size(&self) -> ArgumentSize {
        match self.explicit_size {
            Some(explicit_size) => explicit_size,
            None => {
                match self.second_argument {
                    Some(ref second_argument) => {
                        match self.first_argument {
                            InstructionArgument::Register { ref register } => {
                                get_register_size(register)
                            }
                            InstructionArgument::Immediate { .. } |
                            InstructionArgument::EffectiveAddress { .. } => {
                                match *second_argument {
                                    InstructionArgument::Register { ref register } => {
                                        get_register_size(register)
                                    }
                                    _ => panic!("Cannot determine instruction argument size"),
                                }
                            }
                        }
                    }
                    None => {
                        match self.first_argument {
                            InstructionArgument::Register { ref register } => {
                                get_register_size(register)
                            }
                            InstructionArgument::Immediate { .. } => ArgumentSize::Bit64,
                            InstructionArgument::EffectiveAddress { .. } => ArgumentSize::Bit64,
                        }
                    }
                }
            }
        }
    }
}

pub struct InstructionArgumentsBuilder {
    first_argument: InstructionArgument,
    second_argument: Option<InstructionArgument>,
    opcode: Option<u8>,
    explicit_size: Option<ArgumentSize>,
}

impl InstructionArgumentsBuilder {
    pub fn new(argument: InstructionArgument) -> InstructionArgumentsBuilder {
        InstructionArgumentsBuilder {
            first_argument: argument,
            second_argument: None,
            opcode: None,
            explicit_size: None,
        }
    }

    pub fn second_argument(mut self,
                           second_argument: InstructionArgument)
                           -> InstructionArgumentsBuilder {
        self.second_argument = Some(second_argument);
        self
    }

    pub fn opcode(mut self, opcode: u8) -> InstructionArgumentsBuilder {
        self.opcode = Some(opcode);
        self
    }

    pub fn explicit_size(mut self, explicit_size: ArgumentSize) -> InstructionArgumentsBuilder {
        self.explicit_size = Some(explicit_size);
        self
    }

    pub fn finalize(self) -> InstructionArguments {
        InstructionArguments {
            first_argument: self.first_argument,
            second_argument: self.second_argument,
            opcode: self.opcode,
            explicit_size: self.explicit_size,
        }
    }
}

impl fmt::Display for InstructionArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.second_argument {
            Some(ref second_argument) => write!(f, "{},{}", self.first_argument, second_argument),
            None => write!(f, "{}", self.first_argument),
        }
    }
}

impl fmt::Display for InstructionArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InstructionArgument::Register { ref register } => write!(f, "{}", register),
            InstructionArgument::Immediate { immediate } => write!(f, "$0x{:x}", immediate),
            InstructionArgument::EffectiveAddress { ref register, displacement } => {
                if displacement < 0 {
                    write!(f, "-{:#x}({})", displacement.abs(), register)
                } else if displacement > 0 {
                    write!(f, "{:#x}({})", displacement, register)
                } else {
                    write!(f, "({})", register)
                }
            }
        }
    }
}
