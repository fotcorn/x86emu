use instruction_set::{InstructionArgument, InstructionArguments};
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
        machine_state.stack_push(vector);
    }

    fn pop(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "pop", arg);
        panic!("Not implemented");
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

    fn and(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "and", arg);
        panic!("Not implemented");
    }

    fn sub(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sub", arg);
        arg.assert_two_arguments();
        let argument_size = arg.size();
        let second_argument = arg.second_argument.unwrap();
        let value1 = machine_state.get_value(&arg.first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        machine_state.set_value(value1 - value2, &second_argument, argument_size);
    }

    fn xor(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "xor", arg);
        panic!("Not implemented");
    }

    fn cmp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmp", arg);
        panic!("Not implemented");
    }

    fn call(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "call", arg);
        panic!("Not implemented");
    }

    fn lea(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "lea", arg);
        panic!("Not implemented");
    }

    fn test(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "test", arg);
        panic!("Not implemented");
    }

    fn cmov(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmov", arg);
        panic!("Not implemented");
    }

    fn sar(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sar", arg);
        panic!("Not implemented");
    }

    fn inc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "inc", arg);
        panic!("Not implemented");
    }

    fn dec(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "dec", arg);
        panic!("Not implemented");
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

    fn not(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "not", arg);
        panic!("Not implemented");
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

    fn popf(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "popf");
        panic!("Not implemented");
    }

    fn std(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "std");
        panic!("Not implemented");
    }
    fn cld(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "cld");
        panic!("Not implemented");
    }

    fn movs(&self, _machine_state: &mut MachineState, repeat: bool) {
        if repeat {
            println!("{:<6}", "rep movs %ds:(%rsi),%es:(%rdi)");
        } else {
            println!("{:<6}", "movs %ds:(%rsi),%es:(%rdi)");
        }
        panic!("Not implemented");
    }

    fn jmp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jmp", arg);
        arg.assert_one_argument();
        match arg.first_argument {
            InstructionArgument::Immediate { immediate } => machine_state.rip += immediate as usize,
            _ => panic!("JMP: Unsupported argument."),
        }
    }

    fn jge(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jge", arg);
        panic!("Not implemented");
    }
}
