use instruction_set::{InstructionArgument, InstructionArguments, Register, Flags};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;
use instruction_set::ArgumentSize;
use utils::{convert_i32_to_u8vec, convert_i64_to_u8vec};

pub struct EmulationCPU {}

impl CPU for EmulationCPU {
    fn push(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "push", arg);

        arg.assert_one_argument();
        let vector = match arg.size() {
            ArgumentSize::Bit32 => {
                convert_i32_to_u8vec(machine_state.get_value(&arg.first_argument, ArgumentSize::Bit32) as i32)
            }
            ArgumentSize::Bit64 => {
                convert_i64_to_u8vec(machine_state.get_value(&arg.first_argument, ArgumentSize::Bit64))
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

    fn add(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "add", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        machine_state.set_value(value1 + value2, &second_argument, argument_size);
    }

    fn or(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "or", arg);
        panic!("Not implemented");
    }

    fn adc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "adc", arg);
        panic!("Not implemented");
    }

    fn sbb(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sbb", arg);
        panic!("Not implemented");
    }

    fn and(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "and", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        machine_state.set_value(value2 & value1, &second_argument, argument_size);
    }

    fn sub(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sub", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        machine_state.set_value(value2 - value1, &second_argument, argument_size);
    }

    fn xor(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "xor", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        machine_state.set_value(value1 ^ value2, &second_argument, argument_size);
    }

    fn cmp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmp", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        println!("cmp: {} {}", value1, value2);
        println!("WARNING: cmp not implemented");
    }

    fn call(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "call", arg);
        let value =
            machine_state.get_value(&InstructionArgument::Register { register: Register::RSI },
                                    ArgumentSize::Bit64);
        println!("{}", value);
    }

    fn lea(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "lea", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        match arg.first_argument {
            InstructionArgument::EffectiveAddress { register, displacement } => {
                let reg = InstructionArgument::Register { register: register };
                let mut value = machine_state.get_value(&reg, argument_size);
                value += displacement as i64;
                if value == -6 {
                    value = 0;
                    println!("WARNING: hacked lea implementation");
                }
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

    fn test(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "test", arg);
        println!("WARNING: test not implemented");
    }

    fn cmov(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmov", arg);
        println!("WARNING: not implemented");
    }

    fn sar(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sar", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        machine_state.set_value(value2 >> value1, &second_argument, argument_size);
    }

    fn inc(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "inc", arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        machine_state.set_value(value + 1, &arg.first_argument, argument_size);
    }

    fn dec(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "dec", arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        machine_state.set_value(value - 1, &arg.first_argument, argument_size);
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

    fn imul(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "imul", arg);
        panic!("Not implemented");
    }

    fn not(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "not", arg);
        arg.assert_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&arg.first_argument, argument_size);
        machine_state.set_value(!value, &arg.first_argument, argument_size);
    }

    fn neg(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "neg", arg);
        panic!("Not implemented");
    }

    fn ret(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "ret");
        panic!("Not implemented");
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
        let mut from = machine_state.get_value(&InstructionArgument::Register {register: Register::RSI}, ArgumentSize::Bit64);
        let mut to = machine_state.get_value(&InstructionArgument::Register {register: Register::RDI}, ArgumentSize::Bit64);
        // TODO: do not hardcode to 8byte operand
        if repeat {
            println!("{:<6}", "rep movs %ds:(%rsi),%es:(%rdi)");
            let mut length = machine_state.get_value(&InstructionArgument::Register {register: Register::RCX}, ArgumentSize::Bit64);
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
            InstructionArgument::Register {..} => {
                machine_state.rip = value
            },
            InstructionArgument::Immediate {..} => {
                machine_state.rip += value
            },
            InstructionArgument::EffectiveAddress {..} => {
                panic!("Unsupported argument for jmp");
            },
        }
    }

    fn jge(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jge", arg);
        //self.jmp(machine_state, arg);
        println!("WARNING: jge not implemented");
    }
}
