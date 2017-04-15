use std::collections::hash_map::{HashMap, Entry};
use std::fmt;
use instruction_set::{Flags, ArgumentSize};

const PAGE_SIZE: u64 = 4096;

pub struct MachineState {
    pub rip: i64,

    pub rax: i64,
    pub rbx: i64,
    pub rcx: i64,
    pub rdx: i64,
    pub rsp: i64,
    pub rbp: i64,
    pub rsi: i64,
    pub rdi: i64,

    pub r8: i64,
    pub r9: i64,
    pub r10: i64,
    pub r11: i64,
    pub r12: i64,
    pub r13: i64,
    pub r14: i64,
    pub r15: i64,

    pub rflags: i64,

    memory: HashMap<u64, Vec<u8>>,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            rip: 0,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsp: 0,
            rbp: 0,
            rsi: 0,
            rdi: 0,

            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,

            rflags: 0,
            memory: HashMap::new(),
        }
    }

    fn get_page(&mut self, cell: u64) -> &mut Vec<u8> {
        match self.memory.entry(cell) {
            Entry::Occupied(entry) => &mut *entry.into_mut(),
            Entry::Vacant(entry) => {
                let page = vec![0; PAGE_SIZE as usize];
                &mut *entry.insert(page)
            }
        }
    }

    pub fn mem_read_byte(&mut self, address: u64) -> u8 {
        let page_number = address / PAGE_SIZE;
        let page = self.get_page(page_number);
        let page_offset = address % PAGE_SIZE;
        page[page_offset as usize]
    }

    pub fn mem_read(&mut self, address: u64, length: u64) -> Vec<u8> {
        let mut page_number = address / PAGE_SIZE;
        let mut page_offset = address % PAGE_SIZE;
        let mut data_offset = 0;
        let mut data = Vec::new();
        loop {
            let page = self.get_page(page_number);

            loop {
                if data_offset >= length {
                    return data;
                }
                if page_offset >= PAGE_SIZE {
                    page_number += 1;
                    page_offset = 0;
                    break;
                }

                data.push(page[page_offset as usize]);

                data_offset += 1;
                page_offset += 1;
            }
        }
    }

    pub fn mem_write(&mut self, address: u64, data: &[u8]) {
        let mut page_number = address / PAGE_SIZE;
        let mut page_offset = address % PAGE_SIZE;
        let mut data_offset = 0;
        loop {
            let mut page = self.get_page(page_number);

            loop {
                if data_offset >= data.len() {
                    return;
                }
                if page_offset >= PAGE_SIZE {
                    page_number += 1;
                    page_offset = 0;
                    break;
                }

                page[page_offset as usize] = data[data_offset];

                data_offset += 1;
                page_offset += 1;
            }
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        let f = flag as i64;
        self.rflags & f == f
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            self.rflags |= flag as i64;
        } else {
            self.rflags &= flag as i64;
        }
    }

    pub fn compute_flags(&mut self, result: i64, argument_size: ArgumentSize) {
        self.set_flag(Flags::Zero, result == 0);
        let sign = match argument_size {
            ArgumentSize::Bit8 => (result as u64) & 0x80 != 0,
            ArgumentSize::Bit16 => (result as u64) & 0x8000 != 0,
            ArgumentSize::Bit32 => (result as u64) & 0x80000000 != 0,
            ArgumentSize::Bit64 => (result as u64) & 0x8000000000000000 != 0,
        };
        self.set_flag(Flags::Sign, sign);
    }
}

impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "rax: {:20x} rbx: {:20x} rcx: {:20x}\nrdx: {:20x} rsp: {:20x} rbp: {:20x}\nrsi: \
                {:20x} rdi: {:20x} rip: {:20x}\n",
               self.rax,
               self.rbx,
               self.rcx,
               self.rdx,
               self.rsp,
               self.rbp,
               self.rsi,
               self.rdi,
               self.rip)
    }
}
