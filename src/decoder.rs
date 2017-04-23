use instruction_set::{Register, RegisterSize, InstructionArguments, InstructionArgumentsBuilder,
                      InstructionArgument, ArgumentSize};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;

use zero;

pub struct Decoder<'a> {
    machine_state: &'a mut MachineState,
    cpu: &'a CPU,
}

impl<'a> Decoder<'a> {
    pub fn new(cpu: &'a CPU, machine_state: &'a mut MachineState) -> Decoder<'a> {
        Decoder {
            cpu: cpu,
            machine_state: machine_state,
        }
    }

    pub fn execute(&mut self, debug: bool) {
        loop {
            let rip = self.machine_state.rip as u64;
            let mut first_byte = self.machine_state.mem_read_byte(rip);

            let mut decoder_flags = DecoderFlags { bits: 0 };

            match first_byte {
                0xF0 | 0xF2 => panic!("Lock prefixes/Bound prefix not supported"),
                0xF3 => {
                    decoder_flags |= REPEAT;
                    self.machine_state.rip += 1;
                }
                0x2E | 0x3E | 0x36 | 0x26 | 0x64 | 0x65 => {
                    panic!("Segment override prefixes/branch hints not supported")
                }
                0x66 => {
                    decoder_flags |= OPERAND_16_BIT;
                    self.machine_state.rip += 1;
                }
                0x67 => {
                    decoder_flags |= ADDRESS_SIZE_OVERRIDE;
                    self.machine_state.rip += 1;
                }
                _ => (),
            }

            let rip = self.machine_state.rip as u64;
            first_byte = self.machine_state.mem_read_byte(rip);
            match first_byte {
                0x40...0x4F => {
                    // 64bit REX prefix
                    let temp_rex = REX { bits: first_byte };
                    if temp_rex.contains(B) {
                        decoder_flags |= NEW_64BIT_REGISTER;
                    }
                    if temp_rex.contains(R) {
                        decoder_flags |= MOD_R_M_EXTENSION;
                    }
                    if temp_rex.contains(X) {
                        decoder_flags |= SIB_EXTENSION;
                    }
                    if temp_rex.contains(W) {
                        decoder_flags |= OPERAND_64_BIT;
                    }
                    decoder_flags |= NEW_8BIT_REGISTER;
                    self.machine_state.rip += 1;
                }
                _ => (),
            }

            let register_size = if decoder_flags.contains(OPERAND_64_BIT) {
                RegisterSize::Bit64
            } else {
                if decoder_flags.contains(OPERAND_16_BIT) {
                    RegisterSize::Bit16
                } else {
                    RegisterSize::Bit32
                }
            };

            let rip = self.machine_state.rip as u64;
            first_byte = self.machine_state.mem_read_byte(rip);
            match first_byte {
                opcode @ 0x50...0x57 => {
                    self.inc_rip(1);
                    self.cpu.push(self.machine_state,
                                InstructionArgumentsBuilder::new(InstructionArgument::Register {
                                    register: get_register(opcode - 0x50, RegisterSize::Bit64,
                                            decoder_flags.contains(NEW_64BIT_REGISTER),
                                            decoder_flags.contains(NEW_8BIT_REGISTER)),
                                }).finalize());
                }
                opcode @ 0x58...0x5F => {
                    let argument =
                        InstructionArgumentsBuilder::new(InstructionArgument::Register {
                                register:
                                    get_register(opcode - 0x58,
                                                    RegisterSize::Bit64,
                                                    decoder_flags.contains(NEW_64BIT_REGISTER),
                                                    decoder_flags.contains(NEW_8BIT_REGISTER)),
                            })
                            .finalize();
                    self.inc_rip(1);
                    self.cpu.pop(self.machine_state, argument);
                }
                opcode @ 0xB0...0xB7 => {
                    let immediate = self.machine_state.mem_read_byte(rip + 1) as i64;
                    let argument =
                        InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                immediate: immediate as i64,
                            })
                            .second_argument(InstructionArgument::Register {
                                register:
                                    get_register(opcode - 0xB0,
                                                    RegisterSize::Bit8,
                                                    decoder_flags.contains(NEW_64BIT_REGISTER),
                                                    decoder_flags.contains(NEW_8BIT_REGISTER)),
                            })
                            .finalize();
                    self.inc_rip(2);
                    self.cpu.mov(self.machine_state, argument);
                }
                opcode @ 0xB8...0xBF => {
                    let (immediate, ip_offset) = if decoder_flags.contains(OPERAND_64_BIT) {
                        (self.get_i64_value(1) as i64, 9)
                    } else {
                        if decoder_flags.contains(OPERAND_16_BIT) {
                            (self.get_i16_value(1) as i64, 3)
                        } else {
                            (self.get_i32_value(1) as i64, 5)
                        }
                    };
                    let argument =
                        InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                immediate: immediate,
                            })
                            .second_argument(InstructionArgument::Register {
                                register:
                                    get_register(opcode - 0xB8,
                                                    register_size,
                                                    decoder_flags.contains(NEW_64BIT_REGISTER),
                                                    decoder_flags.contains(NEW_8BIT_REGISTER)),
                            })
                            .finalize();
                    self.inc_rip(ip_offset);
                    self.cpu.mov(self.machine_state, argument);
                }
                0x00 => {
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.add(self.machine_state, argument);
                }
                0x01 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.add(self.machine_state, argument);
                }
                0x08 => {
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::Bit8,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.or(self.machine_state, argument);
                }
                0x09 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.or(self.machine_state, argument);
                }
                0x19 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.sbb(self.machine_state, argument);
                }
                0x21 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.and(self.machine_state, argument);
                }
                0x24 => {
                    let rip = self.machine_state.rip as u64;
                    let immediate = self.machine_state.mem_read_byte(rip + 1) as i64;
                    self.inc_rip(1);
                    self.cpu.and(self.machine_state, InstructionArgumentsBuilder::new(
                                InstructionArgument::Immediate {
                                    immediate: immediate,
                                }).second_argument(InstructionArgument::Register {
                                    register: Register::AL,
                                }).finalize());
                }
                0x28 => {
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.sub(self.machine_state, argument);
                }
                0x29 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.sub(self.machine_state, argument);
                }
                0x70 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jo(self.machine_state, arg);
                }
                0x71 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jno(self.machine_state, arg);
                }
                0x72 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jc(self.machine_state, arg);
                }
                0x73 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jnc(self.machine_state, arg);
                }
                0x74 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jz(self.machine_state, arg);
                }
                0x75 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jnz(self.machine_state, arg);
                }
                0x76 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jbe(self.machine_state, arg);
                }
                0x77 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.ja(self.machine_state, arg);
                }
                0x78 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.js(self.machine_state, arg);
                }
                0x79 => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jns(self.machine_state, arg);
                }
                0x7A => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jp(self.machine_state, arg);
                }
                0x7B => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jnp(self.machine_state, arg);
                }
                0x7C => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jl(self.machine_state, arg);
                }
                0x7D => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jge(self.machine_state, arg);
                }
                0x7E => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jle(self.machine_state, arg);
                }
                0x7F => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jg(self.machine_state, arg);
                }
                0x63 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                    self.inc_rip(ip_offset);
                    self.cpu.movsx(self.machine_state, argument);
                }
                0x6A => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.push(self.machine_state, arg);
                }
                0xEB => {
                    let (arg, ip_offset) = self.read_immediate_8bit();
                    self.inc_rip(ip_offset);
                    self.cpu.jmp(self.machine_state, arg);
                }
                0xE8 => {
                    let immediate = self.get_i32_value(1);
                    self.inc_rip(5);
                    self.cpu.call(self.machine_state,
                                InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                    immediate: immediate as i64,
                                }).finalize());
                }
                0x88 => {
                    // mov
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.mov(self.machine_state, argument);
                }
                0x89 => {
                    // mov
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.mov(self.machine_state, argument);
                }
                0x84 => {
                    // test
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.test(self.machine_state, argument);
                }
                0x85 => {
                    // test
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.test(self.machine_state, argument);
                }
                0x31 => {
                    // xor
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.xor(self.machine_state, argument);
                }
                0x38 => {
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.cmp(self.machine_state, argument);
                }
                0x39 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.cmp(self.machine_state, argument);
                }
                0x3B => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                    self.inc_rip(ip_offset);
                    self.cpu.cmp(self.machine_state, argument);
                }
                0x80 => {
                    // arithmetic operation (8bit register target, 8bit immediate)
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::Bit8,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.arithmetic(self.machine_state, argument);
                }
                0x81 => {
                    // arithmetic operation (32/64bit register target, 32bit immediate)
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::Bit32,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.arithmetic(self.machine_state, argument);
                }
                0x83 => {
                    // arithmetic operation (32/64bit register target, 8bit immediate)
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::Bit8,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.arithmetic(self.machine_state, argument);
                }
                0xC6 => {
                    let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::Bit8,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.mov(self.machine_state, argument);
                }
                0xC7 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::Bit32,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.mov(self.machine_state, argument);
                }
                0x8B => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags |
                                                                    REVERSED_REGISTER_DIRECTION);
                    self.inc_rip(ip_offset);
                    self.cpu.mov(self.machine_state, argument);
                }
                0x8E => {
                    // mov 16bit segment registers
                    let (argument, ip_offset) =
                        self.get_argument(RegisterSize::Segment,
                                            RegOrOpcode::Register,
                                            ImmediateSize::None,
                                            // TODO: REVERSED_REGISTER_DIRECTION correct?
                                            decoder_flags | REVERSED_REGISTER_DIRECTION);
                    self.inc_rip(ip_offset);
                    self.cpu.mov(self.machine_state, argument);
                }
                0x8D => {
                    let (argument, ip_offset) =
                        self.get_argument(register_size,
                                            RegOrOpcode::Register,
                                            ImmediateSize::None,
                                            // TODO: REVERSED_REGISTER_DIRECTION correct?
                                            decoder_flags | REVERSED_REGISTER_DIRECTION);
                    self.machine_state.rip += ip_offset;
                    self.inc_rip(0);
                    self.cpu.lea(self.machine_state, argument);
                }
                0x98 => {
                    let (register1, register2) = if decoder_flags.contains(OPERAND_16_BIT) {
                        (Register::AL, Register::AX)
                    } else if decoder_flags.contains(OPERAND_64_BIT) {
                        (Register::EAX, Register::RAX)
                    } else {
                        (Register::AX, Register::EAX)
                    };

                    let argument = InstructionArgumentsBuilder::new(
                        InstructionArgument::Register{register: register1}
                    ).second_argument(InstructionArgument::Register{register: register2})
                    .finalize();
                    self.inc_rip(1);
                    self.cpu.mov(self.machine_state, argument);
                }
                0x99 => {
                    let (register1, register2) = if decoder_flags.contains(OPERAND_16_BIT) {
                        (Register::AX, Register::DX)
                    } else if decoder_flags.contains(OPERAND_64_BIT) {
                        (Register::RAX, Register::RDX)
                    } else {
                        (Register::EAX, Register::EDX)
                    };

                    let argument = InstructionArgumentsBuilder::new(
                        InstructionArgument::Register{register: register1}
                    ).second_argument(InstructionArgument::Register{register: register2})
                    .finalize();
                    self.inc_rip(1);
                    self.cpu.mov(self.machine_state, argument);
                }
                0xA5 => {
                    let repeat = decoder_flags.contains(REPEAT);
                    self.inc_rip(1);
                    self.cpu.movs(self.machine_state, repeat);
                }
                0xAB => {
                    let repeat = decoder_flags.contains(REPEAT);
                    self.inc_rip(1);
                    self.cpu.stos(self.machine_state, repeat);
                }
                0xC1 => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::Bit8,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.sar(self.machine_state, argument);
                }
                0xC3 => {
                    self.inc_rip(0);
                    self.cpu.ret(self.machine_state);
                }
                0xC9 => {
                    self.inc_rip(1);
                    self.cpu.leave(self.machine_state);
                }
                0xD1 => {
                    let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    argument.second_argument = Some(argument.first_argument);
                    argument.first_argument = InstructionArgument::Immediate{
                        immediate: 1,
                    };
                    self.inc_rip(ip_offset);
                    self.cpu.shift_rotate(self.machine_state, argument);
                }
                0xD3 => {
                    let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    argument.second_argument = Some(argument.first_argument);
                    argument.first_argument = InstructionArgument::Register{
                        register: Register::CL
                    };
                    self.inc_rip(ip_offset);
                    self.cpu.shift_rotate(self.machine_state, argument);
                }
                0xFD => {
                    self.inc_rip(1);
                    self.cpu.std(self.machine_state);
                }
                0x9D => {
                    self.inc_rip(1);
                    self.cpu.popf(self.machine_state);
                }
                0xE9 => {
                    let immediate = self.get_i32_value(1);
                    self.inc_rip(5);
                    self.cpu.jmp(self.machine_state,
                                InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                    immediate: immediate as i64,
                                }).finalize());
                }
                0xEE => {
                    println!("WARNING: OUT not implemented");
                    self.inc_rip(1);
                }
                0xF6 => {
                    let rip = self.machine_state.rip as u64;
                    let modrm = self.machine_state.mem_read_byte(rip + 1);
                    let opcode = (modrm & 0b00111000) >> 3;

                    let (argument, ip_offset) = match opcode {
                        0 | 1 => {
                            self.get_argument(RegisterSize::Bit8,
                                                RegOrOpcode::Opcode,
                                                ImmediateSize::Bit8,
                                                decoder_flags)
                        },
                        _ => panic!("no supported"),
                    };
                    self.inc_rip(ip_offset);
                    self.cpu.compare_mul_operation(self.machine_state, argument);
                }
                0xF7 => {
                    let rip = self.machine_state.rip as u64;
                    let modrm = self.machine_state.mem_read_byte(rip + 1);
                    let opcode = (modrm & 0b00111000) >> 3;

                    let (argument, ip_offset) = match opcode {
                        0 | 1 => {
                            // TODO: could also be 16 bit immediate
                            self.get_argument(register_size,
                                                RegOrOpcode::Opcode,
                                                ImmediateSize::Bit32,
                                                decoder_flags)
                        },
                        2 | 3 => {
                            self.get_argument(register_size,
                                                RegOrOpcode::Opcode,
                                                ImmediateSize::None,
                                                decoder_flags)
                        },
                        4 | 5 | 6 | 7 => {
                            panic!("invalid decoder logic, this opcodes take 3 arguments, we only decode one");
                            /*
                            let register = get_register(
                                0, register_size,decoder_flags.contains(NEW_64BIT_REGISTER), false);

                            (InstructionArgumentsBuilder::new(
                                InstructionArgument::Register{register: register})
                                .opcode(opcode)
                                .finalize(),
                            2)*/
                        },
                        _ => unreachable!()
                    };
                    self.inc_rip(ip_offset);
                    self.cpu.compare_mul_operation(self.machine_state, argument);
                }
                0xFC => {
                    self.inc_rip(1);
                    self.cpu.cld(self.machine_state);
                }
                0xFF => {
                    let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Opcode,
                                                                    ImmediateSize::None,
                                                                    decoder_flags);
                    self.inc_rip(ip_offset);
                    self.cpu.register_operation(self.machine_state, argument);
                }
                0x0F => {
                    // two byte instructions
                    self.machine_state.rip += 1;
                    let rip = self.machine_state.rip as u64;
                    let second_byte = self.machine_state.mem_read_byte(rip);
                    match second_byte {
                        0x1F => {
                            // NOP with hint
                            self.inc_rip(3);
                        }
                        0x44 => {
                            let (argument, ip_offset) = self.get_argument(register_size,
                                                RegOrOpcode::Register,
                                                ImmediateSize::None,
                                                decoder_flags | REVERSED_REGISTER_DIRECTION);
                            self.inc_rip(ip_offset);
                            self.cpu.cmovz(self.machine_state, argument);
                        },
                        0x48 => {
                            let (argument, ip_offset) = self.get_argument(register_size,
                                                RegOrOpcode::Register,
                                                ImmediateSize::None,
                                                decoder_flags | REVERSED_REGISTER_DIRECTION);
                            self.inc_rip(ip_offset);
                            self.cpu.cmovs(self.machine_state, argument);
                        },
                        0x80 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jo(self.machine_state, argument);
                        },
                        0x81 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jno(self.machine_state, argument);
                        },
                        0x82 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jc(self.machine_state, argument);
                        },
                        0x83 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jnc(self.machine_state, argument);
                        },
                        0x84 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jz(self.machine_state, argument);
                        },
                        0x85 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jnz(self.machine_state, argument);
                        },
                        0x86 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jbe(self.machine_state, argument);
                        },
                        0x87 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.ja(self.machine_state, argument);
                        },
                        0x88 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.js(self.machine_state, argument);
                        },
                        0x89 => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jns(self.machine_state, argument);
                        },
                        0x8A => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jp(self.machine_state, argument);
                        },
                        0x8B => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jnp(self.machine_state, argument);
                        },
                        0x8C => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jl(self.machine_state, argument);
                        },
                        0x8D => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jge(self.machine_state, argument);
                        },
                        0x8E => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jle(self.machine_state, argument);
                        },
                        0x8F => {
                            // TODO: could also be 16bit value
                            let argument = self.read_immediate_32bit();
                            self.inc_rip(5);
                            self.cpu.jg(self.machine_state, argument);
                        },
                        0x94 => {
                            let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                            RegOrOpcode::Register,
                                                                            ImmediateSize::None,
                                                                            decoder_flags);
                            self.inc_rip(ip_offset);
                            self.cpu.setz(self.machine_state, argument);
                        },
                        0xB6 => {
                            let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                                RegOrOpcode::Register,
                                                                                ImmediateSize::None,
                                                                                decoder_flags | REVERSED_REGISTER_DIRECTION);

                            match argument.first_argument {
                                InstructionArgument::Register {..}=> {
                                    let modrm = self.machine_state.mem_read_byte(rip + 1);
                                    let register = modrm & 0b00000111;
                                    let register = get_register(register, RegisterSize::Bit8,
                                                                decoder_flags.contains(NEW_64BIT_REGISTER), false);
                                    argument.first_argument = InstructionArgument::Register{
                                        register: register,
                                    };
                                },
                                InstructionArgument::EffectiveAddress {..} => {
                                    argument.explicit_size = Some(ArgumentSize::Bit8)
                                },
                                _ => panic!("Invalid argument for movzx")
                            }

                            self.inc_rip(ip_offset);
                            self.cpu.movzx(self.machine_state, argument);
                        }
                        0xB7 => {
                            let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                                RegOrOpcode::Register,
                                                                                ImmediateSize::None,
                                                                                decoder_flags | REVERSED_REGISTER_DIRECTION);
                            match argument.first_argument {
                                InstructionArgument::Register {..}=> {
                                    let modrm = self.machine_state.mem_read_byte(rip + 1);
                                    let register = modrm & 0b00000111;
                                    let register = get_register(register, RegisterSize::Bit16,
                                                                decoder_flags.contains(NEW_64BIT_REGISTER), false);
                                    argument.first_argument = InstructionArgument::Register{
                                        register: register,
                                    };
                                },
                                InstructionArgument::EffectiveAddress {..} => {
                                    argument.explicit_size = Some(ArgumentSize::Bit16)
                                },
                                _ => panic!("Invalid argument for movzx")
                            }
                            self.inc_rip(ip_offset);
                            self.cpu.movzx(self.machine_state, argument);
                        }
                        0xBE => {
                            let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                                RegOrOpcode::Register,
                                                                                ImmediateSize::None,
                                                                                decoder_flags | REVERSED_REGISTER_DIRECTION);
                            let modrm = self.machine_state.mem_read_byte(rip + 1);
                            let register = modrm & 0b00000111;
                            let register = get_register(register, RegisterSize::Bit8,
                                                        decoder_flags.contains(NEW_64BIT_REGISTER), false);
                            argument.first_argument = InstructionArgument::Register{
                                register: register,
                            };
                            self.inc_rip(ip_offset);
                            self.cpu.movsx(self.machine_state, argument);
                        }
                        0xBF => {
                            let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                                RegOrOpcode::Register,
                                                                                ImmediateSize::None,
                                                                                decoder_flags | REVERSED_REGISTER_DIRECTION);
                            let modrm = self.machine_state.mem_read_byte(rip + 1);
                            let register = modrm & 0b00000111;
                            let register = get_register(register, RegisterSize::Bit16,
                                                        decoder_flags.contains(NEW_64BIT_REGISTER), false);
                            argument.first_argument = InstructionArgument::Register{
                                register: register,
                            };

                            self.cpu.movsx(self.machine_state, argument);
                            self.inc_rip(ip_offset);
                        }
                        0xAF => {
                            let (argument, ip_offset) = self.get_argument(register_size,
                                                                        RegOrOpcode::Register,
                                                                        ImmediateSize::None,
                                                                        decoder_flags);
                            self.cpu.imul(self.machine_state, argument);
                            self.inc_rip(ip_offset);
                        }
                        _ => panic!("Unknown instruction: 0F {:X}", second_byte),
                    }

                }
                0xCC => {
                    // abuse int 3 instruction to signal failed test program
                    panic!("int3 instruction");
                }
                0xCD => {
                    // abuse int X instruction to signal passed test program
                    println!("int    $0x80");
                    return;
                }
                _ => panic!("Unknown instruction: {:x}", first_byte),
            };

            if debug {
                println!("{}", self.machine_state);
            }
        }
    }

    fn inc_rip(&mut self, ip_offset: i64) {
        self.machine_state.rip += ip_offset;
    }

    fn get_i64_value(&mut self, ip_offset: i64) -> i64 {
        let rip = (self.machine_state.rip + ip_offset) as u64;
        let value = self.machine_state.mem_read(rip, 8);
        *zero::read::<i64>(&value)
    }

    fn get_i32_value(&mut self, ip_offset: i64) -> i32 {
        let rip = (self.machine_state.rip + ip_offset) as u64;
        let value = self.machine_state.mem_read(rip, 4);
        *zero::read::<i32>(&value)
    }

    fn get_i16_value(&mut self, ip_offset: i64) -> i16 {
        let rip = (self.machine_state.rip + ip_offset) as u64;
        let value = self.machine_state.mem_read(rip, 2);
        *zero::read::<i16>(&value)
    }


    fn read_immediate_8bit(&mut self) -> (InstructionArguments, i64) {
        let rip = self.machine_state.rip as u64;
        let immediate = self.machine_state.mem_read_byte(rip + 1) as i8 as i64;

        (InstructionArgumentsBuilder::new(InstructionArgument::Immediate { immediate: immediate })
             .finalize(),
         2)
    }

    fn read_immediate_32bit(&mut self) -> InstructionArguments {
        let immediate = self.get_i32_value(1) as i64;
        InstructionArgumentsBuilder::new(InstructionArgument::Immediate { immediate: immediate })
             .finalize()
    }

    fn get_argument(&mut self,
                    register_size: RegisterSize,
                    reg_or_opcode: RegOrOpcode,
                    immediate_size: ImmediateSize,
                    decoder_flags: DecoderFlags)
                    -> (InstructionArguments, i64) {
        let rip = (self.machine_state.rip + 1) as u64;
        let modrm = self.machine_state.mem_read_byte(rip);

        let mut address_mod = modrm >> 6;

        match address_mod {
            0b00 | 0b01 | 0b10 => {
                // effective address / effecive address + 8 bit deplacement /
                // effecive address + 32 bit deplacement
                let rm = modrm & 0b00000111;

                // special case: RIP relative adressing. We fake a 32bit displacement instruction.
                if address_mod == 0b00 && rm == 0x5 {
                    address_mod = 0b100;
                }

                // sib byte
                let (sib, offset) = if rm == 0b100 {
                    (Some(self.machine_state.mem_read_byte(rip + 1)), 3)
                } else {
                    (None, 2)
                };

                let (displacement, mut ip_offset) = match address_mod {
                    0b00 => (0, 0),
                    0b01 => {
                        let rip = (self.machine_state.rip + offset) as u64;
                        (self.machine_state.mem_read_byte(rip) as i8 as i32, 1)
                    }
                    0b10 | 0b100 => {
                        let displacement = self.get_i32_value(offset);
                        // change RIP relative addressing mode back to 0b00
                        if address_mod == 0b100 {
                            address_mod = 0b00;
                        }

                        (displacement, 4)
                    }
                    _ => unreachable!(),
                };
                ip_offset += offset; // skip instruction + modrm byte

                let register_or_opcode = (modrm & 0b00111000) >> 3;
                // TODO: based on REX, this could be a 64bit value
                match immediate_size {
                    ImmediateSize::Bit8 => {
                        assert!(reg_or_opcode == RegOrOpcode::Opcode);
                        let rip = (self.machine_state.rip + ip_offset) as u64;
                        let immediate = self.machine_state.mem_read_byte(rip);

                        let argument_size = match register_size {
                            RegisterSize::Bit8 => ArgumentSize::Bit8,
                            RegisterSize::Bit16 => ArgumentSize::Bit16,
                            RegisterSize::Bit32 => ArgumentSize::Bit32,
                            RegisterSize::Bit64 => ArgumentSize::Bit64,
                            RegisterSize::Segment => panic!("Unsupported register size"),
                        };
                        let register = if address_mod == 0b00 && rm == 0x5 {
                            Register::RIP
                        } else {
                            let register_size = if decoder_flags.contains(ADDRESS_SIZE_OVERRIDE) {
                                RegisterSize::Bit32
                            } else {
                                RegisterSize::Bit64
                            };
                            get_register(rm, register_size, decoder_flags.contains(NEW_64BIT_REGISTER),
                                         decoder_flags.contains(NEW_8BIT_REGISTER))
                        };

                        (InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                 immediate: immediate as i64,
                             })
                             .second_argument(InstructionArgument::EffectiveAddress {
                                 base: register,
                                 displacement: displacement,
                                 index: None,
                                 scale: None,
                             })
                             .opcode(register_or_opcode)
                             .explicit_size(argument_size)
                             .finalize(),
                         ip_offset + 1)
                    }
                    ImmediateSize::Bit32 => {
                        assert!(reg_or_opcode == RegOrOpcode::Opcode);
                        let immediate = self.get_i32_value(ip_offset);

                        let argument_size = match register_size {
                            RegisterSize::Bit8 => ArgumentSize::Bit8,
                            RegisterSize::Bit16 => ArgumentSize::Bit16,
                            RegisterSize::Bit32 => ArgumentSize::Bit32,
                            RegisterSize::Bit64 => ArgumentSize::Bit64,
                            RegisterSize::Segment => panic!("Unsupported register size"),
                        };

                        let register = if address_mod == 0b00 && rm == 0x5 {
                            Register::RIP
                        } else {
                            let register_size = if decoder_flags.contains(ADDRESS_SIZE_OVERRIDE) {
                                RegisterSize::Bit32
                            } else {
                                RegisterSize::Bit64
                            };
                            get_register(rm, register_size, decoder_flags.contains(NEW_64BIT_REGISTER),
                                         decoder_flags.contains(NEW_8BIT_REGISTER))
                        };

                        (InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                 immediate: immediate as i64,
                             })
                             .second_argument(InstructionArgument::EffectiveAddress {
                                 base: register,
                                 displacement: displacement,
                                 index: None,
                                 scale: None,
                             })
                             .opcode(register_or_opcode)
                             .explicit_size(argument_size)
                             .finalize(),
                         ip_offset + 4)
                    }
                    ImmediateSize::None => {
                        assert!(reg_or_opcode == RegOrOpcode::Register);

                        let second_reg_size = if decoder_flags.contains(ADDRESS_SIZE_OVERRIDE) {
                            RegisterSize::Bit32
                        } else {
                            RegisterSize::Bit64
                        };

                        // special case: RIP relative adressing.
                        let register1 = if address_mod == 0b00 && rm == 0x5 {
                            Register::RIP
                        } else {
                            get_register(rm,
                                         second_reg_size,
                                         decoder_flags.contains(NEW_64BIT_REGISTER),
                                         decoder_flags.contains(NEW_8BIT_REGISTER))
                        };
                        let register2 = get_register(register_or_opcode,
                                                     register_size,
                                                     decoder_flags.contains(MOD_R_M_EXTENSION),
                                                     decoder_flags.contains(NEW_8BIT_REGISTER));

                        (if decoder_flags.contains(REVERSED_REGISTER_DIRECTION) {
                             InstructionArgumentsBuilder::new(self.effective_address(sib, register1, displacement, decoder_flags))
                             .second_argument(
                                InstructionArgument::Register {
                                    register: register2,
                                }).finalize()
                         } else {
                             InstructionArgumentsBuilder::new(InstructionArgument::Register {
                                     register: register2,
                                 })
                                 .second_argument(self.effective_address(sib, register1, displacement, decoder_flags))
                                 .finalize()
                         },
                         ip_offset)
                    }
                }
            }
            0b11 => {
                // register
                let register1 = get_register(modrm & 0b00000111,
                                             register_size,
                                             decoder_flags.contains(NEW_64BIT_REGISTER),
                                             decoder_flags.contains(NEW_8BIT_REGISTER));
                let value2 = (modrm & 0b00111000) >> 3;
                match reg_or_opcode {
                    RegOrOpcode::Register => {
                        (if decoder_flags.contains(REVERSED_REGISTER_DIRECTION) {
                             InstructionArgumentsBuilder::new(InstructionArgument::Register {
                                     register: register1,
                                 })
                                 .second_argument(InstructionArgument::Register {
                                     register:
                                         get_register(value2,
                                                      register_size,
                                                      decoder_flags.contains(MOD_R_M_EXTENSION), decoder_flags.contains(NEW_8BIT_REGISTER)),
                                 })
                                 .finalize()
                         } else {
                             InstructionArgumentsBuilder::new(InstructionArgument::Register {
                                     register:
                                         get_register(value2,
                                                      register_size,
                                                      decoder_flags.contains(MOD_R_M_EXTENSION), decoder_flags.contains(NEW_8BIT_REGISTER)),
                                 })
                                 .second_argument(InstructionArgument::Register {
                                     register: register1,
                                 })
                                 .finalize()
                         },
                         2)
                    }
                    RegOrOpcode::Opcode => {
                        match immediate_size {
                            ImmediateSize::Bit8 => {
                                let rip = (self.machine_state.rip + 2) as u64;
                                let immediate = self.machine_state.mem_read_byte(rip);
                                (InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                         immediate: immediate as i8 as i64,
                                     })
                                     .second_argument(InstructionArgument::Register {
                                         register: register1,
                                     })
                                     .opcode(value2)
                                     .finalize(),
                                 3)
                            }
                            ImmediateSize::Bit32 => {
                                let immediate = self.get_i32_value(2);
                                (InstructionArgumentsBuilder::new(InstructionArgument::Immediate {
                                         immediate: immediate as i64,
                                     })
                                     .second_argument(InstructionArgument::Register {
                                         register: register1,
                                     })
                                     .opcode(value2)
                                     .finalize(),
                                 6)
                            }
                            ImmediateSize::None => {
                                (InstructionArgumentsBuilder::new(InstructionArgument::Register {
                                         register: register1,
                                     })
                                     .opcode(value2)
                                     .finalize(),
                                 2)
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }


    fn effective_address(&self, sib: Option<u8>, register: Register, displacement: i32, decoder_flags: DecoderFlags) -> InstructionArgument {
        match sib {
            None => {
                InstructionArgument::EffectiveAddress {
                    base: register,
                    index: None,
                    scale: None,
                    displacement: displacement,
                }
            }
            Some(sib) => {
                let base = sib & 0b00000111;
                let index = (sib & 0b00111000) >> 3;
                let scale = (sib & 0b11000000) >> 6;
                let scale = 2u8.pow(scale as u32) as u8;

                let register_size = if decoder_flags.contains(ADDRESS_SIZE_OVERRIDE) {
                    RegisterSize::Bit32
                } else {
                    RegisterSize::Bit64
                };

                let base = get_register(base, register_size,
                                       decoder_flags.contains(NEW_64BIT_REGISTER), false);
                match base {
                    Register::RSP => {
                        InstructionArgument::EffectiveAddress {
                            base: Register::RSP,
                            index: None,
                            scale: None,
                            displacement: displacement,
                        }
                    },
                    Register::RBP | Register::R13 => panic!("SIB special case RBP/R13 not implemented"),
                    _ => {
                        InstructionArgument::EffectiveAddress {
                            base: base,
                            displacement: displacement,
                            scale: Some(scale),
                            index: Some(get_register(index, register_size,
                                                decoder_flags.contains(SIB_EXTENSION), false)),
                        }
                    }
                }
            }
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
        const B = 0b00000001,
        const X = 0b00000010,
        const R = 0b00000100,
        const W = 0b00001000,
    }
}

bitflags! {
    flags DecoderFlags: u64 {
        const REVERSED_REGISTER_DIRECTION = 1 << 0,
        const ADDRESS_SIZE_OVERRIDE = 1 << 2,
        const REPEAT = 1 << 3,
        const NEW_64BIT_REGISTER = 1 << 4,
        const NEW_8BIT_REGISTER = 1 << 5,
        const MOD_R_M_EXTENSION = 1 << 6,
        const SIB_EXTENSION = 1 << 7,
        const OPERAND_16_BIT = 1 << 8,
        const OPERAND_64_BIT = 1 << 9,
    }
}

fn get_register(num: u8, size: RegisterSize, new_64bit_register: bool, new_8bit_register: bool) -> Register {
    match size {
        RegisterSize::Bit64 => {
            if new_64bit_register {
                match num {
                    0 => Register::R8,
                    1 => Register::R9,
                    2 => Register::R10,
                    3 => Register::R11,
                    4 => Register::R12,
                    5 => Register::R13,
                    6 => Register::R14,
                    7 => Register::R15,
                    _ => panic!("Unknown instruction argument"),
                }
            } else {
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
        }
        RegisterSize::Bit32 => {
            if new_64bit_register {
                match num {
                    0 => Register::R8D,
                    1 => Register::R9D,
                    2 => Register::R10D,
                    3 => Register::R11D,
                    4 => Register::R12D,
                    5 => Register::R13D,
                    6 => Register::R14D,
                    7 => Register::R15D,
                    _ => panic!("Unknown instruction argument"),
                }
            } else {
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
        }
        RegisterSize::Bit16 => {
            if new_64bit_register {
                match num {
                    0 => Register::R8W,
                    1 => Register::R9W,
                    2 => Register::R10W,
                    3 => Register::R11W,
                    4 => Register::R12W,
                    5 => Register::R13W,
                    6 => Register::R14W,
                    7 => Register::R15W,
                    _ => panic!("Unknown instruction argument"),
                }
            } else {
                match num {
                    0 => Register::AX,
                    1 => Register::CX,
                    2 => Register::DX,
                    3 => Register::BX,
                    4 => Register::SP,
                    5 => Register::BP,
                    6 => Register::SI,
                    7 => Register::DI,
                    _ => panic!("Unknown instruction argument"),
                }
            }
        }
        RegisterSize::Bit8 => {
            if new_64bit_register {
                match num {
                    0 => Register::R8B,
                    1 => Register::R9B,
                    2 => Register::R10B,
                    3 => Register::R11B,
                    4 => Register::R12B,
                    5 => Register::R13B,
                    6 => Register::R14B,
                    7 => Register::R15B,
                    _ => panic!("Unknown instruction argument"),
                }
            } else {
                if new_8bit_register {
                    match num {
                        0 => Register::AL,
                        1 => Register::CL,
                        2 => Register::DL,
                        3 => Register::BL,
                        4 => Register::SPL,
                        5 => Register::BPL,
                        6 => Register::SIL,
                        7 => Register::DIL,
                        _ => panic!("Unknown instruction argument"),
                    }
                } else {
                    match num {
                        0 => Register::AL,
                        1 => Register::CL,
                        2 => Register::DL,
                        3 => Register::BL,
                        4 => Register::AH,
                        5 => Register::CH,
                        6 => Register::DH,
                        7 => Register::BH,
                        _ => panic!("Unknown instruction argument"),
                    }
                }
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
