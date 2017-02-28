use instruction_set::{Register, RegisterSize, InstructionArgument};
use cpu::CPU;

use zero;

impl CPU {
    pub fn execute(&mut self) {
        loop {
            let mut first_byte = self.code[self.instruction_pointer];

            let mut rex: Option<REX> = None;
            let mut decoder_flags = DecoderFlags {bits: 0};

            match first_byte {
                0xF0 | 0xF2 => panic!("Lock prefixes/Bound prefix not supported"),
                0xF3 => {
                    decoder_flags |= REPEAT;
                    self.instruction_pointer += 1;
                }
                0x2E | 0x3E | 0x36 | 0x26 | 0x64 | 0x65 => {
                    panic!("Segment override prefixes/branch hints not supported")
                }
                0x66 => panic!("Operand-size override prefix not supported"),
                0x67 => {
                    decoder_flags |= ADDRESS_SIZE_OVERRIDE;
                    self.instruction_pointer += 1;
                }
                _ => (),
            }

            first_byte = self.code[self.instruction_pointer];
            match first_byte {
                0x40...0x4F => {
                    // 64bit REX prefix
                    rex = Some(REX { bits: first_byte });
                    self.instruction_pointer += 1;
                }
                _ => (),
            }

            let register_size = match rex {
                Some(r) if r.contains(OPERAND_64_BIT) => RegisterSize::Bit64,
                _ => RegisterSize::Bit32,
            };

            let first_byte = self.code[self.instruction_pointer];
            let ip_offset: usize = match first_byte {
                opcode @ 0x50...0x57 => {
                    self.push(InstructionArgument::OneRegister {
                        register: get_register(opcode - 0x50, RegisterSize::Bit64),
                        opcode: 0,
                    });
                    1
                }
                opcode @ 0xB8...0xBF => {
                    let immediate = self.get_i32_value(1);
                    self.mov(InstructionArgument::Immediate32BitRegister {
                        register: get_register(opcode - 0xB8, register_size),
                        displacement: 0,
                        opcode: 0,
                        immediate: immediate,
                    });
                    5
                }
                0x01 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags);
                    self.add(argument);
                    ip_offset
                }
                0x21 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags);
                    self.and(argument);
                    ip_offset
                }
                0x7D => {
                    let immediate = self.code[self.instruction_pointer + 1] as i8;
                    self.jge(InstructionArgument::Immediate8 { immediate: immediate});
                    2
                }
                0x6A => {
                    let immediate = self.code[self.instruction_pointer + 1] as i8;
                    self.push(InstructionArgument::Immediate8 { immediate: immediate});
                    2
                }
                0xE8 => {
                    let immediate = self.get_i32_value(1);
                    self.call(InstructionArgument::Immediate32 { immediate: immediate });
                    5
                }
                0x89 => {
                    // mov
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags);
                    self.mov(argument);
                    ip_offset
                }
                0x85 => {
                    // test
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags);
                    self.test(argument);
                    ip_offset
                }
                0x31 => {
                    // xor
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags);
                    self.xor(argument);
                    ip_offset
                }
                0x81 => {
                    // arithmetic operation (64bit register target, 8bit immediate)
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Opcode,
                                                                  ImmediateSize::Bit32,
                                                                  decoder_flags);
                    self.arithmetic(argument);
                    ip_offset                    
                }
                0x83 => {
                    // arithmetic operation (64bit register target, 8bit immediate)
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Opcode,
                                                                  ImmediateSize::Bit8,
                                                                  decoder_flags);
                    self.arithmetic(argument);
                    ip_offset
                }
                0xC7 => {
                    // TODO: register size can also be 32bit with address_size_override
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit64,
                                                                  RegOrOpcode::Opcode,
                                                                  ImmediateSize::Bit32,
                                                                  decoder_flags);
                    self.mov(argument);
                    ip_offset
                }
                0x8B => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags | REVERSED_REGISTER_DIRECTION);
                    self.mov(argument);
                    ip_offset
                }
                0x8E => {
                    // mov 16bit segment registers
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Segment,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  // TODO: REVERSED_REGISTER_DIRECTION correct?
                                                                  decoder_flags | REVERSED_REGISTER_DIRECTION); 
                    self.mov(argument);
                    ip_offset
                }
                0x8D => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  // TODO: REVERSED_REGISTER_DIRECTION correct?
                                                                  decoder_flags | REVERSED_REGISTER_DIRECTION);
                    self.lea(argument);
                    ip_offset
                }
                0xA5 => {
                    self.movs(true);
                    1
                }
                0xC1 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Opcode,
                                                                  ImmediateSize::Bit8,
                                                                  decoder_flags);
                    self.sar(argument);
                    ip_offset
                }
                0xC3 => {
                    self.ret();
                    1
                }
                0xC9 => {
                    self.leave();
                    1
                }
                0xFD => {
                    self.std();
                    1
                }
                0x9D => {
                    self.popf();
                    1
                }
                0xE9 => {
                    let immediate = self.get_i32_value(1);
                    self.jmp(InstructionArgument::Immediate32 { immediate: immediate });
                    5
                }
                0xF7 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Opcode,
                                                                  ImmediateSize::None,
                                                                  decoder_flags);
                    self.compare_mul_operation(argument);
                    ip_offset
                }
                0xFF => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Opcode,
                                                                  ImmediateSize::None,
                                                                  decoder_flags);
                    self.register_operation(argument);
                    ip_offset
                }
                0x0F => {
                    // two byte instructions
                    let second_byte = self.code[self.instruction_pointer + 1];
                    match second_byte {
                        0x48 => {
                            // TODO: fixme, wrong register + deplacement
                            let (argument, ip_offset) = self.get_argument(register_size,
                                                                          RegOrOpcode::Register,
                                                                          ImmediateSize::None,
                                                                          decoder_flags);
                            self.cmov(argument);
                            ip_offset
                        }
                        _ => panic!("Unknown instruction: 0F {:x}", first_byte),
                    }

                }
                _ => panic!("Unknown instruction: {:x}", first_byte),
            };
            self.instruction_pointer += ip_offset;
        }
    }

    fn get_i32_value(&self, ip_offset: usize) -> i32 {
        let value = &self.code[self.instruction_pointer + ip_offset..
                     self.instruction_pointer + ip_offset + 4];
        *zero::read::<i32>(value)
    }

    fn get_argument(&self,
                    register_size: RegisterSize,
                    reg_or_opcode: RegOrOpcode,
                    immediate_size: ImmediateSize,
                    //address_size_override: bool,
                    //reverse_direction: bool)
                    decoder_flags: DecoderFlags)
                    -> (InstructionArgument, usize) {
        let modrm = self.code[self.instruction_pointer + 1];
        let mut address_mod = modrm >> 6;

        match address_mod {
            0b00 | 0b01 | 0b10 => {
                // effective address / effecive address + 8 bit deplacement /
                // effecive address + 32 bit deplacement
                let rm = modrm & 0b00000111;

                // special case: RIP relative adressing. We fake a 32bit displacement instruction.
                if address_mod == 0b00 && rm == 0x5 {
                    address_mod = 0b10;
                }

                let register = get_register(rm, register_size);

                let (displacement, mut ip_offset) = match address_mod {
                    0b00 => (0, 0),
                    0b01 => (self.code[self.instruction_pointer + 2] as i8 as i32, 1),
                    0b10 => {
                        let displacement = &self.code[self.instruction_pointer + 2..
                                            self.instruction_pointer + 6];
                        let displacement = *zero::read::<i32>(displacement);
                        (displacement, 4)
                    }
                    _ => unreachable!(),
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
                             opcode: register_or_opcode,
                         },
                         ip_offset + 1)
                    }
                    ImmediateSize::Bit32 => {
                        assert!(reg_or_opcode == RegOrOpcode::Opcode);
                        let immediate = &self.code[self.instruction_pointer + ip_offset..
                                         self.instruction_pointer + ip_offset + 4];
                        let immediate = *zero::read::<i32>(immediate);
                        (InstructionArgument::Immediate32BitRegister {
                             register: register,
                             displacement: displacement,
                             immediate: immediate,
                             opcode: register_or_opcode,
                         },
                         ip_offset + 4)
                    }
                    ImmediateSize::None => {
                        assert!(reg_or_opcode == RegOrOpcode::Register);

                        let second_register_size = if decoder_flags.contains(ADDRESS_SIZE_OVERRIDE) {
                            RegisterSize::Bit32
                        } else {
                            RegisterSize::Bit64
                        };

                        // special case: RIP relative adressing.
                        let register1 = if rm == 0x5 {
                            Register::RIP
                        } else {
                            get_register(rm, second_register_size)
                        };
                        let register2 = get_register(register_or_opcode, register_size);

                        (InstructionArgument::TwoRegister {
                             register1: register1,
                             register2: register2,
                             displacement: displacement,
                             reverse_direction: if decoder_flags.contains(REVERSED_REGISTER_DIRECTION) { true } else { false },
                         },
                         ip_offset)
                    }
                }
            }
            0b11 => {
                // register
                let register1 = get_register(modrm & 0b00000111, register_size);
                let value2 = (modrm & 0b00111000) >> 3;
                match reg_or_opcode {
                    RegOrOpcode::Register => {
                        (InstructionArgument::TwoRegister {
                             register1: register1,
                             register2: get_register(value2, register_size),
                             displacement: 0,
                             reverse_direction: false,
                         },
                         2)
                    }
                    RegOrOpcode::Opcode => {
                        match immediate_size {
                            ImmediateSize::Bit8 => {
                                (InstructionArgument::Immediate8BitRegister {
                                     register: register1,
                                     opcode: value2,
                                     immediate: self.code[self.instruction_pointer + 2],
                                     displacement: 0,
                                 },
                                 3)
                            }
                            ImmediateSize::None => {
                                (InstructionArgument::OneRegister {
                                     register: register1,
                                     opcode: value2,
                                 },
                                 2)
                            }
                            ImmediateSize::Bit32 => {
                                (InstructionArgument::Immediate32BitRegister {
                                     register: register1,
                                     opcode: value2,
                                     immediate: self.get_i32_value(2),
                                     displacement: 0,
                                 },
                                 6)
                            }
                        }
                    }
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

bitflags! {
    flags DecoderFlags: u64 {
        const REVERSED_REGISTER_DIRECTION = 0b1,
        const ADDRESS_SIZE_OVERRIDE = 0b10,
        const REPEAT = 0b100,
        // const OPERAND_64_BIT = 0b1000,
        // const REGISTER_EXTENSION = 0b10000,
        // const SIB_EXTENSION = 0b100000,
        // const REX_B = 0b1000000,
    }
}

fn get_register(num: u8, size: RegisterSize) -> Register {
    match size {
        RegisterSize::Bit32 => {
            match num {
                0 => Register::EAX,
                1 => Register::ECX,
                2 => Register::EDX,
                3 => Register::EBX,
                4 => Register::ESP,
                5 => Register::EBP,
                6 => Register::ESI,
                7 => Register::EDI,
                _ => panic!("Unknown instruction argument"),
            }
        }
        RegisterSize::Bit64 => {
            match num {
                0 => Register::RAX,
                1 => Register::RCX,
                2 => Register::RDX,
                3 => Register::RBX,
                4 => Register::RSP,
                5 => Register::RBP,
                6 => Register::RSI,
                7 => Register::RDI,
                _ => panic!("Unknown instruction argument"),
            }
        }
        RegisterSize::Segment => {
            match num {
                0 => Register::ES,
                1 => Register::CS,
                2 => Register::SS,
                3 => Register::DS,
                4 => Register::FS,
                5 => Register::GS,
                _ => panic!("Unknown instruction argument"),
            }
        }
    }
}
