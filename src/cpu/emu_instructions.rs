use instruction_set::{InstructionArgument, InstructionArguments, Register, Flags};
use machine_state::MachineState;
use instruction_set::{ArgumentSize, get_register_size, print_instr, print_instr_arg, print_instr_arg_no_size};
use utils::{convert_i32_to_u8vec, convert_i64_to_u8vec};

pub struct EmulationCPU;

impl EmulationCPU {
    // implementations used by multiple instructions
    fn sub_impl(&self, machine_state: &mut MachineState, arg: &InstructionArguments, set: bool) {
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);

        let (result, carry, overflow) = match argument_size {
            ArgumentSize::Bit8 => {
                let (result, carry) = (value2 as u8).overflowing_sub(value1 as u8);
                let (_, overflow) = (value2 as i8).overflowing_sub(value1 as i8);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit16 => {
                let (result, carry) = (value2 as u16).overflowing_sub(value1 as u16);
                let (_, overflow) = (value2 as i16).overflowing_sub(value1 as i16);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit32 => {
                let (result, carry) = (value2 as u32).overflowing_sub(value1 as u32);
                let (_, overflow) = (value2 as i32).overflowing_sub(value1 as i32);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit64 => {
                let (result, carry) = (value2 as u64).overflowing_sub(value1 as u64);
                let (_, overflow) = (value2 as i64).overflowing_sub(value1 as i64);
                (result as i64, carry, overflow)
            }
        };
        machine_state.set_flag(Flags::Carry, carry);
        machine_state.set_flag(Flags::Overflow, overflow);
        machine_state.compute_flags(result, argument_size);
        if set {
            machine_state.set_value(result, &second_argument, argument_size);
        }
    }

    fn and_impl(&self, machine_state: &mut MachineState, arg: &InstructionArguments, set: bool) {

        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value1 & value2;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_flag(Flags::Carry, false);
        machine_state.set_flag(Flags::Overflow, false);
        if set {
            machine_state.set_value(result, &second_argument, argument_size);
        }
    }

    fn jmp_iml(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        let first_argument = arg.get_one_argument();
        let value = machine_state.get_value(&first_argument, arg.size());
        match *first_argument {
            InstructionArgument::Register { .. } => machine_state.rip = value,
            InstructionArgument::Immediate { .. } => machine_state.rip += value,
            InstructionArgument::EffectiveAddress { .. } => {
                panic!("Unsupported argument for jump");
            }
        }
    }

    fn mov_impl(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value = machine_state.get_value(&first_argument, argument_size);
        machine_state.set_value(value, second_argument, argument_size);
    }

    // different instructions with same opcode
    pub fn arithmetic(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
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

    pub fn register_operation(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
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

    pub fn compare_mul_operation(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
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

    pub fn shift_rotate(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
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

    // all other instructions
    pub fn push(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("push", &arg);
        let first_argument = arg.get_one_argument();
        let vector = match arg.size() {
            ArgumentSize::Bit32 => {
                convert_i32_to_u8vec(machine_state.get_value(&first_argument,
                                                             ArgumentSize::Bit32) as i32)
            }
            ArgumentSize::Bit64 => {
                convert_i64_to_u8vec(machine_state.get_value(&first_argument,
                                                             ArgumentSize::Bit64))
            }
            _ => panic!("Unsupported push value size"),
        };
        machine_state.stack_push(&vector);
    }

    pub fn pop(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("pop", &arg);
        let first_argument = arg.get_one_argument();
        let value = machine_state.stack_pop();
        machine_state.set_value(value, &first_argument, arg.size());
    }

    pub fn mov(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("mov", &arg);
        self.mov_impl(machine_state, arg);
    }

    pub fn movsx(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg_no_size("movsx", &arg);
        // normal mov already does the sign extension
        self.mov_impl(machine_state, arg);
    }

    pub fn movzx(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg_no_size("movzx", &arg);
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value = machine_state.get_value(&first_argument, argument_size);
        let first_argument_size = match *first_argument {
            InstructionArgument::Register {ref register} => {
                get_register_size(register)
            },
            InstructionArgument::EffectiveAddress {..} => {
                match arg.explicit_size {
                    Some(explicit_size) => explicit_size,
                    None => panic!("movzx instruction needs explicit size when using an effective address"),
                }
            }
            _ => panic!("Invalid parameter for mov")
        };

        let value = match first_argument_size {
            ArgumentSize::Bit8 => value as u8 as u64,
            ArgumentSize::Bit16 => value as u16 as u64,
            ArgumentSize::Bit32 => value as u32 as u64,
            ArgumentSize::Bit64 => value as u64 as u64,
        };

        // ArgumentSize::Bit64 is not used because target is always a register
        machine_state.set_value(value as i64, second_argument, ArgumentSize::Bit64);
    }

    pub fn add(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("add", &arg);
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);

        let (result, carry, overflow) = match argument_size {
            ArgumentSize::Bit8 => {
                let (result, carry) = (value2 as u8).overflowing_add(value1 as u8);
                let (_, overflow) = (value2 as i8).overflowing_add(value1 as i8);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit16 => {
                let (result, carry) = (value2 as u16).overflowing_add(value1 as u16);
                let (_, overflow) = (value2 as i16).overflowing_add(value1 as i16);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit32 => {
                let (result, carry) = (value2 as u32).overflowing_add(value1 as u32);
                let (_, overflow) = (value2 as i32).overflowing_add(value1 as i32);
                (result as i64, carry, overflow)
            }
            ArgumentSize::Bit64 => {
                let (result, carry) = (value2 as u64).overflowing_add(value1 as u64);
                let (_, overflow) = (value2 as i64).overflowing_add(value1 as i64);
                (result as i64, carry, overflow)
            }
        };
        machine_state.set_flag(Flags::Carry, carry);
        machine_state.set_flag(Flags::Overflow, overflow);

        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
    }

    pub fn or(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("or", &arg);
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value1 | value2;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
    }

    pub fn adc(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("adc", &arg);
        panic!("Not implemented");
    }

    pub fn sbb(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sbb", &arg);
        self.sub_impl(machine_state, arg, true);
        // TODO: SBB implemented without carry
    }

    pub fn and(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("and", &arg);
        self.and_impl(machine_state, arg, true);
    }

    pub fn sub(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sub", &arg);
        self.sub_impl(machine_state, arg, true);
    }

    pub fn xor(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("xor", &arg);
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value1 ^ value2;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
    }

    pub fn cmp(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmp", &arg);
        self.sub_impl(machine_state, arg, false);
    }

    pub fn call(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("call", &arg);
        let rip = convert_i64_to_u8vec(machine_state.rip);
        machine_state.stack_push(&rip);
        self.jmp_iml(machine_state, arg);
    }

    pub fn lea(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("lea", &arg);
        let (first_argument, second_argument) = arg.get_two_arguments();
        let argument_size = arg.size();
        match *first_argument {
            InstructionArgument::EffectiveAddress { .. } => {
                let value = machine_state.calculate_effective_address(&first_argument) as i64;
                match *second_argument {
                    InstructionArgument::Register { .. } => {
                        machine_state.set_value(value, &second_argument, argument_size)
                    }
                    _ => panic!("Unsupported lea argument"),
                }
            }
            _ => panic!("Unsupported lea argument"),
        }
    }

    pub fn test(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("test", &arg);
        // TODO:  test not fully implemented
        self.and_impl(machine_state, arg, false);
    }

    pub fn cmovo(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovo", &arg);
        if machine_state.get_flag(Flags::Overflow) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovno(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovno", &arg);
        if !machine_state.get_flag(Flags::Overflow) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovb(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovb", &arg);
        if machine_state.get_flag(Flags::Carry) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovae(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovae", &arg);
        if !machine_state.get_flag(Flags::Carry) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmove(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmove", &arg);
        if machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovne(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovne", &arg);
        if !machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovbe(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovbe", &arg);
        if machine_state.get_flag(Flags::Carry) || machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmova(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmova", &arg);
        if !machine_state.get_flag(Flags::Carry) && !machine_state.get_flag(Flags::Zero) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovs(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovs", &arg);
        if machine_state.get_flag(Flags::Sign) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovns(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovns", &arg);
        if !machine_state.get_flag(Flags::Sign) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovp(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovp", &arg);
        if machine_state.get_flag(Flags::Parity) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovnp(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovnp", &arg);
        if !machine_state.get_flag(Flags::Parity) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovl(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovl", &arg);
        if machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow){
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovge(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovge", &arg);
        if machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow){
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovle(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovle", &arg);
        if machine_state.get_flag(Flags::Zero) ||
                (machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow)) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn cmovg(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("cmovg", &arg);
        if !machine_state.get_flag(Flags::Zero) &&
                (machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow)) {
            self.mov_impl(machine_state, arg);
        }
    }

    pub fn rol(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("rol", &arg);
        panic!("Not implemented");
    }

    pub fn ror(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("rol", &arg);
        panic!("Not implemented");
    }

    pub fn rcl(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("rcl", &arg);
        panic!("Not implemented");
    }

    pub fn rcr(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("rcr", &arg);
        panic!("Not implemented");
    }

    pub fn shl(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("shl", &arg);
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 << value1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        // TODO:  shl does not set carry/overflow flag
    }

    pub fn shr(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("shr", &arg);
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size) as u64;
        let value2 = machine_state.get_value(&second_argument, argument_size) as u64;
        let result = (value2 >> value1) as i64;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        // TODO:  shr does not set carry/overflow flag
    }

    pub fn sar(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sar", &arg);
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 >> value1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &second_argument, argument_size);
        // TODO:  sar does not preserve highest byte; sets O/C flags
    }

    pub fn inc(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("inc", &arg);
        let first_argument = arg.get_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&first_argument, argument_size);
        let result = value + 1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &first_argument, argument_size);
    }

    pub fn dec(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("dec", &arg);
        let first_argument = arg.get_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&first_argument, argument_size);
        let result = value - 1;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &first_argument, argument_size);
    }

    pub fn div(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("div", &arg);
        panic!("Not implemented");
    }

    pub fn idiv(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("idiv", &arg);
        panic!("Not implemented");
    }

    pub fn mul(&self, _machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("mul", &arg);
        panic!("Not implemented");
    }

    pub fn imul(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("imul", &arg);
        // TODO: implement one argument version
        let argument_size = arg.size();
        let (first_argument, second_argument) = arg.get_two_arguments();
        let value1 = machine_state.get_value(&first_argument, argument_size);
        let value2 = machine_state.get_value(&second_argument, argument_size);
        let result = value2 * value1;
        machine_state.compute_flags(result, argument_size);
        match arg.third_argument {
            Some(ref third_argument) => {
                machine_state.set_value(result, third_argument, argument_size);
            },
            None => {
                machine_state.set_value(result, &second_argument, argument_size);
            }
        }
        // TODO:  imul does not set carry/overflow flag
    }

    pub fn not(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("not", &arg);
        let first_argument = arg.get_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&first_argument, argument_size);
        let result = !value;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &first_argument, argument_size);
    }

    pub fn neg(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("neg", &arg);
        let first_argument = arg.get_one_argument();
        let argument_size = arg.size();
        let value = machine_state.get_value(&first_argument, argument_size);
        let result = -value;
        machine_state.compute_flags(result, argument_size);
        machine_state.set_value(result, &first_argument, argument_size);
    }

    pub fn ret(&self, machine_state: &mut MachineState) {
        print_instr("ret");
        let value = machine_state.stack_pop();
        machine_state.rip = value;
    }

    pub fn leave(&self, machine_state: &mut MachineState) {
        print_instr("leave");
        let value = machine_state.get_register_value(&Register::RBP);
        machine_state.set_register_value(&Register::RSP, value);
        let value = machine_state.stack_pop();
        machine_state.set_register_value(&Register::RBP, value);
    }

    pub fn pushf(&self, machine_state: &mut MachineState) {
        let vector = convert_i64_to_u8vec(machine_state.rflags);
        machine_state.stack_push(&vector);
    }

    pub fn popf(&self, machine_state: &mut MachineState) {
        print_instr("popf");
        let value = machine_state.stack_pop();
        machine_state.rflags = value;
    }

    pub fn std(&self, machine_state: &mut MachineState) {
        print_instr("std");
        machine_state.set_flag(Flags::Direction, true);
    }

    pub fn cld(&self, machine_state: &mut MachineState) {
        print_instr("cld");
        machine_state.set_flag(Flags::Direction, false);
    }

    pub fn stos(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        let to =
            machine_state.get_value(&InstructionArgument::Register { register: Register::RDI },
                                    ArgumentSize::Bit64);
        if arg.repeat {
            let mut length =
                machine_state.get_value(&InstructionArgument::Register { register: Register::RCX },
                                        ArgumentSize::Bit64);
            length *= 8; // 8 bytes per repeat
            print_instr("rep stos %rax,%es:(%rdi)");
            if machine_state.get_flag(Flags::Direction) {
                panic!("stos NOOP");
            } else {
                // TODO: actually do something
                machine_state.set_register_value(&Register::RDI, to + length);
                machine_state.set_register_value(&Register::RCX, 0);
            }
        } else {
            print_instr("stos %ds:(%rsi),%es:(%rdi)");
        }
        // TODO:  stos: NOOP
    }


    pub fn movs(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        let mut from =
            machine_state.get_value(&InstructionArgument::Register { register: Register::RSI },
                                    ArgumentSize::Bit64);
        let mut to =
            machine_state.get_value(&InstructionArgument::Register { register: Register::RDI },
                                    ArgumentSize::Bit64);
        let bytes_per_mov = match arg.explicit_size.expect("movs need an explicit_size") {
            ArgumentSize::Bit64 => 8,
            ArgumentSize::Bit32 => 4,
            ArgumentSize::Bit16 => 2,
            ArgumentSize::Bit8 => 1,
        };
        if arg.repeat {
            print_instr("rep movs %ds:(%rsi),%es:(%rdi)");
            let mut length =
                machine_state.get_value(&InstructionArgument::Register { register: Register::RCX },
                                        ArgumentSize::Bit64);
            length *= bytes_per_mov;
            if machine_state.get_flag(Flags::Direction) {
                // TODO:  address calculation could be incorrect
                from -= length;
                to -= length;
                let data = machine_state.mem_read(from as u64, length as u64);
                machine_state.mem_write(to as u64, &data);
                machine_state.set_register_value(&Register::RSI, from);
                machine_state.set_register_value(&Register::RDI, to);
            } else {
                let data = machine_state.mem_read(from as u64, length as u64);
                machine_state.mem_write(to as u64, &data);
                // TODO:  rsi and rdi not set
                // TODO: set rsi, rdi registers
            }
            machine_state.set_register_value(&Register::RCX, 0);
        } else {
            print_instr("movs %ds:(%rsi),%es:(%rdi)");
            let data = machine_state.mem_read(from as u64, bytes_per_mov as u64);
            machine_state.mem_write(to as u64, &data);
        }
    }

    pub fn jmp(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jmp", &arg);
        self.jmp_iml(machine_state, arg);
    }

    pub fn jo(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jo", &arg);
        if machine_state.get_flag(Flags::Overflow) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jno(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jno", &arg);
        if !machine_state.get_flag(Flags::Overflow) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jb(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jb", &arg);
        if machine_state.get_flag(Flags::Carry) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jae(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jae", &arg);
        if !machine_state.get_flag(Flags::Carry) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn je(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("je", &arg);
        if machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jne(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jne", &arg);
        if !machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jbe(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jbe", &arg);
        // CF=1 OR ZF=1
        if machine_state.get_flag(Flags::Carry) || machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn ja(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("ja", &arg);
        // CF=0 AND ZF=0
        if !machine_state.get_flag(Flags::Carry) && !machine_state.get_flag(Flags::Zero) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn js(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("js", &arg);
        if machine_state.get_flag(Flags::Sign) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jns(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jns", &arg);
        if !machine_state.get_flag(Flags::Sign) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jp(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jp", &arg);
        if machine_state.get_flag(Flags::Parity) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jnp(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("jnp", &arg);
        if !machine_state.get_flag(Flags::Parity) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jl(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        // SF!=OF
        print_instr_arg("jl", &arg);
        if machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow){
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jge(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        // SF=OF
        print_instr_arg("jge", &arg);
        if machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow){
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jle(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        // (ZF=1) OR (SF!=OF)
        print_instr_arg("jle", &arg);
        if machine_state.get_flag(Flags::Zero) ||
                (machine_state.get_flag(Flags::Sign) != machine_state.get_flag(Flags::Overflow)) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn jg(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        // (ZF=0) AND (SF=OF)
        print_instr_arg("jg", &arg);
        if !machine_state.get_flag(Flags::Zero) &&
                (machine_state.get_flag(Flags::Sign) == machine_state.get_flag(Flags::Overflow)) {
            self.jmp_iml(machine_state, arg);
        }
    }

    pub fn sete(&self, machine_state: &mut MachineState, arg: &InstructionArguments) {
        print_instr_arg("sete", &arg);
        let first_argument = arg.get_one_argument();
        if machine_state.get_flag(Flags::Zero) {
            machine_state.set_value(1, &first_argument, ArgumentSize::Bit8);
        } else {
            machine_state.set_value(0, &first_argument, ArgumentSize::Bit8);
        }
    }

    pub fn out(&self, _machine_state: &mut MachineState) {
        print_instr("out   %al,(%dx)");
        // TODO:  out not implemented
    }

    pub fn cpuid(&self, machine_state: &mut MachineState) {
        print_instr("cpuid");
        let value = machine_state.get_register_value(&Register::RAX);
        match value {
            _ => panic!("CPUID: unsupported input: {}", value),
        }
    }
}
