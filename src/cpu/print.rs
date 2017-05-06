use instruction_set::{InstructionArgument, InstructionArguments, print_instruction};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;

pub struct PrintCPU {}

impl CPU for PrintCPU {
    fn push(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("push", &arg);
    }

    fn pop(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("pop", &arg);
    }

    fn mov(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("mov", &arg);
    }

    fn movsx(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "movsx", arg);
    }

    fn movzx(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        println!("{:<6} {}", "movzx", arg);
    }

    fn add(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("add", &arg);
    }

    fn or(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("or", &arg);
    }

    fn adc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("adc", &arg);
    }

    fn sbb(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("sbb", &arg);
    }

    fn and(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("and", &arg);
    }

    fn sub(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("sub", &arg);
    }

    fn xor(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("xor", &arg);
    }

    fn cmp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmp", &arg);
    }

    fn call(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("call", &arg);
    }

    fn lea(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("lea", &arg);
    }

    fn test(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("test", &arg);
    }

    fn cmovo(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovo", &arg);
    }

    fn cmovno(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovno", &arg);
    }

    fn cmovb(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovb", &arg);
    }

    fn cmovae(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovae", &arg);
    }

    fn cmove(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmove", &arg);
    }

    fn cmovne(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovne", &arg);
    }

    fn cmovbe(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovbe", &arg);
    }

    fn cmova(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmova", &arg);
    }

    fn cmovs(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovs", &arg);
    }

    fn cmovns(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovns", &arg);
    }

    fn cmovp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovp", &arg);
    }

    fn cmovnp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovnp", &arg);
    }

    fn cmovl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovl", &arg);
    }

    fn cmovge(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovge", &arg);
    }

    fn cmovle(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovle", &arg);
    }

    fn cmovg(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("cmovg", &arg);
    }

    fn rol(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("rol", &arg);
    }

    fn ror(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("ror", &arg);
    }

    fn rcl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("rcl", &arg);
    }

    fn rcr(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("rcr", &arg);
    }

    fn shl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("shl", &arg);
    }

    fn shr(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("shr", &arg);
    }

    fn sar(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("sar", &arg);
    }

    fn inc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("inc", &arg);
    }

    fn dec(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("dec", &arg);
    }

    fn div(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("div", &arg);
    }

    fn idiv(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("idiv", &arg);
    }

    fn mul(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("mul", &arg);
    }

    fn imul(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("imul", &arg);
    }

    fn not(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("not", &arg);
    }

    fn neg(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("neg", &arg);
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
        print_instruction("jmp", &arg);
        arg.assert_one_argument();
        match arg.first_argument {
            InstructionArgument::Immediate { immediate } => machine_state.rip += immediate,
            _ => panic!("JMP: Unsupported argument."),
        }
    }

    fn jo(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jo", &arg);
    }

    fn jno(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jno", &arg);
    }

    fn jc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jc", &arg);
    }

    fn jnc(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jnc", &arg);
    }

    fn jz(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jz", &arg);
    }

    fn jnz(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jnz", &arg);
    }

    fn jbe(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jbe", &arg);
    }

    fn ja(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("ja", &arg);
    }

    fn js(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("js", &arg);
    }

    fn jns(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jns", &arg);
    }

    fn jp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jp", &arg);
    }

    fn jnp(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jnp", &arg);
    }

    fn jl(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jl", &arg);
    }

    fn jge(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jge", &arg);
    }

    fn jle(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jle", &arg);
    }

    fn jg(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("jg", &arg);
    }

    fn sete(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {
        print_instruction("sete", &arg);
    }

    fn out(&self, _machine_state: &mut MachineState) {
        println!("{:<6} %al,(%dx)", "out");
    }
}
