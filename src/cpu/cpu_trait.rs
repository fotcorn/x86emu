use instruction_set::InstructionArgument;
use machine_state::MachineState;

pub trait CPU {
    fn push(&mut self, machine_state: &mut MachineState, arg: InstructionArgument);

    fn pop(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn mov(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn add(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn or(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn adc(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn sbb(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn and(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn sub(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn xor(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn cmp(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn call(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn lea(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn test(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn cmov(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn sar(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn inc(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn dec(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn div(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn idiv(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn mul(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn imul(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn not(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn neg(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn ret(&mut self, machine_state: &mut MachineState);

    fn leave(&mut self, machine_state: &mut MachineState);

    fn popf(&mut self, machine_state: &mut MachineState);

    fn std(&mut self, machine_state: &mut MachineState);

    fn cld(&mut self, machine_state: &mut MachineState);

    fn movs(&mut self, machine_state: &mut MachineState, repeat: bool);

    fn jmp(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn jge(&mut self, machine_state: &mut MachineState, arg:InstructionArgument);

    fn arithmetic(&mut self, machine_state: &mut MachineState, arg:InstructionArgument) {
        let opcode = match arg {
            InstructionArgument::Immediate8BitRegister { opcode, .. } => opcode,
            InstructionArgument::Immediate32BitRegister { opcode, .. } => opcode,
            _ => panic!("Unsupported argument type for arithmetic"),
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

    fn register_operation(&mut self, machine_state: &mut MachineState, arg:InstructionArgument) {
        let opcode = match arg {
            InstructionArgument::OneRegister { opcode, .. } => opcode,
            _ => panic!("Unsupported argument type for register operation"),
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

    fn compare_mul_operation(&mut self, machine_state: &mut MachineState, arg:InstructionArgument) {
        let opcode = match arg {
            InstructionArgument::OneRegister { opcode, .. } => opcode,
            _ => panic!("Unsupported argument type for register operation"),
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
