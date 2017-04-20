use instruction_set::{InstructionArgument, InstructionArguments};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;

pub struct PrintCPU {}

impl CPU for PrintCPU {
    fn push(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "push", arg);
    }

    fn pop(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "pop", arg);
    }

    fn mov(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "mov", arg);
    }

    fn movsxd(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "movsxd", arg);
    }

    fn movzbl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "movzbl", arg);
    }

    fn add(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "add", arg);
    }

    fn or(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "or", arg);
    }

    fn adc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "adc", arg);
    }

    fn sbb(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sbb", arg);
    }

    fn and(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "and", arg);
    }

    fn sub(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sub", arg);
    }

    fn xor(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "xor", arg);
    }

    fn cmp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmp", arg);
    }

    fn call(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "call", arg);
    }

    fn lea(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "lea", arg);
    }

    fn test(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "test", arg);
    }

    fn cmovs(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmovs", arg);
    }

    fn cmovz(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "cmovz", arg);
    }

    fn rol(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "rol", arg);
    }

    fn ror(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "ror", arg);
    }

    fn rcl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "rcl", arg);
    }

    fn rcr(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "rcr", arg);
    }

    fn shl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "shl", arg);
    }

    fn shr(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "shr", arg);
    }

    fn sar(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "sar", arg);
    }

    fn inc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "inc", arg);
    }

    fn dec(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "dec", arg);
    }

    fn div(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "div", arg);
    }

    fn idiv(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "idiv", arg);
    }

    fn mul(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "mul", arg);
    }

    fn imul(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "imul", arg);
    }

    fn not(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "not", arg);
    }

    fn neg(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "neg", arg);
    }

    fn ret(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "ret");
    }

    fn leave(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "leave");
    }

    fn popf(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "popf");
    }

    fn std(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "std");
    }
    fn cld(&self, _machine_state: &mut MachineState) {
        println!("{:<6}", "cld");
    }

    fn movs(&self, _machine_state: &mut MachineState, repeat: bool) {
        if repeat {
            println!("{:<6}", "rep movs %ds:(%rsi),%es:(%rdi)");
        } else {
            println!("{:<6}", "movs %ds:(%rsi),%es:(%rdi)");
        }
    }

    fn stos(&self, _machine_state: &mut MachineState, repeat: bool) {
        if repeat {
            println!("{:<6}", "rep stos %ds:(%rsi),%es:(%rdi)");
        } else {
            println!("{:<6}", "stos %ds:(%rsi),%es:(%rdi)");
        }
    }

    fn jmp(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jmp", arg);
        arg.assert_one_argument();
        match arg.first_argument {
            InstructionArgument::Immediate { immediate } => machine_state.rip += immediate,
            _ => panic!("JMP: Unsupported argument."),
        }
    }

    fn jo(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jo", arg);
    }

    fn jno(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jno", arg);
    }

    fn jc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jc", arg);
    }

    fn jnc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jnc", arg);
    }

    fn jz(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jz", arg);
    }

    fn jnz(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jnz", arg);
    }

    fn jbe(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jbe", arg);
    }

    fn ja(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "ja", arg);
    }

    fn js(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "js", arg);
    }

    fn jns(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jns", arg);
    }

    fn jp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jp", arg);
    }

    fn jnp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jnp", arg);
    }

    fn jl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jl", arg);
    }

    fn jge(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jge", arg);
    }

    fn jle(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jle", arg);
    }

    fn jg(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "jg", arg);
    }
}
