use instruction_set::{Register, InstructionArgument};
use cpu;

use zero;

pub struct CPU {
    pub instruction_pointer: usize,
    pub code: Vec<u8>,
}

impl CPU {
    pub fn disassemble(&mut self) {
        loop {
            let first_byte = self.code[self.instruction_pointer];

            let mut rex: Option<REX> = None;

            match first_byte {
                0xF0 | 0xF2 | 0xF3 => panic!("Lock and repeat prefixes/Bound prefix not supported"),
                0x2E | 0x3E | 0x36 | 0x26 | 0x64 | 0x65 => panic!("Segment override prefixes/branch hints not supported"),
                0x66 => panic!("Operand-size override prefix not supported"),
                0x67 => panic!("Address-size override prefix not supported"),
                0x40...0x4F => {  // 64bit REX prefix
                    rex = Some(REX{ bits: first_byte });
                    self.instruction_pointer += 1;
                },
                _ => ()
            }

            let first_byte = self.code[self.instruction_pointer];
            let ip_offset: usize = match first_byte {
                opcode @ 0x50...0x57 => {
                    cpu::push(InstructionArgument::OneRegister{ register: get_register(opcode - 0x50) });
                    1
                },
                0x89 => { /* mov */
                    let (argument, ip_offset) = self.get_argument(rex, RegOrOpcode::Register, ImmediateSize::None);
                    cpu::mov(argument);
                    ip_offset
                },
                0x83 => {  /* arithmetic operation (64bit register target, 8bit immediate) */
                    // TODO: other register sized are supported (REX, probably other)
                    let (argument, ip_offset) = self.get_argument(rex, RegOrOpcode::Opcode, ImmediateSize::Bit8);
                    cpu::arithmetic(argument);
                    ip_offset
                },
                0xC7 => {
                    // TODO: this somehow also support 16 bit immediate, investigate how
                    let (argument, ip_offset) = self.get_argument(rex, RegOrOpcode::Opcode, ImmediateSize::Bit32);
                    cpu::mov(argument);
                    ip_offset
                },
                _ => panic!("Unknown instruction: {:x}", first_byte)
            };
            self.instruction_pointer += ip_offset;
        }
    }

    fn get_argument(&self, rex: Option<REX>, reg_or_opcode: RegOrOpcode, immediate_size: ImmediateSize) -> (InstructionArgument, usize) {
        let modrm = self.code[self.instruction_pointer + 1];
        match modrm >> 6 {
            /* effecive address */  0b00 => panic!("effective address not implemented"),
            /* effecive address + 8 bit deplacement */ 0b01 => {
                let register = get_register(modrm & 0b00000111);
                let displacement = self.code[self.instruction_pointer + 2] as i8;
                assert!(reg_or_opcode == RegOrOpcode::Opcode);
                let opcode = (modrm & 0b00111000) >> 3;
                // TODO: based on REX, this could be a 64bit value
                match immediate_size {
                   ImmediateSize::Bit8 => {
                       let immediate = self.code[self.instruction_pointer + 3] as i8;
                        (InstructionArgument::Immediate8BitRegister8BitDisplacement {
                            register: register,
                            displacement: displacement,
                            immediate: immediate,
                            opcode: opcode },
                        3)
                    },
                    ImmediateSize::Bit32 => {
                        let immediate = &self.code[self.instruction_pointer + 3..self.instruction_pointer+7];
                        let immediate = *zero::read::<i32>(immediate);
                        (InstructionArgument::Immediate32BitRegister8BitDisplacement {
                            register: register,
                            displacement: displacement,
                            immediate: immediate,
                            opcode: opcode },
                        8)
                    },
                    _ => panic!("Unsupported immediate size"),
                }
            }
            /* effecive address + 32 bit displacement */ 0b10 => panic!("effective address 32bit displacement not implemented"),
            /* register */ 0b11 => {
                let register1 = get_register(modrm & 0b00000111);
                let value2 = (modrm & 0b00111000) >> 3;
                match reg_or_opcode {
                    RegOrOpcode::Register => {
                        (InstructionArgument::TwoRegister{ register1: register1, register2: get_register(value2) }, 2)
                    },
                    // TODO: why do we now here that this is an 8 bit immediate code?
                    RegOrOpcode::Opcode => 
                        (InstructionArgument::Immediate8BitRegister {
                            register: register1,
                            opcode: value2,
                            immediate: self.code[self.instruction_pointer + 2]
                        },
                        3)
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum RegOrOpcode {
    Register,
    Opcode,
}

#[derive(PartialEq)]
enum ImmediateSize {
    None,
    Bit8,
    Bit32,
}

bitflags! {
    flags REX: u8 {
        const OPERAND_64_BIT = 0b00001000,
        const MOD_R_M_EXTENSION = 0b00000100,
        const SIB_EXTENSION = 0b00000010,
        const B = 0b00000001,
    }
}

fn get_register(num: u8) -> Register {
    match num {
        0 => Register::RAX,
        1 => Register::RBX,
        2 => Register::RCX,
        3 => Register::RDX,
        4 => Register::RSP,
        5 => Register::RBP,
        6 => Register::RSI,
        7 => Register::RDI,
        _ => panic!("Unknown instruction argument"),
    }
}
