use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum RegisterSize {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
    Segment,
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    // 64 Bit
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

    CR0,
    CR2,
    CR3,
    CR4,
    CR8,

    // 32 Bit
    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,

    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D,

    // 32 Bit
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W,

    // 16 Bit
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,

    SPL,
    BPL,
    SIL,
    DIL,

    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B,

    ES,
    CS,
    SS,
    DS,
    FS,
    GS,
}

pub enum Flags {
    Carry = 1 << 0,
    Parity = 1 << 2,
    Zero = 1 << 6,
    Sign = 1 << 7,
    Direction = 1 << 10,
    Overflow = 1 << 11,
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
        Register::R14 | Register::R15 | Register::CR0 | Register::CR2 | Register::CR3 |
        Register::CR4 | Register::CR8 => ArgumentSize::Bit64,

        Register::EAX | Register::EBX | Register::ECX | Register::EDX | Register::ESP |
        Register::EBP | Register::ESI | Register::EDI | Register::R8D | Register::R9D |
        Register::R10D | Register::R11D | Register::R12D | Register::R13D | Register::R14D |
        Register::R15D => ArgumentSize::Bit32,

        Register::AX | Register::CX | Register::DX | Register::BX | Register::SP |
        Register::BP | Register::SI | Register::DI | Register::R8W | Register::R9W |
        Register::R10W | Register::R11W | Register::R12W | Register::R13W | Register::R14W |
        Register::R15W | Register::ES | Register::CS | Register::SS | Register::DS |
        Register::FS | Register::GS => ArgumentSize::Bit16,

        Register::AL | Register::CL | Register::DL | Register::BL | Register::AH |
        Register::CH | Register::DH | Register::BH | Register::SPL | Register::BPL |
        Register::SIL | Register::DIL | Register::R8B | Register::R9B |
        Register::R10B | Register::R11B | Register::R12B | Register::R13B | Register::R14B |
        Register::R15B => ArgumentSize::Bit8,
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep = format!("{:?}", self).to_lowercase();
        write!(f, "%{}", rep)
    }
}


pub enum InstructionArgument {
    Immediate { immediate: i64 },
    Register { register: Register },
    EffectiveAddress {
        base: Option<Register>,
        index: Option<Register>,
        scale: Option<u8>,
        displacement: i32,
    },
}

impl InstructionArgument {
    pub fn format(&self, size: ArgumentSize) -> String {
        match *self {
            InstructionArgument::Register {..} | InstructionArgument::EffectiveAddress {..} => format!("{}", self),
            InstructionArgument::Immediate { immediate } => {
                format!("$0x{:x}", match size {
                    ArgumentSize::Bit8 => immediate as u8 as u64,
                    ArgumentSize::Bit16 => immediate as u16 as u64,
                    ArgumentSize::Bit32 => immediate as u32 as u64,
                    ArgumentSize::Bit64 => immediate as u64,
                })
            }
        }
    }
}

pub struct InstructionArguments {
    pub first_argument: Option<InstructionArgument>,
    pub second_argument: Option<InstructionArgument>,
    pub third_argument: Option<InstructionArgument>,
    pub opcode: Option<u8>,
    pub explicit_size: Option<ArgumentSize>,
    pub repeat: bool,
}

impl InstructionArguments {
    pub fn get_one_argument(&self) -> &InstructionArgument {
        let first_argument = match self.first_argument {
            Some(ref first_argument) => first_argument,
            None => panic!("Instructions needs one argument"),
        };
        match self.second_argument {
            Some(_) => panic!("Instruction accepts only one argument"),
            None => (),
        };
        first_argument
    }

    pub fn get_two_arguments(&self) -> (&InstructionArgument, &InstructionArgument) {
        let first_argument = match self.first_argument {
            Some(ref first_argument) => first_argument,
            None => panic!("Instruction needs first_argument"),
        };
        let second_argument = match self.second_argument {
            Some(ref first_argument) => first_argument,
            None => panic!("Instruction needs second_argument"),
        };    
        (first_argument, second_argument)
    }

    pub fn size(&self) -> ArgumentSize {
        match self.explicit_size {
            Some(explicit_size) => explicit_size,
            None => {
                match self.second_argument {
                    Some(ref second_argument) => {
                        match self.first_argument {
                            Some(ref first_argument) => {
                                match *first_argument {
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
                            },
                            None => panic!("Instructions with second_argument also need a first_argument"),
                        }
                    },
                    None => {
                        match self.first_argument {
                            Some(ref first_argument) => {
                                match *first_argument {
                                    InstructionArgument::Register { ref register } => {
                                        get_register_size(register)
                                    }
                                    InstructionArgument::Immediate { .. } => ArgumentSize::Bit64,
                                    InstructionArgument::EffectiveAddress { .. } => ArgumentSize::Bit64,
                                }
                            },
                            None => panic!("Instructions without arguments needs explicit_size set"),
                        }
                    }
                }
            }
        }
    }
}

pub struct InstructionArgumentsBuilder {
    first_argument: Option<InstructionArgument>,
    second_argument: Option<InstructionArgument>,
    opcode: Option<u8>,
    explicit_size: Option<ArgumentSize>,
    repeat: bool,
}

impl InstructionArgumentsBuilder {
    pub fn new() -> InstructionArgumentsBuilder {
        InstructionArgumentsBuilder {
            first_argument: None,
            second_argument: None,
            opcode: None,
            explicit_size: None,
            repeat: false,
        }
    }

    pub fn first_argument(mut self,
                           first_argument: InstructionArgument)
                           -> InstructionArgumentsBuilder {
        self.first_argument = Some(first_argument);
        self
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
            third_argument: None,
            opcode: self.opcode,
            explicit_size: self.explicit_size,
            repeat: self.repeat,
        }
    }

    pub fn repeat(mut self, repeat: bool) -> InstructionArgumentsBuilder {
        self.repeat = repeat;
        self
    }
}

impl fmt::Display for InstructionArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.second_argument {
            Some(ref second_argument) => {
                match self.first_argument {
                    Some(ref first_argument) => write!(f, "{},{}", first_argument.format(self.size()), second_argument.format(self.size())),
                    None => panic!("Instructions with second_argument also need a first_argument"),
                }
            },
            None => {
                match self.first_argument {
                    Some(ref first_argument) => write!(f, "{}", first_argument.format(self.size())),
                    None =>  write!(f, ""),
                }
            },
        }
    }
}

impl fmt::Display for InstructionArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InstructionArgument::Register { ref register } => write!(f, "{}", register),
            InstructionArgument::Immediate { immediate } => write!(f, "$0x{:x}", immediate),
            InstructionArgument::EffectiveAddress { displacement, .. } => {
                if displacement < 0 {
                    write!(f, "-{:#x}{}", displacement.abs(), format_effective_address(self))
                } else if displacement > 0 {
                    write!(f, "{:#x}{}", displacement, format_effective_address(self))
                } else {
                    write!(f, "0x0{}", format_effective_address(self))
                }
            }
        }
    }
}

fn format_effective_address(arg: &InstructionArgument) -> String {
    match *arg {
        InstructionArgument::EffectiveAddress { ref base, ref index, scale, .. } => {
            match *index {
                None => {
                    match *base {
                        Some(ref base) => format!("({})", base),
                        None => format!(""),
                    }
                }
                Some(ref index) => {
                    match *base {
                        Some(ref base) => format!("({},{},{})", base, index, scale.unwrap()),
                        None => format!("(,{},{})", index, scale.unwrap()),
                    }
                }
            }
        },
        _ => unreachable!()
    }
}

pub struct InstructionCache {
    pub instruction: Instruction,
    pub arguments: Option<InstructionArguments>,
    pub size: u64
}

pub enum Instruction {
    Adc,
    Add,
    And,
    Arithmetic,
    BitManipulation,
    Call,
    Cld,
    Cmova,
    Cmovae,
    Cmovb,
    Cmovbe,
    Cmove,
    Cmovg,
    Cmovge,
    Cmovl,
    Cmovle,
    Cmovne,
    Cmovno,
    Cmovnp,
    Cmovns,
    Cmovo,
    Cmovp,
    Cmovs,
    Cmp,
    CompareMulOperation,
    Cpuid,
    Imul,
    Int,
    Ja,
    Jae,
    Jb,
    Jbe,
    Je,
    Jg,
    Jge,
    Jl,
    Jle,
    Jmp,
    Jne,
    Jno,
    Jnp,
    Jns,
    Jo,
    Jp,
    Js,
    Lea,
    Leave,
    Lgdt,
    Mov,
    Movs,
    Movsx,
    Movzx,
    Nop,
    Or,
    Out,
    Pop,
    Popf,
    Push,
    Pushf,
    Rdmsr,
    RegisterOperation,
    Ret,
    Sbb,
    Sete,
    ShiftRotate,
    Std,
    Stos,
    Sub,
    Test,
    Wrmsr,
    Xor,
}
