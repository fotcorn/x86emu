use instruction_set::InstructionArgument;

pub fn push(arg: InstructionArgument) {
    println!("PUSH {:?}", arg);
}

pub fn mov(arg: InstructionArgument) {
    println!("MOV {:?}", arg);
}


pub fn add(arg: InstructionArgument) {
    println!("ADD {:?}", arg);
}

pub fn or(arg: InstructionArgument) {
    println!("OR {:?}", arg);
}

pub fn adc(arg: InstructionArgument) {
    println!("ADC {:?}", arg);
}

pub fn sbb(arg: InstructionArgument) {
    println!("SBB {:?}", arg);
}

pub fn and(arg: InstructionArgument) {
    println!("AND {:?}", arg);
}

pub fn sub(arg: InstructionArgument) {
    println!("SUB {:?}", arg);
}

pub fn xor(arg: InstructionArgument) {
    println!("XOR {:?}", arg);
}

pub fn cmp(arg: InstructionArgument) {
    println!("CMP {:?}", arg);
}


pub fn arithmetic(arg: InstructionArgument) {
    let opcode = match arg {
        InstructionArgument::Immediate8BitRegister { opcode, .. } => opcode,
        InstructionArgument::Immediate8BitRegister8BitDisplacement { opcode, .. } => opcode,
        InstructionArgument::Immediate32BitRegister8BitDisplacement { opcode, .. } => opcode,
        _ => panic!("Unsupported argument type for arithmetic"),
    };
    match opcode {
        0 => add(arg),
        1 => or(arg),
        2 => adc(arg),
        3 => sbb(arg),
        4 => and(arg),
        5 => sub(arg),
        6 => xor(arg),
        7 => cmp(arg),
        _ => unreachable!(),
    }
}