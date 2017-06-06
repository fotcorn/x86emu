use std::io::Write;
use fnv::FnvHashMap;
use std::collections::hash_map::{Entry};
use time::PreciseTime;

use instruction_set::{Register, RegisterSize, InstructionArguments, InstructionArgumentsBuilder,
                      InstructionArgument, ArgumentSize, Instruction, InstructionCache};
use machine_state::MachineState;
use cpu::emu_instructions::EmulationCPU;

use zero;

pub struct Decoder<'a> {
    machine_state: &'a mut MachineState,
    cpu: &'a EmulationCPU,
    counter: u64,
}

impl<'a> Decoder<'a> {
    pub fn new(cpu: &'a EmulationCPU, machine_state: &'a mut MachineState) -> Decoder<'a> {
        Decoder {
            cpu: cpu,
            machine_state: machine_state,
            counter: 0,
        }
    }

    pub fn execute(&mut self, benchmark: bool) {
        let mut instruction_cache = FnvHashMap::default();

        let start = PreciseTime::now();
        loop {
            self.counter += 1;
            let instruction_start = self.machine_state.rip as u64;
            
            let cache_entry = match instruction_cache.entry(instruction_start) {
                Entry::Occupied(entry) => {
                    let ref entry: InstructionCache = *entry.into_mut();
                    self.machine_state.rip += entry.size as i64;
                    entry
                },
                Entry::Vacant(entry) => {
                    let cache_entry = self.decode();

                    let instruction_end = self.machine_state.rip as u64;

                    let cache_entry = InstructionCache {
                        instruction: cache_entry.0,
                        arguments: cache_entry.1,
                        size: instruction_end - instruction_start,
                    };
                    entry.insert(cache_entry)
                }
            };

            self.execute_instruction(cache_entry);
            match cache_entry.instruction {
                Instruction::Int => {
                    break;
                },
                _ => (),
            }

            if self.machine_state.print_registers {
                println!("{}", self.machine_state);
            }
        }
        if benchmark {
            let r = writeln!(&mut ::std::io::stderr(), "duration: {}", start.to(PreciseTime::now()));
            r.expect("failed printing to stderr");
        }
    }

    pub fn decode(&mut self) -> (Instruction, Option<InstructionArguments>) {
        let mut first_byte;

        let mut decoder_flags = DecoderFlags { bits: 0 };

        loop {
            let rip = self.machine_state.rip as u64;
            first_byte = self.machine_state.mem_read_byte(rip);
            match first_byte {
                0xF0 | 0xF2 => {
                    // todo: do not ignore lock/bound prefix
                }
                0xF3 => {
                    decoder_flags |= REPEAT;
                }
                0x2E | 0x3E | 0x36 | 0x26 | 0x64 | 0x65 => {
                    //TODO: do not ignore segment prefix (or probably we should?)
                }
                0x66 => {
                    decoder_flags |= OPERAND_16_BIT;
                }
                0x67 => {
                    decoder_flags |= ADDRESS_SIZE_OVERRIDE;
                }
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
                }
                _ => break,
            }
            self.machine_state.rip += 1;
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
            0x00 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::Add, Some(argument))
            }
            0x01 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::Add, Some(argument))
            }
            0x02 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Add, Some(argument))
            }
            0x03 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Add, Some(argument))
            }
            0x04 => {
                let argument = self.decode_al_immediate();
                (Instruction::Add, Some(argument))
            }
            0x05 => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Add, Some(argument))
            }
            0x08 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::Or, Some(argument))
            }
            0x09 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::Or, Some(argument))
            }
            0x0A => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Or, Some(argument))
            }
            0x0B => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Or, Some(argument))
            }
            0x0C => {
                let argument = self.decode_al_immediate();
                (Instruction::Or, Some(argument))
            }
            0x0D => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Or, Some(argument))
            }
            0x10 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::Adc, Some(argument))
            }
            0x11 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::Adc, Some(argument))
            }
            0x12 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Adc, Some(argument))
            }
            0x13 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Adc, Some(argument))
            }
            0x14 => {
                let argument = self.decode_al_immediate();
                (Instruction::Adc, Some(argument))
            }
            0x15 => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Adc, Some(argument))
            }
            0x18 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::Sbb, Some(argument))
            }
            0x19 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::Sbb, Some(argument))
            }
            0x1A => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Sbb, Some(argument))
            }
            0x1B => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Sbb, Some(argument))
            }
            0x1C => {
                let argument = self.decode_al_immediate();
                (Instruction::Sbb, Some(argument))
            }
            0x1D => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Sbb, Some(argument))
            }
            0x20 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::And, Some(argument))
            }
            0x21 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::And, Some(argument))
            }
            0x22 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::And, Some(argument))
            }
            0x23 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::And, Some(argument))
            }
            0x24 => {
                let argument = self.decode_al_immediate();
                (Instruction::And, Some(argument))
            }
            0x25 => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::And, Some(argument))
            }
            0x28 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::Sub, Some(argument))
            }
            0x29 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::Sub, Some(argument))
            }
            0x2A => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Sub, Some(argument))
            }
            0x2B => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Sub, Some(argument))
            }
            0x2C => {
                let argument = self.decode_al_immediate();
                (Instruction::Sub, Some(argument))
            }
            0x2D => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Sub, Some(argument))
            }
            0x30 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::Xor, Some(argument))
            }
            0x31 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::Xor, Some(argument))
            }
            0x32 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Xor, Some(argument))
            }
            0x33 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Xor, Some(argument))
            }
            0x34 => {
                let argument = self.decode_al_immediate();
                (Instruction::Xor, Some(argument))
            }
            0x35 => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Xor, Some(argument))
            }
            0x38 => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
                (Instruction::Cmp, Some(argument))
            }
            0x39 => {
                let argument = self.decode_reg_reg(register_size, decoder_flags);
                (Instruction::Cmp, Some(argument))
            }
            0x3A => {
                let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Cmp, Some(argument))
            }
            0x3B => {
                let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
                (Instruction::Cmp, Some(argument))
            }
            0x3C => {
                let argument = self.decode_al_immediate();
                (Instruction::Cmp, Some(argument))
            }
            0x3D => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Cmp, Some(argument))
            }
            opcode @ 0x50...0x57 => {
                self.inc_rip(1);
                (Instruction::Push, Some(
                            InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Register {
                                register: get_register(opcode - 0x50, RegisterSize::Bit64,
                                        decoder_flags.contains(NEW_64BIT_REGISTER),
                                        decoder_flags.contains(NEW_8BIT_REGISTER)),
                            }).finalize()))
            }
            opcode @ 0x58...0x5F => {
                let argument =
                    InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Register {
                            register:
                                get_register(opcode - 0x58,
                                                RegisterSize::Bit64,
                                                decoder_flags.contains(NEW_64BIT_REGISTER),
                                                decoder_flags.contains(NEW_8BIT_REGISTER)),
                        })
                        .finalize();
                self.inc_rip(1);
                (Instruction::Pop, Some(argument))
            }
            0x63 => {
                let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                self.override_argument_size(&mut argument, ArgumentSize::Bit32, rip, &decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Movsx, Some(argument))
            }
            0x68 => {
                let immediate = if decoder_flags.contains(OPERAND_16_BIT) {
                    let immediate = self.get_i16_value(1) as i64;
                    self.inc_rip(3);
                    immediate
                } else {
                    let immediate = self.get_i32_value(1) as i64;
                    self.inc_rip(5);
                    immediate
                };
                let argument = InstructionArgumentsBuilder::new().first_argument(
                    InstructionArgument::Immediate { immediate: immediate }
                ).finalize();
                (Instruction::Push, Some(argument))
            }
            0x69 => {
                let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags | REVERSED_REGISTER_DIRECTION);
                self.inc_rip(ip_offset);
                let immediate = if decoder_flags.contains(OPERAND_16_BIT) {
                    let immediate = self.get_i16_value(0) as i64;
                    self.inc_rip(2);
                    immediate
                } else {
                    let immediate = self.get_i32_value(0) as i64;
                    self.inc_rip(4);
                    immediate
                };
                argument.third_argument = argument.second_argument;
                argument.second_argument = argument.first_argument;
                argument.first_argument = Some(InstructionArgument::Immediate { immediate: immediate });
                (Instruction::Imul, Some(argument))
            }
            0x6A => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Push, Some(arg))
            }
            0x6B => {
                let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags | REVERSED_REGISTER_DIRECTION);
                self.inc_rip(ip_offset);
                let rip = self.machine_state.rip as u64;
                let immediate = self.machine_state.mem_read_byte(rip) as i8 as i64;
                argument.third_argument = argument.second_argument;
                argument.second_argument = argument.first_argument;
                argument.first_argument = Some(InstructionArgument::Immediate { immediate: immediate });
                self.inc_rip(1);
                (Instruction::Imul, Some(argument))
            }
            0x70 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jo, Some(arg))
            }
            0x71 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jno, Some(arg))
            }
            0x72 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jb, Some(arg))
            }
            0x73 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jae, Some(arg))
            }
            0x74 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Je, Some(arg))
            }
            0x75 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jne, Some(arg))
            }
            0x76 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jbe, Some(arg))
            }
            0x77 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Ja, Some(arg))
            }
            0x78 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Js, Some(arg))
            }
            0x79 => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jns, Some(arg))
            }
            0x7A => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jp, Some(arg))
            }
            0x7B => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jnp, Some(arg))
            }
            0x7C => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jl, Some(arg))
            }
            0x7D => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jge, Some(arg))
            }
            0x7E => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jle, Some(arg))
            }
            0x7F => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jg, Some(arg))
            }
            0x80 => {
                // arithmetic operation (8bit register target, 8bit immediate)
                let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::Bit8,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Arithmetic, Some(argument))
            }
            0x81 => {
                // arithmetic operation (32/64bit register target, 32bit immediate)
                let (argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::Bit32,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Arithmetic, Some(argument))
            }
            0x83 => {
                // arithmetic operation (32/64bit register target, 8bit immediate)
                let (argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::Bit8,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Arithmetic, Some(argument))
            }
            0x84 => {
                // test
                let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                RegOrOpcode::Register,
                                                                ImmediateSize::None,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Test, Some(argument))
            }
            0x85 => {
                // test
                let (argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Register,
                                                                ImmediateSize::None,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Test, Some(argument))
            }
            0x88 => {
                // mov
                let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                RegOrOpcode::Register,
                                                                ImmediateSize::None,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Mov, Some(argument))
            }
            0x89 => {
                // mov
                let (argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Register,
                                                                ImmediateSize::None,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Mov, Some(argument))
            }
            0x8A => {
                // mov
                let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                RegOrOpcode::Register,
                                                                ImmediateSize::None,
                                                                decoder_flags | REVERSED_REGISTER_DIRECTION);
                self.inc_rip(ip_offset);
                (Instruction::Mov, Some(argument))
            }
            0x90 => {
                self.inc_rip(1);
                (Instruction::Nop, None)
            }
            0x8B => {
                let (argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Register,
                                                                ImmediateSize::None,
                                                                decoder_flags |
                                                                REVERSED_REGISTER_DIRECTION);
                self.inc_rip(ip_offset);
                (Instruction::Mov, Some(argument))
            }
            0x8F => {
                let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags |
                                                                  REVERSED_REGISTER_DIRECTION);
                argument.second_argument = None;
                self.inc_rip(ip_offset);
                (Instruction::Pop, Some(argument))
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
                (Instruction::Mov, Some(argument))
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
                (Instruction::Lea, Some(argument))
            }
            0x98 => {
                let (register1, register2) = if decoder_flags.contains(OPERAND_16_BIT) {
                    (Register::AL, Register::AX)
                } else if decoder_flags.contains(OPERAND_64_BIT) {
                    (Register::EAX, Register::RAX)
                } else {
                    (Register::AX, Register::EAX)
                };

                let argument = InstructionArgumentsBuilder::new().first_argument(
                    InstructionArgument::Register{register: register1}
                ).second_argument(InstructionArgument::Register{register: register2})
                .finalize();
                self.inc_rip(1);
                (Instruction::Mov, Some(argument))
            }
            0x99 => {
                let (register1, register2) = if decoder_flags.contains(OPERAND_16_BIT) {
                    (Register::AX, Register::DX)
                } else if decoder_flags.contains(OPERAND_64_BIT) {
                    (Register::RAX, Register::RDX)
                } else {
                    (Register::EAX, Register::EDX)
                };

                let argument = InstructionArgumentsBuilder::new().first_argument(
                    InstructionArgument::Register{register: register1}
                ).second_argument(InstructionArgument::Register{register: register2})
                .finalize();
                self.inc_rip(1);
                (Instruction::Mov, Some(argument))
            }
            0x9C => {
                self.inc_rip(1);
                (Instruction::Pushf, None)
            }
            0x9D => {
                self.inc_rip(1);
                (Instruction::Popf, None)
            }
            0xA4 => {
                println!("{:?}", decoder_flags);
                self.inc_rip(1);
                (Instruction::Movs, Some(InstructionArgumentsBuilder::new()
                    .repeat(decoder_flags.contains(REPEAT))
                    .explicit_size(ArgumentSize::Bit8)
                    .finalize()))
            }
            0xA5 => {
                let argument_size = if decoder_flags.contains(OPERAND_16_BIT) {
                    ArgumentSize::Bit16
                } else if decoder_flags.contains(OPERAND_64_BIT) {
                    ArgumentSize::Bit64
                } else {
                    ArgumentSize::Bit32
                };
                self.inc_rip(1);
                (Instruction::Movs, Some(InstructionArgumentsBuilder::new()
                    .repeat(decoder_flags.contains(REPEAT))
                    .explicit_size(argument_size)
                    .finalize()))
            }
            0xA8 => {
                let argument = self.decode_al_immediate();
                (Instruction::Test, Some(argument))
            }
            0xA9 => {
                let argument = self.decode_ax_immediate(register_size, decoder_flags);
                (Instruction::Test, Some(argument))
            }
            0xAA => {
                self.inc_rip(1);
                (Instruction::Stos, Some(InstructionArgumentsBuilder::new()
                    .repeat(decoder_flags.contains(REPEAT))
                    .finalize()))
            }
            0xAB => {
                self.inc_rip(1);
                (Instruction::Stos, Some(InstructionArgumentsBuilder::new()
                    .repeat(decoder_flags.contains(REPEAT))
                    .finalize()))
            }
            0xAE => {
                self.inc_rip(1);
                (Instruction::Scas, Some(InstructionArgumentsBuilder::new()
                    .first_argument(InstructionArgument::EffectiveAddress{
                        base: Some(Register::RDI),
                        index: None,
                        scale: None,
                        displacement: 0,
                     })
                    .second_argument(InstructionArgument::Register{ register: Register::AL })
                    .repeat(decoder_flags.contains(REPEAT))
                    .finalize()))
            }
            opcode @ 0xB0...0xB7 => {
                let immediate = self.machine_state.mem_read_byte(rip + 1) as i64;
                let argument =
                    InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
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
                (Instruction::Mov, Some(argument))
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
                    InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
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
                (Instruction::Mov, Some(argument))
            }
            0xC6 => {
                let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::Bit8,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Mov, Some(argument))
            }
            0xC7 => {
                let (argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::Bit32,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::Mov, Some(argument))
            }
            0xC1 => {
                let (argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::Bit8,
                                                                decoder_flags);
                self.inc_rip(ip_offset);
                (Instruction::ShiftRotate, Some(argument))
            }
            0xC3 => {
                self.inc_rip(0);
                (Instruction::Ret, None)
            }
            0xC9 => {
                self.inc_rip(1);
                (Instruction::Leave, None)
            }
            0xCB => {
                self.inc_rip(0);
                (Instruction::Ret, None)
            }
            0xD1 => {
                let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::None,
                                                                decoder_flags);
                argument.second_argument = Some(argument.first_argument.unwrap());
                argument.first_argument = Some(InstructionArgument::Immediate{
                    immediate: 1,
                });
                self.inc_rip(ip_offset);
                (Instruction::ShiftRotate, Some(argument))
            }
            0xD3 => {
                let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                RegOrOpcode::Opcode,
                                                                ImmediateSize::None,
                                                                decoder_flags);
                argument.second_argument = Some(argument.first_argument.unwrap());
                argument.first_argument = Some(InstructionArgument::Register{
                    register: Register::CL
                });
                self.inc_rip(ip_offset);
                (Instruction::ShiftRotate, Some(argument))
            }
            0xEB => {
                let (arg, ip_offset) = self.read_immediate_8bit();
                self.inc_rip(ip_offset);
                (Instruction::Jmp, Some(arg))
            }
            0xE8 => {
                let immediate = self.get_i32_value(1);
                self.inc_rip(5);
                (Instruction::Call, Some(
                            InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
                                immediate: immediate as i64,
                            }).finalize()))
            }
            0xE9 => {
                let immediate = self.get_i32_value(1);
                self.inc_rip(5);
                (Instruction::Jmp, Some(
                            InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
                                immediate: immediate as i64,
                            }).finalize()))
            }
            0xEE => {
                self.inc_rip(1);
                (Instruction::Out, None)
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
                (Instruction::CompareMulOperation, Some(argument))
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
                        let register = get_register(
                            0, register_size,decoder_flags.contains(NEW_64BIT_REGISTER), false);

                        (InstructionArgumentsBuilder::new().first_argument(
                            InstructionArgument::Register{register: register})
                            .opcode(opcode)
                            .finalize(),
                        2)
                    },
                    _ => unreachable!()
                };
                self.inc_rip(ip_offset);
                (Instruction::CompareMulOperation, Some(argument))
            }
            0xFA => {
                // todo: implement cli instruction
                self.inc_rip(1);
                (Instruction::Nop, None)
            }
            0xFB => {
                // todo: implement sti instruction
                self.inc_rip(1);
                (Instruction::Nop, None)
            }
            0xFC => {
                self.inc_rip(1);
                (Instruction::Cld, None)
            }
            0xFD => {
                self.inc_rip(1);
                (Instruction::Std, None)
            }
            0xFF => {
                // todo: cleanup code
                let modrm = self.machine_state.mem_read_byte(rip + 1);
                let opcode = (modrm & 0b00111000) >> 3;
                let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                  RegOrOpcode::Register,
                                                                  ImmediateSize::None,
                                                                  decoder_flags |
                                                                  REVERSED_REGISTER_DIRECTION);
                argument.second_argument = None;
                argument.opcode = Some(opcode);
                self.inc_rip(ip_offset);
                (Instruction::RegisterOperation, Some(argument))
            }
            0x0F => {
                // two byte instructions
                self.machine_state.rip += 1;
                let rip = self.machine_state.rip as u64;
                let second_byte = self.machine_state.mem_read_byte(rip);
                match second_byte {
                    0x01 => {
                        let modrm = self.machine_state.mem_read_byte(rip + 1);
                        let opcode = (modrm & 0b00111000) >> 3;
                        match opcode {
                            2  | 3 => {
                                let table = self.machine_state.mem_read(rip + 2, 4);
                                let table = *zero::read::<i32>(&table);
                                let argument = InstructionArgumentsBuilder::new()
                                    .first_argument(InstructionArgument::Immediate{ immediate: table as i64 })
                                    .finalize();
                                    self.inc_rip(6);
                                if opcode == 2 {
                                    (Instruction::Lgdt, Some(argument))
                                } else {
                                    (Instruction::Lidt, Some(argument))
                                }
                            },
                            _ => panic!("0F 01 unsupported opcode: {:x}", opcode)
                        }
                    }
                    0x1F => {
                        // NOP with hint
                        let (_, ip_offset) = self.get_argument(register_size,
                                                                        RegOrOpcode::Register,
                                                                        ImmediateSize::None,
                                                                        decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Nop, None)
                    }
                    0x20 => {
                        let (mut argument, ip_offset) = self.get_argument(RegisterSize::Bit64,
                                                                      RegOrOpcode::Register,
                                                                      ImmediateSize::None,
                                                                      decoder_flags);
                        let register = match argument.first_argument.unwrap() {
                            InstructionArgument::Register { register } => {
                                match register {
                                    Register::R8 => Register::CR8,
                                    Register::RAX => Register::CR0,
                                    Register::RDX => Register::CR2,
                                    Register::RBX => Register::CR3,
                                    Register::RSP => Register::CR4,
                                    _ => panic!("Invalid argument for mov r64, CRn instruciton"),
                                }
                            },
                            _ => panic!("Invalid argument for mov r64, CRn instruciton"),
                        };
                        argument.first_argument = Some(InstructionArgument::Register {register: register});
                        self.inc_rip(ip_offset);
                        (Instruction::Mov, Some(argument))
                    },
                    0x22 => {
                        let (mut argument, ip_offset) = self.get_argument(RegisterSize::Bit64,
                                                                      RegOrOpcode::Register,
                                                                      ImmediateSize::None,
                                                                      decoder_flags | REVERSED_REGISTER_DIRECTION);
                        let register = match argument.second_argument.unwrap() {
                            InstructionArgument::Register { register } => {
                                match register {
                                    Register::R8 => Register::CR8,
                                    Register::RAX => Register::CR0,
                                    Register::RDX => Register::CR2,
                                    Register::RBX => Register::CR3,
                                    Register::RSP => Register::CR4,
                                    _ => panic!("Invalid argument for mov r64, CRn instruciton"),
                                }
                            },
                            _ => panic!("Invalid argument for mov r64, CRn instruciton"),
                        };
                        argument.second_argument = Some(InstructionArgument::Register {register: register});
                        self.inc_rip(ip_offset);
                        (Instruction::Mov, Some(argument))
                    },
                    0x30 => {
                        self.inc_rip(1);
                        (Instruction::Wrmsr, None)
                    }
                    0x32 => {
                        self.inc_rip(1);
                        (Instruction::Rdmsr, None)
                    }
                    0x40 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovo, Some(argument))
                    },
                    0x41 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovno, Some(argument))
                    },
                    0x42 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovb, Some(argument))
                    },
                    0x43 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovae, Some(argument))
                    },
                    0x44 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmove, Some(argument))
                    },
                    0x45 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovne, Some(argument))
                    },
                    0x46 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovbe, Some(argument))
                    },
                    0x47 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmova, Some(argument))
                    },
                    0x48 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovs, Some(argument))
                    },
                    0x49 => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovns, Some(argument))
                    },
                    0x4a => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovp, Some(argument))
                    },
                    0x4b => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovnp, Some(argument))
                    },
                    0x4c => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovl, Some(argument))
                    },
                    0x4d => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovge, Some(argument))
                    },
                    0x4e => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovle, Some(argument))
                    },
                    0x4f => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Cmovg, Some(argument))
                    },
                    0x80 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jo, Some(argument))
                    },
                    0x81 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jno, Some(argument))
                    },
                    0x82 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jb, Some(argument))
                    },
                    0x83 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jae, Some(argument))
                    },
                    0x84 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Je, Some(argument))
                    },
                    0x85 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jne, Some(argument))
                    },
                    0x86 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jbe, Some(argument))
                    },
                    0x87 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Ja, Some(argument))
                    },
                    0x88 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Js, Some(argument))
                    },
                    0x89 => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jns, Some(argument))
                    },
                    0x8A => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jp, Some(argument))
                    },
                    0x8B => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jnp, Some(argument))
                    },
                    0x8C => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jl, Some(argument))
                    },
                    0x8D => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jge, Some(argument))
                    },
                    0x8E => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jle, Some(argument))
                    },
                    0x8F => {
                        // TODO: could also be 16bit value
                        let argument = self.read_immediate_32bit();
                        self.inc_rip(5);
                        (Instruction::Jg, Some(argument))
                    },
                    0x94 => {
                        let (mut argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                                        RegOrOpcode::Register,
                                                                        ImmediateSize::None,
                                                                        decoder_flags);
                        // TODO: change this hack to Something sane
                        argument.first_argument = Some(argument.second_argument.unwrap());
                        argument.second_argument = None;
                        self.inc_rip(ip_offset);
                        (Instruction::Sete, Some(argument))
                    },
                    0xA2 => {
                        self.inc_rip(1);
                        (Instruction::Cpuid, None)
                    }
                    0xAF => {
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                    RegOrOpcode::Register,
                                                                    ImmediateSize::None,
                                                                    decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.inc_rip(ip_offset);
                        (Instruction::Imul, Some(argument))
                    }
                    0xB6 => {
                        let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                            RegOrOpcode::Register,
                                                                            ImmediateSize::None,
                                                                            decoder_flags | REVERSED_REGISTER_DIRECTION);

                        self.override_argument_size(&mut argument, ArgumentSize::Bit8, rip, &decoder_flags);
                        self.inc_rip(ip_offset);
                        (Instruction::Movzx, Some(argument))
                    }
                    0xB7 => {
                        let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                            RegOrOpcode::Register,
                                                                            ImmediateSize::None,
                                                                            decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.override_argument_size(&mut argument, ArgumentSize::Bit16, rip, &decoder_flags);
                        self.inc_rip(ip_offset);
                        (Instruction::Movzx, Some(argument))
                    }
                    0xBA => {
                        // bit manipulation
                        let (argument, ip_offset) = self.get_argument(register_size,
                                                                      RegOrOpcode::Opcode,
                                                                      ImmediateSize::Bit8,
                                                                      decoder_flags);
                        self.inc_rip(ip_offset);
                        (Instruction::BitManipulation, Some(argument))
                    }
                    0xBE => {
                        let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                            RegOrOpcode::Register,
                                                                            ImmediateSize::None,
                                                                            decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.override_argument_size(&mut argument, ArgumentSize::Bit8, rip, &decoder_flags);
                        self.inc_rip(ip_offset);
                        (Instruction::Movsx, Some(argument))
                    }
                    0xBF => {
                        let (mut argument, ip_offset) = self.get_argument(register_size,
                                                                            RegOrOpcode::Register,
                                                                            ImmediateSize::None,
                                                                            decoder_flags | REVERSED_REGISTER_DIRECTION);
                        self.override_argument_size(&mut argument, ArgumentSize::Bit16, rip, &decoder_flags);
                        self.inc_rip(ip_offset);
                        (Instruction::Movsx, Some(argument))
                    }
                    _ => panic!("Unknown instruction: 0F {:X}, executed instructions: {}", second_byte, self.counter),
                }
            }
            0xCC => {
                // abuse int 3 instruction to signal failed test program
                panic!("int3 instruction");
            }
            0xCD => {
                // abuse int X instruction to signal passed test program
                (Instruction::Int, None)
            }
            _ => panic!("Unknown instruction: {:x}, executed instructions: {}", first_byte, self.counter),
        }
    }

    fn fetch_argument(cache_entry: &InstructionCache) -> &InstructionArguments {
        match cache_entry.arguments {
            Some(ref arg) => arg,
            None => panic!("expected arg")
        }
    }

    fn execute_instruction(&mut self, cache_entry: &InstructionCache) {
        match cache_entry.instruction {
            Instruction::Adc => self.cpu.adc(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Add => self.cpu.add(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::And => self.cpu.and(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Arithmetic => self.cpu.arithmetic(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::BitManipulation => self.cpu.bit_manipulation(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Call => self.cpu.call(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cld => self.cpu.cld(self.machine_state),
            Instruction::Cmova => self.cpu.cmova(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovae => self.cpu.cmovae(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovb => self.cpu.cmovb(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovbe => self.cpu.cmovbe(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmove => self.cpu.cmove(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovg => self.cpu.cmovg(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovge => self.cpu.cmovge(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovl => self.cpu.cmovl(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovle => self.cpu.cmovle(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovne => self.cpu.cmovne(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovno => self.cpu.cmovno(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovnp => self.cpu.cmovnp(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovns => self.cpu.cmovns(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovo => self.cpu.cmovo(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovp => self.cpu.cmovp(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmovs => self.cpu.cmovs(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cmp => self.cpu.cmp(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Cpuid => self.cpu.cpuid(self.machine_state),
            Instruction::CompareMulOperation => self.cpu.compare_mul_operation(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Imul => self.cpu.imul(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Int => {
                if self.machine_state.print_instructions {
                    self.machine_state.print_instr("int    $0x80");
                }
            },
            Instruction::Ja => self.cpu.ja(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jae => self.cpu.jae(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jb => self.cpu.jb(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jbe => self.cpu.jbe(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Je => self.cpu.je(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jg => self.cpu.jg(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jge => self.cpu.jge(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jl => self.cpu.jl(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jle => self.cpu.jle(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jmp => self.cpu.jmp(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jne => self.cpu.jne(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jno => self.cpu.jno(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jnp => self.cpu.jnp(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jns => self.cpu.jns(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jo => self.cpu.jo(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Jp => self.cpu.jp(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Js => self.cpu.js(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Lea => self.cpu.lea(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Leave => self.cpu.leave(self.machine_state),
            Instruction::Lidt => self.cpu.lidt(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Lgdt => self.cpu.lgdt(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Mov => self.cpu.mov(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Movs => self.cpu.movs(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Movsx => self.cpu.movsx(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Movzx => self.cpu.movzx(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Nop => (),
            Instruction::Or => self.cpu.or(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Out => self.cpu.out(self.machine_state),
            Instruction::Pop => self.cpu.pop(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Popf => self.cpu.popf(self.machine_state),
            Instruction::Push => self.cpu.push(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Pushf => self.cpu.pushf(self.machine_state),
            Instruction::RegisterOperation => self.cpu.register_operation(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Ret => self.cpu.ret(self.machine_state),
            Instruction::Rdmsr => self.cpu.rdmsr(self.machine_state),
            Instruction::Sbb => self.cpu.sbb(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Sete => self.cpu.sete(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::ShiftRotate => self.cpu.shift_rotate(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Std => self.cpu.std(self.machine_state),
            Instruction::Stos => self.cpu.stos(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Sub => self.cpu.sub(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Test => self.cpu.test(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Wrmsr => self.cpu.wrmsr(self.machine_state),
            Instruction::Xor => self.cpu.xor(self.machine_state, Decoder::fetch_argument(cache_entry)),
            Instruction::Scas => self.cpu.scas(self.machine_state, Decoder::fetch_argument(cache_entry)),
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

    fn get_i8_value(&mut self, ip_offset: i64) -> i8 {
        let rip = (self.machine_state.rip + ip_offset) as u64;
        self.machine_state.mem_read_byte(rip) as i8
    }

    fn read_immediate_8bit(&mut self) -> (InstructionArguments, i64) {
        let rip = self.machine_state.rip as u64;
        let immediate = self.machine_state.mem_read_byte(rip + 1) as i8 as i64;

        (InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate { immediate: immediate })
             .finalize(),
         2)
    }

    fn read_immediate_32bit(&mut self) -> InstructionArguments {
        let immediate = self.get_i32_value(1) as i64;
        InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate { immediate: immediate })
             .finalize()
    }

    fn get_argument(&mut self,
                    register_size: RegisterSize,
                    reg_or_opcode: RegOrOpcode,
                    immediate_size: ImmediateSize,
                    mut decoder_flags: DecoderFlags)
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
                    0b00 => {
                        match sib {
                            Some(sib) => {
                                let base = sib & 0b00000111;
                                if base == 0x5 {
                                    let displacement = self.get_i32_value(offset);
                                    decoder_flags |= SIB_DISPLACEMENT_ONLY;
                                    (displacement, 4)
                                } else {
                                    (0, 0)
                                }
                            },
                            None => (0, 0)
                        }
                    }
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

                        (InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
                                 immediate: immediate as i64,
                             })
                             .second_argument(self.effective_address(sib, register, displacement, decoder_flags))
                             .opcode(register_or_opcode)
                             .explicit_size(argument_size)
                             .finalize(),
                         ip_offset + 1)
                    }
                    ImmediateSize::Bit32 => {
                        assert!(reg_or_opcode == RegOrOpcode::Opcode);
                        let immediate = if decoder_flags.contains(OPERAND_16_BIT) {
                            ip_offset += 2;
                            self.get_i16_value(ip_offset - 2) as i64
                        } else {
                            ip_offset += 4;
                            self.get_i32_value(ip_offset - 4) as i64
                        };

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

                        (InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
                                 immediate: immediate,
                             })
                             .second_argument(self.effective_address(sib, register, displacement, decoder_flags))
                             .opcode(register_or_opcode)
                             .explicit_size(argument_size)
                             .finalize(),
                         ip_offset)
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
                             InstructionArgumentsBuilder::new().first_argument(self.effective_address(sib, register1, displacement, decoder_flags))
                             .second_argument(
                                InstructionArgument::Register {
                                    register: register2,
                                }).finalize()
                         } else {
                             InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Register {
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
                             InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Register {
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
                             InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Register {
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
                                (InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
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
                                (InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
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
                                (InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Register {
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
                    base: Some(register),
                    index: None,
                    scale: None,
                    displacement: displacement,
                }
            }
            Some(sib) => {
                let base_num = sib & 0b00000111;
                let index = (sib & 0b00111000) >> 3;
                let scale = (sib & 0b11000000) >> 6;
                let scale = 2u8.pow(scale as u32) as u8;

                let register_size = if decoder_flags.contains(ADDRESS_SIZE_OVERRIDE) {
                    RegisterSize::Bit32
                } else {
                    RegisterSize::Bit64
                };

                let base = get_register(base_num, register_size,
                                       decoder_flags.contains(NEW_64BIT_REGISTER), false);

                if index == 0x4 {
                    if base_num == 0x5 && decoder_flags.contains(SIB_DISPLACEMENT_ONLY) {
                        InstructionArgument::EffectiveAddress {
                            base: None,
                            displacement: displacement,
                            scale: None,
                            index: None,
                        }
                    } else {
                        InstructionArgument::EffectiveAddress {
                            base: Some(base),
                            displacement: displacement,
                            scale: None,
                            index: None,
                        }
                    }
                } else {
                    if base_num == 0x5 && decoder_flags.contains(SIB_DISPLACEMENT_ONLY) {
                        InstructionArgument::EffectiveAddress {
                            base: None,
                            displacement: displacement,
                            scale: Some(scale),
                            index: Some(get_register(index, register_size,
                                                    decoder_flags.contains(SIB_EXTENSION), false))
                        }
                    } else {
                        InstructionArgument::EffectiveAddress {
                            base: Some(base),
                            displacement: displacement,
                            scale: Some(scale),
                            index: Some(get_register(index, register_size,
                                                    decoder_flags.contains(SIB_EXTENSION), false))
                        }
                    }
                }
            }
        }

    }


    fn decode_8bit_reg_8bit_immediate(&mut self, decoder_flags: DecoderFlags) -> InstructionArguments {
        let (argument, ip_offset) = self.get_argument(RegisterSize::Bit8,
                                                      RegOrOpcode::Register,
                                                      ImmediateSize::None,
                                                      decoder_flags);
        self.inc_rip(ip_offset);
        argument
    }

    fn decode_reg_reg(&mut self, register_size: RegisterSize, decoder_flags: DecoderFlags) -> InstructionArguments {
        let (argument, ip_offset) = self.get_argument(register_size,
                                                      RegOrOpcode::Register,
                                                      ImmediateSize::None,
                                                      decoder_flags);
        self.inc_rip(ip_offset);
        argument
    }

    fn decode_al_immediate(&mut self) -> InstructionArguments {
        let immediate = self.get_i8_value(1);
        let argument =
            InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
                    immediate: immediate as i64,
                })
                .second_argument(InstructionArgument::Register {
                    register: Register::AL,
                })
                .finalize();
        self.inc_rip(2);
        argument
    }

    fn decode_ax_immediate(&mut self, register_size: RegisterSize, decoder_flags: DecoderFlags) -> InstructionArguments {
        let (immediate, ip_offset) = if decoder_flags.contains(OPERAND_16_BIT) {
            (self.get_i16_value(1) as i64, 3)
        } else {
            (self.get_i32_value(1) as i64, 5)
        };

        let register = get_register(0,
            register_size, decoder_flags.contains(NEW_64BIT_REGISTER),
            false);

        let argument =
            InstructionArgumentsBuilder::new().first_argument(InstructionArgument::Immediate {
                    immediate: immediate,
                })
                .second_argument(InstructionArgument::Register {
                    register: register,
                })
                .finalize();
        self.inc_rip(ip_offset);
        argument
    }

    fn override_argument_size(&mut self,
                              argument: &mut InstructionArguments,
                              size: ArgumentSize,
                              rip: u64,
                              decoder_flags: &DecoderFlags) {
        
        let new_first_argument = match argument.first_argument {
            Some(ref first_argument) => {
                match *first_argument {
                    InstructionArgument::Register {..}=> {
                        let register_size = match size {
                            ArgumentSize::Bit8 => RegisterSize::Bit8,
                            ArgumentSize::Bit16 => RegisterSize::Bit16,
                            ArgumentSize::Bit32 => RegisterSize::Bit32,
                            ArgumentSize::Bit64 => RegisterSize::Bit64,
                        };
                        let modrm = self.machine_state.mem_read_byte(rip + 1);
                        let register = modrm & 0b00000111;
                        let register = get_register(register, register_size,
                                                    decoder_flags.contains(NEW_64BIT_REGISTER),
                                                    decoder_flags.contains(NEW_8BIT_REGISTER));
                        Some(InstructionArgument::Register {
                            register: register,
                        })
                    },
                    InstructionArgument::EffectiveAddress {..} => {
                        argument.explicit_size = Some(size);
                        None
                    },
                    _ => panic!("Invalid argument")
                }
            },
            None => panic!("Needs first_argument to override argument size"),
        };
        match new_first_argument {
            Some(nfa) => argument.first_argument = Some(nfa),
            None => (),
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
        const SIB_DISPLACEMENT_ONLY = 1 << 10,
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
