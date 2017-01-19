use std::fs::File;
use std::io::Read;
use std::env;

extern crate zero;
use zero::read_str;

extern crate xmas_elf;
use xmas_elf::{ElfFile, program, sections};
use xmas_elf::symbol_table::Entry;

#[macro_use]
extern crate bitflags;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: cargo run <program>");
            return;
        }
    };

    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let elf_file = ElfFile::new(&buffer);

    // get address where to load the text section
    let load_address = get_load_address(&elf_file).expect("can not get load address");

    // get the binary code from the .text section
    let text_section = elf_file.find_section_by_name(".text").expect("text section not found.");
    let code = text_section.raw_data(&elf_file);
    let code_offset = text_section.offset();

    // get the virtual address of the main function
    let (main_symbol_address, main_size) = get_main_symbol_address(&elf_file);
    // get the offset of the main function
    let offset = main_symbol_address - code_offset - load_address;

    let main_code = &code[offset as usize .. (offset + main_size) as usize];

    disassemble(&main_code);
}

fn get_load_address(elf_file: &ElfFile) -> Option<u64> {
    for sect in elf_file.program_iter() {
        let t = sect.get_type().unwrap();
        match t {
            program::Type::Load if sect.flags() & program::FLAG_X == program::FLAG_X => {
                return Some(sect.virtual_addr());
            }
            _ => {}
        }
    }
    return None
}

fn get_main_symbol_address(elf_file: &ElfFile) -> (u64, u64) {
    let symbol_string_table = elf_file.find_section_by_name(".strtab").expect("strtab (String table) section not found, is this a stripped binary?");
    let symbol_string_table = symbol_string_table.raw_data(&elf_file);

    let symbol_table = elf_file.find_section_by_name(".symtab").expect("symtab (Symbol table) section not found");
    if let sections::SectionData::SymbolTable64(data) = symbol_table.get_data(&elf_file).unwrap() {
       let symbol = data.iter().find(|&symbol| read_str(&symbol_string_table[symbol.name() as usize..]) == "main").expect("main symbol not found");
       return (symbol.value(), symbol.size());
    } else {
        unreachable!();
    };
}

fn disassemble(code: &[u8]) {
    let mut instruction_pointer = 0;

    loop {
        let first_byte = code[instruction_pointer];

        let mut rex: Option<REX> = None;

        match first_byte {
            0xF0 | 0xF2 | 0xF3 => panic!("Lock and repeat prefixes/Bound prefix not supported"),
            0x2E | 0x3E | 0x36 | 0x26 | 0x64 | 0x65 => panic!("Segment override prefixes/branch hints not supported"),
            0x66 => panic!("Operand-size override prefix not supported"),
            0x67 => panic!("Address-size override prefix not supported"),
            0x40...0x4F => {  // 64bit REX prefix
                rex = Some(REX{ bits: first_byte });
                instruction_pointer += 1;
            },
            _ => ()
        }

        let first_byte = code[instruction_pointer];
        match first_byte {
            opcode @ 0x50...0x57 => cpu_push(InstructionArgument::OneRegister{ register: get_register(opcode - 0x50) }),
            0x89 => {
                instruction_pointer += 1;
                cpu_move(get_two_register_argument(rex, code[instruction_pointer]));
            },
            _ => panic!("end"),
        }

        instruction_pointer += 1;
        

    }
}


bitflags! {
    flags REX: u8 {
        const OPERAND_64_BIT = 0b00001000,
        const MOD_R_M_EXTENSION = 0b00000100,
        const SIB_EXTENSION = 0b00000010,
        const B = 0b00000001,
    }
}

bitflags! {
    flags Mod: u8 {
        const EFFECTIVE_ADDRESS = 0b00,
        const EFFECTIVE_ADDRESS_8BIT_DEPLACEMENT = 0b01,
        const EFFECTIVE_ADDRESS_32BIT_DEPLACEMENT = 0b10,
        const REGISTER = 0b11,
    }
}

#[derive(Debug)]
enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSP,
    RBP,
    RSI,
    RDI,
}

#[derive(Debug)]
enum InstructionArgument {
    OneRegister { register: Register },
    TwoRegister {register1: Register, register2: Register }
}

fn get_two_register_argument(rex: Option<REX>, modrm: u8) -> InstructionArgument {
    let mode = Mod{ bits: modrm >> 6};
    match mode {
        EFFECTIVE_ADDRESS => panic!("effective address not implemented"),
        EFFECTIVE_ADDRESS_8BIT_DEPLACEMENT => panic!("effective address with 8bit displacement not implemented"),
        EFFECTIVE_ADDRESS_32BIT_DEPLACEMENT => panic!("effective address not 32bit displacement not implemented"),
        REGISTER => {
            let register1 = get_register((modrm & 0b00111000) >> 3);
            let register2 = get_register(modrm & 0b00000111);
            InstructionArgument::TwoRegister{ register1: register1, register2: register2 }
        }
        _ => unreachable!(),
    }
}

fn get_register(num: u8) -> Register {
    match num {
        0 => Register::RAX,
        1 => Register::RBX,
        2 => Register::RCX,
        3 => Register::RDX,
        4 => Register::RSP,
        5 => Register::RBP,
        6 => Register::RSI,
        7 => Register::RDI,
        _ => panic!("Unknown instruction argument"),
    }
}

fn cpu_push(arg: InstructionArgument) {
    println!("PUSH {:?}", arg);
}

fn cpu_move(arg: InstructionArgument) {
    println!("MOVE {:?}", arg);
}
