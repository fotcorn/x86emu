use instruction_set::{InstructionArgument, InstructionArguments, print_instr_arg};
use cpu::cpu_trait::CPU;
use machine_state::MachineState;

pub struct PrintCPU {}

impl CPU for PrintCPU {
    fn push(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("push", &arg);
    }

    fn pop(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("pop", &arg);
    }

    fn mov(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("mov", &arg);
    }

    fn movsx(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        println!("{:<6} {}", "movsx", arg);
    }

    fn movzx(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        println!("{:<6} {}", "movzx", arg);
    }

    fn add(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("add", &arg);
    }

    fn or(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("or", &arg);
    }

    fn adc(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("adc", &arg);
    }

    fn sbb(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sbb", &arg);
    }

    fn and(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("and", &arg);
    }

    fn sub(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sub", &arg);
    }

    fn xor(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("xor", &arg);
    }

    fn cmp(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmp", &arg);
    }

    fn call(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("call", &arg);
    }

    fn lea(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("lea", &arg);
    }

    fn test(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("test", &arg);
    }

    fn cmovo(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovo", &arg);
    }

    fn cmovno(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovno", &arg);
    }

    fn cmovb(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovb", &arg);
    }

    fn cmovae(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovae", &arg);
    }

    fn cmove(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmove", &arg);
    }

    fn cmovne(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovne", &arg);
    }

    fn cmovbe(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovbe", &arg);
    }

    fn cmova(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmova", &arg);
    }

    fn cmovs(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovs", &arg);
    }

    fn cmovns(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovns", &arg);
    }

    fn cmovp(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovp", &arg);
    }

    fn cmovnp(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovnp", &arg);
    }

    fn cmovl(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovl", &arg);
    }

    fn cmovge(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovge", &arg);
    }

    fn cmovle(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovle", &arg);
    }

    fn cmovg(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovg", &arg);
    }

    fn rol(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("rol", &arg);
    }

    fn ror(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("ror", &arg);
    }

    fn rcl(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("rcl", &arg);
    }

    fn rcr(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("rcr", &arg);
    }

    fn shl(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("shl", &arg);
    }

    fn shr(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("shr", &arg);
    }

    fn sar(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sar", &arg);
    }

    fn inc(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("inc", &arg);
    }

    fn dec(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("dec", &arg);
    }

    fn div(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("div", &arg);
    }

    fn idiv(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("idiv", &arg);
    }

    fn mul(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("mul", &arg);
    }

    fn imul(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("imul", &arg);
    }

    fn not(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("not", &arg);
    }

    fn neg(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("neg", &arg);
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

    fn jmp(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jmp", &arg);
        arg.assert_one_argument();
        match arg.first_argument {
            InstructionArgument::Immediate { immediate } => machine_state.rip += immediate,
            _ => panic!("JMP: Unsupported argument."),
        }
    }

    fn jo(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jo", &arg);
    }

    fn jno(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jno", &arg);
    }

    fn jb(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jb", &arg);
    }

    fn jae(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jae", &arg);
    }

    fn je(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("je", &arg);
    }

    fn jne(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jne", &arg);
    }

    fn jbe(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jbe", &arg);
    }

    fn ja(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("ja", &arg);
    }

    fn js(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("js", &arg);
    }

    fn jns(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jns", &arg);
    }

    fn jp(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jp", &arg);
    }

    fn jnp(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jnp", &arg);
    }

    fn jl(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jl", &arg);
    }

    fn jge(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jge", &arg);
    }

    fn jle(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jle", &arg);
    }

    fn jg(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jg", &arg);
    }

    fn sete(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sete", &arg);
    }

    fn out(&self, _machine_state: &mut MachineState) {
        println!("{:<6} %al,(%dx)", "out");
    }
}
