use instruction_set::InstructionArgument;
use cpu::cpu_trait::CPU;
use machine_state::MachineState;
use instruction_set::ArgumentSize;
use utils::{convert_i32_to_u8vec, convert_i64_to_u8vec};

pub struct EmulationCPU {}

impl CPU for EmulationCPU {
    fn push(&mut self, machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "push", arg);
        let argument_size = machine_state.first_argument_size(&arg);
        match argument_size {
            ArgumentSize::Bit32 => {
                let first_arg = machine_state.first_argument_i32(&arg);
                machine_state.stack_push(convert_i32_to_u8vec(first_arg));
            }
            ArgumentSize::Bit64 => {
                let first_arg = machine_state.first_argument_i64(&arg);
                machine_state.stack_push(convert_i64_to_u8vec(first_arg));
            }
            _ => panic!("Unsupported push argument size"),
        }
    }

    fn pop(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "pop", arg);
        panic!("Not implemented");
    }

    fn mov(&mut self, machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "mov", arg);
        arg.assert_two_arguments();
        let value = machine_state.first_argument_i64(&arg);
        machine_state.set_value(value, &arg);
    }

    fn add(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "add", arg);
        panic!("Not implemented");
    }

    fn or(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "or", arg);
        panic!("Not implemented");
    }

    fn adc(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "adc", arg);
        panic!("Not implemented");
    }

    fn sbb(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "sbb", arg);
        panic!("Not implemented");
    }

    fn and(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "and", arg);
        panic!("Not implemented");
    }

    fn sub(&mut self, machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "sub", arg);
        arg.assert_two_arguments();
        let value1 = machine_state.first_argument_i64(&arg);
        let value2 = machine_state.second_argument_i64(&arg);
        machine_state.set_value(value1 - value2, &arg);
    }

    fn xor(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "xor", arg);
        panic!("Not implemented");
    }

    fn cmp(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "cmp", arg);
        panic!("Not implemented");
    }

    fn call(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "call", arg);
        panic!("Not implemented");
    }

    fn lea(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "lea", arg);
        panic!("Not implemented");
    }

    fn test(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "test", arg);
        panic!("Not implemented");
    }

    fn cmov(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "cmov", arg);
        panic!("Not implemented");
    }

    fn sar(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "sar", arg);
        panic!("Not implemented");
    }

    fn inc(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "inc", arg);
        panic!("Not implemented");
    }

    fn dec(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "dec", arg);
        panic!("Not implemented");
    }

    fn div(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "div", arg);
        panic!("Not implemented");
    }

    fn idiv(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "idiv", arg);
        panic!("Not implemented");
    }

    fn mul(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "mul", arg);
        panic!("Not implemented");
    }

    fn imul(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "imul", arg);
        panic!("Not implemented");
    }

    fn not(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "not", arg);
        panic!("Not implemented");
    }

    fn neg(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "neg", arg);
        panic!("Not implemented");
    }

    fn ret(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "ret");
        panic!("Not implemented");
    }

    fn leave(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "leave");
        panic!("Not implemented");
    }

    fn popf(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "popf");
        panic!("Not implemented");
    }

    fn std(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "std");
        panic!("Not implemented");
    }
    fn cld(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "cld");
        panic!("Not implemented");
    }

    fn movs(&mut self, _machine_state: &mut MachineState, repeat: bool) {
        if repeat {
            println!("{:<6}", "rep movs %ds:(%rsi),%es:(%rdi)");
        } else {
            println!("{:<6}", "movs %ds:(%rsi),%es:(%rdi)");
        }
        panic!("Not implemented");
    }

    fn jmp(&mut self, machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "jmp", arg);
        match arg {
            InstructionArgument::Immediate32 { immediate } => {
                machine_state.rip += immediate as usize
            }
            _ => panic!("JMP: Unsupported argument."),
        }
    }

    fn jge(&mut self, _machine_state: &mut MachineState, arg: InstructionArgument) {
        println!("{:<6} {}", "jge", arg);
        panic!("Not implemented");
    }
}
