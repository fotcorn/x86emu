use instruction_set::InstructionArgument;

pub fn push(arg: InstructionArgument) {
    println!("PUSH {:?}", arg);
}

pub fn mov(arg: InstructionArgument) {
    println!("MOV {:?}", arg);
}

/*
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
*/

pub fn arithmetic(arg: InstructionArgument) {
    println!("ARITHMETIC {:?}", arg);
}