use instruction_set::InstructionArguments;
use machine_state::MachineState;

pub trait CPU {
    fn push(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn pop(&self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn mov(&self, machine_state: &mut MachineState, arg: InstructionArguments);

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

    fn cmov(&self, machine_state: &mut MachineState, arg: InstructionArguments);

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

    fn jge(&self, machine_state: &mut MachineState, arg: InstructionArguments);

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
}
