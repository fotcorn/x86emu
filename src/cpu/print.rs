use instruction_set::{InstructionArgument, InstructionArguments};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;

pub struct PrintCPU {}

impl CPU for PrintCPU {
    fn push(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "push", arg);
    }

    fn pop(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "pop", arg);
    }

    fn mov(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "mov", arg);
    }

    fn add(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "add", arg);
    }

    fn or(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "or", arg);
    }

    fn adc(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "adc", arg);
    }

    fn sbb(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sbb", arg);
    }

    fn and(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "and", arg);
    }

    fn sub(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sub", arg);
    }

    fn xor(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "xor", arg);
    }

    fn cmp(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmp", arg);
    }

    fn call(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "call", arg);
    }

    fn lea(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "lea", arg);
    }

    fn test(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "test", arg);
    }

    fn cmov(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmov", arg);
    }

    fn sar(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sar", arg);
    }

    fn inc(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "inc", arg);
    }

    fn dec(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "dec", arg);
    }

    fn div(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "div", arg);
    }

    fn idiv(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "idiv", arg);
    }

    fn mul(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "mul", arg);
    }

    fn imul(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "imul", arg);
    }

    fn not(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "not", arg);
    }

    fn neg(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "neg", arg);
    }

    fn ret(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "ret");
    }

    fn leave(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "leave");
    }

    fn popf(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "popf");
    }

    fn std(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "std");
    }
    fn cld(&mut self, _machine_state: &mut MachineState) {
        println!("{:<6}", "cld");
    }

    fn movs(&mut self, _machine_state: &mut MachineState, repeat: bool) {
        if repeat {
            println!("{:<6}", "rep movs %ds:(%rsi),%es:(%rdi)");
        } else {
            println!("{:<6}", "movs %ds:(%rsi),%es:(%rdi)");
        }
    }

    fn jmp(&mut self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jmp", arg);
        arg.assert_one_argument();
        match arg.first_argument {
            InstructionArgument::Immediate { immediate } => {
                machine_state.rip += immediate as usize
            }
            _ => panic!("JMP: Unsupported argument."),
        }
    }

    fn jge(&mut self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jge", arg);
    }
}
