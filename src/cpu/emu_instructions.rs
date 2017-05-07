use instruction_set::{InstructionArgument, InstructionArguments, Register, Flags};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;
use instruction_set::{ArgumentSize, get_register_size, print_instr, print_instr_arg, print_instr_arg_no_size};
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

    fn jmp_iml(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        arg.assert_one_argument();
        let value = machine_state.get_value(&arg.first_argument, arg.size());
        match arg.first_argument {
            InstructionArgument::Register { .. } => machine_state.rip = value,
            InstructionArgument::Immediate { .. } => machine_state.rip += value,
            InstructionArgument::EffectiveAddress { .. } => {
                panic!("Unsupported argument for jump");
            }
        }
    }

    fn mov_impl(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        machine_state.set_value(value, &arg.second_argument.unwrap(), argument_size);
    }
}

impl CPU for EmulationCPU {
    fn push(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("push", &arg);

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
        print_instr_arg("pop", &arg);
        arg.assert_one_argument();
        let value = machine_state.stack_pop();
        machine_state.set_value(value, &arg.first_argument, arg.size());
    }

    fn mov(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("mov", &arg);
        self.mov_impl(machine_state, arg);
    }

    fn movsx(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg_no_size("movsx", &arg);
        // normal mov already does the sign extension
        self.mov_impl(machine_state, arg);
    }

    fn movzx(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg_no_size("movzx", &arg);
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
        print_instr_arg("add", &arg);
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
        print_instr_arg("or", &arg);
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
        print_instr_arg("adc", &arg);
        panic!("Not implemented");
    }

    fn sbb(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("sbb", &arg);
        self.sub_impl(machine_state, arg, true);
        println!("WARNING: SBB implemented without carry")
    }

    fn and(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("and", &arg);
        self.and_impl(machine_state, arg, true);
    }

    fn sub(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("sub", &arg);
        self.sub_impl(machine_state, arg, true);
    }

    fn xor(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("xor", &arg);
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
        print_instr_arg("cmp", &arg);
        self.sub_impl(machine_state, arg, false);
    }

    fn call(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("call", &arg);
        arg.assert_one_argument();

        let rip = convert_i64_to_u8vec(machine_state.rip);
        machine_state.stack_push(&rip);

        self.jmp_iml(machine_state, arg);
    }

    fn lea(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("lea", &arg);
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
        print_instr_arg("test", &arg);
        println!("WARNING: test not fully implemented");
        self.and_impl(machine_state, arg, false);
    }

    fn cmovo(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovo", &arg);
        if machine_state.get_flag(Flags::Overflow) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovno(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovno", &arg);
        if !machine_state.get_flag(Flags::Overflow) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovb(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovb", &arg);
        if machine_state.get_flag(Flags::Carry) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovae(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovae", &arg);
        if !machine_state.get_flag(Flags::Carry) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmove(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmove", &arg);
        if machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovne(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovne", &arg);
        if !machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovbe(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovbe", &arg);
        if machine_state.get_flag(Flags::Carry) || machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmova(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmova", &arg);
        if !machine_state.get_flag(Flags::Carry) && !machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovs(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovs", &arg);
        if machine_state.get_flag(Flags::Sign) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovns(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovns", &arg);
        if !machine_state.get_flag(Flags::Sign) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovp", &arg);
        if machine_state.get_flag(Flags::Parity) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovnp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovnp", &arg);
        if !machine_state.get_flag(Flags::Parity) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovl(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovl", &arg);
        if machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow){
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovge(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovge", &arg);
        if machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow){
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovle(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovle", &arg);
        if machine_state.get_flag(Flags::Zero) ||
                (machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow)) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn cmovg(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("cmovg", &arg);
        if !machine_state.get_flag(Flags::Zero) &&
                (machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow)) {
            self.mov_impl(machine_state, arg);
        }
    }

    fn rol(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("rol", &arg);
    }

    fn ror(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("ror", &arg);
        panic!("Not implemented");
    }

    fn rcl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("rcl", &arg);
        panic!("Not implemented");
    }

    fn rcr(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("rcr", &arg);
        panic!("Not implemented");
    }

    fn shl(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("shl", &arg);
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
        print_instr_arg("shr", &arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size) as u64;
        let value2 = machine_state.get_value(&second_argument, argument_size) as u64;
        let result = (value2 >> value1) as i64;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        println!("WARNING: shr does not set carry/overflow flag");
    }

    fn sar(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("sar", &arg);
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
        print_instr_arg("inc", &arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = value + 1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);
    }

    fn dec(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("dec", &arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = value - 1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);

    }

    fn div(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("div", &arg);
        panic!("Not implemented");
    }

    fn idiv(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("idiv", &arg);
        panic!("Not implemented");
    }

    fn mul(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("mul", &arg);
        panic!("Not implemented");
    }

    fn imul(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("imul", &arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 * value1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        println!("WARNING: instruction argument decoding is invalid");
        println!("WARNING: imul does not set carry/overflow flag");
    }

    fn not(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("not", &arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = !value;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);
    }

    fn neg(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("neg", &arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        let result = -value;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &arg.first_argument, argument_size);
    }

    fn ret(&self, machine_state: &mut MachineState) {
        print_instr("ret");
        let value = machine_state.stack_pop();
        machine_state.rip = value;
    }

    fn leave(&self, machine_state: &mut MachineState) {
        print_instr("leave");
        let value = machine_state.get_register_value(&Register::RBP);
        machine_state.set_register_value(&Register::RSP, value);
        let value = machine_state.stack_pop();
        machine_state.set_register_value(&Register::RBP, value);
    }

    fn popf(&self, machine_state: &mut MachineState) {
        print_instr("popf");
        let value = machine_state.stack_pop();
        machine_state.rflags = value;
    }

    fn std(&self, machine_state: &mut MachineState) {
        print_instr("std");
        machine_state.set_flag(Flags::Direction, true);
    }

    fn cld(&self, machine_state: &mut MachineState) {
        print_instr("cld");
        machine_state.set_flag(Flags::Direction, false);
    }

    fn stos(&self, machine_state: &mut MachineState, repeat: bool) {
        let to =
            machine_state.get_value(&InstructionArgument::Register { register: Register::RDI },
                                    ArgumentSize::Bit64);
        if repeat {
            let mut length =
                machine_state.get_value(&InstructionArgument::Register { register: Register::RCX },
                                        ArgumentSize::Bit64);
            length *= 8; // 8 bytes per repeat
            print_instr("rep stos %rax,%es:(%rdi)");
            if machine_state.get_flag(Flags::Direction) {
                panic!("stos NOOP");
            } else {
                // TODO: actually do something
                machine_state.set_register_value(&Register::RDI, to + length);
                machine_state.set_register_value(&Register::RCX, 0);
            }
        } else {
            print_instr("stos %ds:(%rsi),%es:(%rdi)");
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
            print_instr("rep movs %ds:(%rsi),%es:(%rdi)");
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
                machine_state.set_register_value(&Register::RSI, from);
                machine_state.set_register_value(&Register::RDI, to);
            } else {
                let data = machine_state.mem_read(from as u64, length as u64);
                machine_state.mem_write(to as u64, &data);
                println!("WARNING: rsi and rdi not set");
                // TODO: set rsi, rdi registers
            }
            machine_state.set_register_value(&Register::RCX, 0);
        } else {
            print_instr("movs %ds:(%rsi),%es:(%rdi)");
            panic!("Not implemented");
        }
    }

    fn jmp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jmp", &arg);
        self.jmp_iml(machine_state, arg);
    }

    fn jo(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jo", &arg);
        if machine_state.get_flag(Flags::Overflow) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jno(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jno", &arg);
        if !machine_state.get_flag(Flags::Overflow) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jb(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jb", &arg);
        if machine_state.get_flag(Flags::Carry) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jae(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jae", &arg);
        if !machine_state.get_flag(Flags::Carry) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn je(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("je", &arg);
        if machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jne(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jne", &arg);
        if !machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jbe(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jbe", &arg);
        // CF=1 OR ZF=1
        if machine_state.get_flag(Flags::Carry) || machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn ja(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("ja", &arg);
        // CF=0 AND ZF=0
        if !machine_state.get_flag(Flags::Carry) && !machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn js(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("js", &arg);
        if machine_state.get_flag(Flags::Sign) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jns(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jns", &arg);
        if !machine_state.get_flag(Flags::Sign) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jp", &arg);
        if machine_state.get_flag(Flags::Parity) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jnp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("jnp", &arg);
        if !machine_state.get_flag(Flags::Parity) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jl(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // SF!=OF
        print_instr_arg("jl", &arg);
        if machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow){
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jge(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // SF=OF
        print_instr_arg("jge", &arg);
        if machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow){
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jle(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // (ZF=1) OR (SF!=OF)
        print_instr_arg("jle", &arg);
        if machine_state.get_flag(Flags::Zero) ||
                (machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow)) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn jg(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        // (ZF=0) AND (SF=OF)
        print_instr_arg("jg", &arg);
        if !machine_state.get_flag(Flags::Zero) &&
                (machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow)) {
            self.jmp_iml(machine_state, arg);
        }
    }

    fn sete(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instr_arg("sete", &arg);
        if machine_state.get_flag(Flags::Zero) {
            machine_state.set_value(1, &arg.first_argument, ArgumentSize::Bit8);
        } else {
            machine_state.set_value(0, &arg.first_argument, ArgumentSize::Bit8);
        }
    }

    fn out(&self, _machine_state: &mut MachineState) {
        println!("{:<6} %al,(%dx)", "out");
        println!("WARNING: out not implemented");
    }
}
