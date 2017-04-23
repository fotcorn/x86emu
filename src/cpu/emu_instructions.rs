use instruction_set::{InstructionArgument, InstructionArguments, Register, Flags};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;
use instruction_set::{ArgumentSize, get_register_size};
use utils::{convert_i32_to_u8vec, convert_i64_to_u8vec};

pub struct EmulationCPU {}

impl EmulationCPU {
    fn sub_impl(&self, machine_state: &mut MachineState, arg: InstructionArguments, set: bool) {
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);

        let (result, carry, overflow) = match argument_size {
            ArgumentSize::Bit8 => {
                let (result, carry) = (value2 as u8).overflowing_sub(value1 as u8);
                let (_, overflow) = (value2 as i8).overflowing_sub(value1 as i8);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit16 => {
                let (result, carry) = (value2 as u16).overflowing_sub(value1 as u16);
                let (_, overflow) = (value2 as i16).overflowing_sub(value1 as i16);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit32 => {
                let (result, carry) = (value2 as u32).overflowing_sub(value1 as u32);
                let (_, overflow) = (value2 as i32).overflowing_sub(value1 as i32);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit64 => {
                let (result, carry) = (value2 as u64).overflowing_sub(value1 as u64);
                let (_, overflow) = (value2 as i64).overflowing_sub(value1 as i64);
                (result as i64, carry, overflow)
            }
        };
        machine_state.set_flag(Flags::Carry, carry);
        machine_state.set_flag(Flags::Overflow, overflow);
        machine_state.compute_flags(result, argument_size);
        if set {
            machine_state.set_value(result, &second_argument, argument_size);
        }
    }

    fn and_impl(&self, machine_state: &mut MachineState, arg: InstructionArguments, set: bool) {
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value1 & value2;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_flag(Flags::Carry, false);
        machine_state.set_flag(Flags::Overflow, false);
        if set {
            machine_state.set_value(result, &second_argument, argument_size);
        }
    }
}

impl CPU for EmulationCPU {
    fn push(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "push", arg);

        arg.assert_one_argument();
        let vector = match arg.size() {
            ArgumentSize::Bit32 => {
                convert_i32_to_u8vec(machine_state.get_value(&arg.first_argument,
                                                             ArgumentSize::Bit32) as i32)
            }
            ArgumentSize::Bit64 => {
                convert_i64_to_u8vec(machine_state.get_value(&arg.first_argument,
                                                             ArgumentSize::Bit64))
            }
            _ => panic!("Unsupported push value size"),
        };
        machine_state.stack_push(&vector);
    }

    fn pop(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "pop", arg);
        arg.assert_one_argument();
        let value = machine_state.stack_pop();
        machine_state.set_value(value, &arg.first_argument, arg.size());
    }

    fn mov(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "mov", arg);
        arg.assert_two_arguments();
        let value = machine_state.get_value(&arg.first_argument, arg.size());
        let argument_size = arg.size();
        machine_state.set_value(value, &arg.second_argument.unwrap(), argument_size);
    }

    fn movsx(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "movsx", arg);
        arg.assert_two_arguments();
        // get_value already does the sign extension
        let value = machine_state.get_value(&arg.first_argument, arg.size());
        let argument_size = arg.size();
        machine_state.set_value(value, &arg.second_argument.unwrap(), argument_size);
    }

    fn movzx(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "movzx", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);

        let first_argument_size = match arg.first_argument {
            InstructionArgument::Register {ref register} => {
                get_register_size(register)
            },
            InstructionArgument::EffectiveAddress {..} => {
                match arg.explicit_size {
                    Some(explicit_size) => explicit_size,
                    None => panic!("movzx instruction needs explicit size when using an effective address"),
                }
            }
            _ => panic!("Invalid parameter for mov")
        };

        let value = match first_argument_size {
            ArgumentSize::Bit8 => value as u8 as u64,
            ArgumentSize::Bit16 => value as u16 as u64,
            ArgumentSize::Bit32 => value as u32 as u64,
            ArgumentSize::Bit64 => value as u64 as u64,
        };

        // ArgumentSize::Bit64 is not used because target is always a register
        machine_state.set_value(value as i64, &arg.second_argument.unwrap(), ArgumentSize::Bit64);
    }

    fn add(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "add", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);

        let (result, carry, overflow) = match argument_size {
            ArgumentSize::Bit8 => {
                let (result, carry) = (value2 as u8).overflowing_add(value1 as u8);
                let (_, overflow) = (value2 as i8).overflowing_add(value1 as i8);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit16 => {
                let (result, carry) = (value2 as u16).overflowing_add(value1 as u16);
                let (_, overflow) = (value2 as i16).overflowing_add(value1 as i16);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit32 => {
                let (result, carry) = (value2 as u32).overflowing_add(value1 as u32);
                let (_, overflow) = (value2 as i32).overflowing_add(value1 as i32);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit64 => {
                let (result, carry) = (value2 as u64).overflowing_add(value1 as u64);
                let (_, overflow) = (value2 as i64).overflowing_add(value1 as i64);
                (result as i64, carry, overflow)
            }
        };
        machine_state.set_flag(Flags::Carry, carry);
        machine_state.set_flag(Flags::Overflow, overflow);

        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
    }

    fn or(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "or", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value1 | value2;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
    }

    fn adc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "adc", arg);
        panic!("Not implemented");
    }

    fn sbb(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sbb", arg);
        self.sub_impl(machine_state, arg, true);
        println!("WARNING: SBB implemented without carry")
    }

    fn and(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "and", arg);
        self.and_impl(machine_state, arg, true);
    }

    fn sub(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sub", arg);
        self.sub_impl(machine_state, arg, true);
    }

    fn xor(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "xor", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value1 ^ value2;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
    }

    fn cmp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmp", arg);
        self.sub_impl(machine_state, arg, false);
        println!("WARNING: cmp not fully implemented");
    }

    fn call(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "call", arg);
        arg.assert_one_argument();

        let rip = convert_i64_to_u8vec(machine_state.rip);
        machine_state.stack_push(&rip);

        self.jmp(machine_state, arg);
    }

    fn lea(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "lea", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        match arg.first_argument {
            InstructionArgument::EffectiveAddress { .. } => {
                let value = machine_state.calculate_effective_address(&arg.first_argument) as i64;
                let second_argument = arg.second_argument.unwrap();
                match second_argument {
                    InstructionArgument::Register { .. } => {
                        machine_state.set_value(value, &second_argument, argument_size)
                    }
                    _ => panic!("Unsupported lea argument"),
                }
            }
            _ => panic!("Unsupported lea argument"),
        }
    }

    fn test(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "test", arg);
        println!("WARNING: test not fully implemented");
        self.and_impl(machine_state, arg, false);
    }

    fn cmovs(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmovs", arg);
        if machine_state.get_flag(Flags::Sign) {
            self.mov(machine_state, arg);
        }
    }

    fn cmovz(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmovz", arg);
        if machine_state.get_flag(Flags::Zero) {
            self.mov(machine_state, arg);
        }
    }

    fn rol(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "rol", arg);
    }

    fn ror(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "ror", arg);
        panic!("Not implemented");
    }

    fn rcl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "rcl", arg);
        panic!("Not implemented");
    }

    fn rcr(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "rcr", arg);
        panic!("Not implemented");
    }

    fn shl(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "shl", arg);
        println!("{:<6} {}", "shr", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 << value1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        println!("WARNING: shl does not set carry/overflow flag");
    }

    fn shr(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "shr", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 >> value1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        println!("WARNING: shr does not set carry/overflow flag");
    }

    fn sar(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sar", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 >> value1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        println!("WARNING: sar does not preserve highest byte; sets O/C flags");
    }

    fn inc(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "inc", arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = value + 1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);
    }

    fn dec(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "dec", arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = value - 1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);

    }

    fn div(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "div", arg);
        panic!("Not implemented");
    }

    fn idiv(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "idiv", arg);
        panic!("Not implemented");
    }

    fn mul(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "mul", arg);
        panic!("Not implemented");
    }

    fn imul(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "imul", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 * value1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        println!("WARNING: imul does not set carry/overflow flag");
    }

    fn not(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "not", arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = !value;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);
    }

    fn neg(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "neg", arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = -value;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);
    }

    fn ret(&self, machine_state: &mut MachineState) {
        println!("{:<6}", "ret");
        let value = machine_state.stack_pop();
        machine_state.rip = value;
    }

    fn leave(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "leave");
        panic!("Not implemented");
    }

    fn popf(&self, machine_state: &mut MachineState) {
        println!("{:<6}", "popf");
        let value = machine_state.stack_pop();
        machine_state.rflags = value;
    }

    fn std(&self, machine_state: &mut MachineState) {
        println!("{:<6}", "std");
        machine_state.set_flag(Flags::Direction, true);
    }

    fn cld(&self, machine_state: &mut MachineState) {
        println!("{:<6}", "cld");
        machine_state.set_flag(Flags::Direction, false);
    }

    fn stos(&self, _machine_state: &mut MachineState, repeat: bool) {
        if repeat {
            println!("{:<6}", "rep stos %ds:(%rsi),%es:(%rdi)");
        } else {
            println!("{:<6}", "stos %ds:(%rsi),%es:(%rdi)");
        }
        println!("WARNING: stos: NOOP");
    }


    fn movs(&self, machine_state: &mut MachineState, repeat: bool) {
        let mut from =
            machine_state.get_value(&InstructionArgument::Register { register: Register::RSI },
                                    ArgumentSize::Bit64);
        let mut to =
            machine_state.get_value(&InstructionArgument::Register { register: Register::RDI },
                                    ArgumentSize::Bit64);
        // TODO: do not hardcode to 8byte operand
        if repeat {
            println!("{:<6}", "rep movs %ds:(%rsi),%es:(%rdi)");
            let mut length =
                machine_state.get_value(&InstructionArgument::Register { register: Register::RCX },
                                        ArgumentSize::Bit64);
            length *= 8; // 8 bytes per mov
            if machine_state.get_flag(Flags::Direction) {
                println!("WARNING: address calculation could be incorrect");
                from -= length;
                to -= length;
                let data = machine_state.mem_read(from as u64, length as u64);
                machine_state.mem_write(to as u64, &data);
            } else {
                let data = machine_state.mem_read(from as u64, length as u64);
                machine_state.mem_write(to as u64, &data);
            }
            // TODO: set rsi, rdi, rcx registers
        } else {
            println!("{:<6}", "movs %ds:(%rsi),%es:(%rdi)");
            panic!("Not implemented");
        }
    }

    fn jmp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jmp", arg);
        arg.assert_one_argument();
        let value = machine_state.get_value(&arg.first_argument, arg.size());
        match arg.first_argument {
            InstructionArgument::Register { .. } => machine_state.rip = value,
            InstructionArgument::Immediate { .. } => machine_state.rip += value,
            InstructionArgument::EffectiveAddress { .. } => {
                panic!("Unsupported argument for jmp");
            }
        }
    }

    fn jo(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jo", arg);
        if machine_state.get_flag(Flags::Overflow) {
            self.jmp(machine_state, arg);
        }
    }

    fn jno(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jno", arg);
        if !machine_state.get_flag(Flags::Overflow) {
            self.jmp(machine_state, arg);
        }
    }

    fn jc(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jc", arg);
        if machine_state.get_flag(Flags::Carry) {
            self.jmp(machine_state, arg);
        }
    }

    fn jnc(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jnc", arg);
        if !machine_state.get_flag(Flags::Carry) {
            self.jmp(machine_state, arg);
        }
    }

    fn jz(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jz", arg);
        if machine_state.get_flag(Flags::Zero) {
            self.jmp(machine_state, arg);
        }
    }

    fn jnz(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jnz", arg);
        if !machine_state.get_flag(Flags::Zero) {
            self.jmp(machine_state, arg);
        }
    }

    fn jbe(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jbe", arg);
        // CF=1 OR ZF=1
        if machine_state.get_flag(Flags::Carry) || machine_state.get_flag(Flags::Zero) {
            self.jmp(machine_state, arg);
        }
    }

    fn ja(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "ja", arg);
        // CF=0 AND ZF=0
        if !machine_state.get_flag(Flags::Carry) && !machine_state.get_flag(Flags::Zero) {
            self.jmp(machine_state, arg);
        }
    }

    fn js(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "js", arg);
        if machine_state.get_flag(Flags::Sign) {
            self.jmp(machine_state, arg);
        }
    }

    fn jns(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jns", arg);
        if !machine_state.get_flag(Flags::Sign) {
            self.jmp(machine_state, arg);
        }
    }

    fn jp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jp", arg);
        if machine_state.get_flag(Flags::Parity) {
            self.jmp(machine_state, arg);
        }
    }

    fn jnp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jnp", arg);
        if !machine_state.get_flag(Flags::Parity) {
            self.jmp(machine_state, arg);
        }
    }

    fn jl(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // SF!=OF
        println!("{:<6} {}", "jl", arg);
        if machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow){
            self.jmp(machine_state, arg);
        }
    }

    fn jge(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // SF=OF
        println!("{:<6} {}", "jge", arg);
        if machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow){
            self.jmp(machine_state, arg);
        }
    }

    fn jle(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // (ZF=1) OR (SF!=OF)
        if machine_state.get_flag(Flags::Zero) ||
                (machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow)) {
            self.jmp(machine_state, arg);
        }
    }

    fn jg(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // (ZF=0) AND (SF=OF)
        println!("{:<6} {}", "jg", arg);
        if !machine_state.get_flag(Flags::Zero) &&
                (machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow)) {
            self.jmp(machine_state, arg);
        }
    }

    fn setz(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6}", "setz");
        if machine_state.get_flag(Flags::Zero) {
            machine_state.set_value(1, &arg.first_argument, ArgumentSize::Bit8);
        }
    }
}
