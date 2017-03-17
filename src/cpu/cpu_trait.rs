use instruction_set::InstructionArguments;
use machine_state::MachineState;

pub trait CPU {
    fn push(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn pop(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn mov(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn add(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn or(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn adc(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn sbb(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn and(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn sub(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn xor(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn cmp(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn call(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn lea(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn test(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn cmov(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn sar(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn inc(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn dec(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn div(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn idiv(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn mul(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn imul(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn not(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn neg(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn ret(&mut self, machine_state: &mut MachineState);

    fn leave(&mut self, machine_state: &mut MachineState);

    fn popf(&mut self, machine_state: &mut MachineState);

    fn std(&mut self, machine_state: &mut MachineState);

    fn cld(&mut self, machine_state: &mut MachineState);

    fn movs(&mut self, machine_state: &mut MachineState, repeat: bool);

    fn jmp(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn jge(&mut self, machine_state: &mut MachineState, arg: InstructionArguments);

    fn arithmetic(&mut self, machine_state: &mut MachineState, arg: InstructionArguments) {
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

    fn register_operation(&mut self, machine_state: &mut MachineState, arg: InstructionArguments) {
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

    fn compare_mul_operation(&mut self, machine_state: &mut MachineState, arg: InstructionArguments) {
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
