use instruction_set::InstructionArgument;

use cpu::CPU;

pub enum ArgumentSize {
    Bit64,
    Bit32,
    Bit16,
}


impl CPU {
    pub fn first_argument_size(&self, arg: &InstructionArgument) -> ArgumentSize {
        ArgumentSize::Bit64
    }

    pub fn second_argument_size(&self, arg: InstructionArgument) -> ArgumentSize {
        ArgumentSize::Bit64
    }

    pub fn first_argument_i8(&self, arg: InstructionArgument) -> i8 {
        0
    }

    pub fn first_argument_i16(&self, arg: InstructionArgument) -> i16 {
        0
    }

    pub fn first_argument_i32(&self, arg: InstructionArgument) -> i32 {
        0
    }

    pub fn first_argument_i64(&self, arg: InstructionArgument) -> i64 {
        0
    }


    pub fn second_argument_i8(&self, arg: InstructionArgument) -> i8 {
        0
    }

    pub fn second_argument_i16(&self, arg: InstructionArgument) -> i16 {
        0
    }

    pub fn second_argument_i32(&self, arg: InstructionArgument) -> i32 {
        0
    }

    pub fn second_argument_i64(&self, arg: InstructionArgument) -> i64 {
        0
    }
}

pub fn convert_i8_to_u8vec(value: i32) -> Vec<u8> {
    vec![]
}

pub fn convert_i16_to_u8vec(value: i32) -> Vec<u8> {
    vec![]
}

pub fn convert_i32_to_u8vec(value: i32) -> Vec<u8> {
    vec![]
}

pub fn convert_i64_to_u8vec(value: i64) -> Vec<u8> {
    vec![]
}
