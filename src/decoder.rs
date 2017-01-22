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
                opcode @ 0xB8...0xBF => {
                    cpu::mov(InstructionArgument::Immediate32BitRegister {
                        register: get_register(opcode - 0xB8),
                        displacement: 0,
                        opcode: 0,
                        immediate : self.get_i32_value(1),
                    });
                    5
                },
                0xE8 => {
                    cpu::call(InstructionArgument::Immediate32 {
                        immediate : self.get_i32_value(1),
                    });
                    5
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
                0x8b => {
                    let (argument, ip_offset) = self.get_argument(rex, RegOrOpcode::Register, ImmediateSize::None);
                    cpu::mov(argument);
                    ip_offset
                },
                _ => panic!("Unknown instruction: {:x}", first_byte)
            };
            self.instruction_pointer += ip_offset;
        }
    }

    fn get_i32_value(&self, ip_offset: usize) -> i32 {
        let value = &self.code[self.instruction_pointer + ip_offset..self.instruction_pointer + ip_offset + 4];
        *zero::read::<i32>(value)
    }

    fn get_argument(&self, rex: Option<REX>, reg_or_opcode: RegOrOpcode, immediate_size: ImmediateSize) -> (InstructionArgument, usize) {
        let modrm = self.code[self.instruction_pointer + 1];
        let address_mod = modrm >> 6;
        match address_mod {
            0b00 | 0b01 | 0b10 => { /* effective address / effecive address + 8 bit deplacement / effecive address + 32 bit deplacement */
                let register = get_register(modrm & 0b00000111);
                
                let (displacement, mut ip_offset) = match address_mod {
                    0b00 => (0, 0),
                    0b01 => (self.code[self.instruction_pointer + 2] as i8 as i32, 1),
                    0b10 => {
                        let displacement = &self.code[self.instruction_pointer + 2..self.instruction_pointer+6];
                        let displacement = *zero::read::<i32>(displacement);
                        (displacement, 4)
                    },
                    _ => unreachable!()
                };
                ip_offset += 2; // skip instruction + modrm byte

                let register_or_opcode = (modrm & 0b00111000) >> 3;
                // TODO: based on REX, this could be a 64bit value
                match immediate_size {
                    ImmediateSize::Bit8 => {
                       assert!(reg_or_opcode == RegOrOpcode::Opcode);
                       let immediate = self.code[self.instruction_pointer + ip_offset];
                        (InstructionArgument::Immediate8BitRegister {
                            register: register,
                            displacement: displacement,
                            immediate: immediate,
                            opcode: register_or_opcode },
                        ip_offset + 1)
                    },
                    ImmediateSize::Bit32 => {
                        assert!(reg_or_opcode == RegOrOpcode::Opcode);
                        let immediate = &self.code[self.instruction_pointer + ip_offset..self.instruction_pointer + ip_offset + 4];
                        let immediate = *zero::read::<i32>(immediate);
                        (InstructionArgument::Immediate32BitRegister {
                            register: register,
                            displacement: displacement,
                            immediate: immediate,
                            opcode: register_or_opcode },
                        ip_offset + 4)
                    },
                    ImmediateSize::None => {
                        assert!(reg_or_opcode == RegOrOpcode::Register);
                        (InstructionArgument::TwoRegister {
                            register1: register,
                            register2: get_register(register_or_opcode),
                            displacement: displacement },
                        ip_offset)
                    }
                }
            }
            0b11 => { /* register */
                let register1 = get_register(modrm & 0b00000111);
                let value2 = (modrm & 0b00111000) >> 3;
                match reg_or_opcode {
                    RegOrOpcode::Register => {
                        (InstructionArgument::TwoRegister{ register1: register1, register2: get_register(value2), displacement: 0 }, 2)
                    },
                    // TODO: why do we now here that this is an 8 bit immediate code?
                    RegOrOpcode::Opcode => 
                        (InstructionArgument::Immediate8BitRegister {
                            register: register1,
                            opcode: value2,
                            immediate: self.code[self.instruction_pointer + 2],
                            displacement: 0,
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
