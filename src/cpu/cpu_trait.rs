use instruction_set::InstructionArguments;
use machine_state::MachineState;

pub trait CPU {
    fn push(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn pop(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn mov(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn movsx(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn movzx(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn add(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn or(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn adc(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn sbb(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn and(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn sub(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn xor(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn cmp(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn call(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn lea(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn test(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn cmovs(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn cmove(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    // rotate
    fn rol(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn ror(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn rcl(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn rcr(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    // shift
    fn shl(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn shr(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn sar(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn inc(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn dec(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn div(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn idiv(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn mul(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn imul(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn not(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn neg(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn ret(&self, machine_state: &mut MachineState);

    fn leave(&self, machine_state: &mut MachineState);

    fn popf(&self, machine_state: &mut MachineState);

    fn std(&self, machine_state: &mut MachineState);

    fn cld(&self, machine_state: &mut MachineState);

    fn movs(&self, machine_state: &mut MachineState, repeat: bool);

    fn stos(&self, machine_state: &mut MachineState, repeat: bool);

    fn jmp(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jo(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jno(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jc(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jnc(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jz(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jnz(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jbe(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn ja(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn js(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jns(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jp(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jnp(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jl(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jge(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jle(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jg(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn sete(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn out(&self, machine_state: &mut MachineState);

    fn arithmetic(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        let opcode = match arg.opcode {
            Some(opcode) => opcode,
            None => panic!("Unsupported argument type for arithmetic"),
        };
        match opcode {
            0 => self.add(machine_state, arg),
            1 => self.or(machine_state, arg),
            2 => self.adc(machine_state, arg),
            3 => self.sbb(machine_state, arg),
            4 => self.and(machine_state, arg),
            5 => self.sub(machine_state, arg),
            6 => self.xor(machine_state, arg),
            7 => self.cmp(machine_state, arg),
            _ => unreachable!(),
        }
    }

    fn register_operation(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        let opcode = match arg.opcode {
            Some(opcode) => opcode,
            None => panic!("Unsupported argument type for register_operation"),
        };
        match opcode {
            0 => self.inc(machine_state, arg),
            1 => self.dec(machine_state, arg),
            2 => self.call(machine_state, arg),
            3 => self.call(machine_state, arg), // far call
            4 => self.jmp(machine_state, arg),
            5 => self.jmp(machine_state, arg), // far jmp
            6 => self.push(machine_state, arg),
            _ => unreachable!(),
        }
    }

    fn compare_mul_operation(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        let opcode = match arg.opcode {
            Some(opcode) => opcode,
            None => panic!("Unsupported argument type for compare_mul_operation"),
        };
        match opcode {
            0 => self.test(machine_state, arg),
            1 => self.test(machine_state, arg),
            2 => self.not(machine_state, arg),
            3 => self.neg(machine_state, arg),
            4 => self.mul(machine_state, arg),
            5 => self.imul(machine_state, arg),
            6 => self.div(machine_state, arg),
            7 => self.idiv(machine_state, arg),
            _ => unreachable!(),
        }
    }

    fn shift_rotate(&self, machine_state: &mut MachineState, arg: InstructionArguments) {
        let opcode = match arg.opcode {
            Some(opcode) => opcode,
            None => panic!("Unsupported argument type for shift_rotate"),
        };
        match opcode {
            0 => self.rol(machine_state, arg),
            1 => self.ror(machine_state, arg),
            2 => self.rcl(machine_state, arg),
            3 => self.rcr(machine_state, arg),
            4 => self.shl(machine_state, arg),
            5 => self.shr(machine_state, arg),
            6 => self.shl(machine_state, arg), // sal and shl are the same
            7 => self.sar(machine_state, arg),
            _ => unreachable!(),
        }
    }
}
