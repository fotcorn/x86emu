use std::collections::hash_map::{Entry};
use machine_state::MachineState;

use zero;

const PAGE_SIZE: u64 = 4096;

impl MachineState {
    fn get_page(&mut self, cell: u64) -> &mut Vec<u8> {
        match self.memory.entry(cell) {
            Entry::Occupied(entry) => &mut *entry.into_mut(),
            Entry::Vacant(entry) => {
                let page = vec![0; PAGE_SIZE as usize];
                &mut *entry.insert(page)
            }
        }
    }

    fn translate_virtual_to_physical_address(&mut self, address: u64) -> u64 {
        let cr3 = self.cr3 as u64;
        if cr3 == 0 {
            address
        } else {
            let page_address = address & 0b0000000000000000000000000000000000000000000000000000111111111111;
            let level4 = (address & 0b0000000000000000000000000000000000000000000111111111000000000000) >> 12;
            let level3 = (address & 0b0000000000000000000000000000000000111111111000000000000000000000) >> 21;
            let level2 = (address & 0b0000000000000000000000000111111111000000000000000000000000000000) >> 30;
            let level1 = (address & 0b0000000000000000111111111000000000000000000000000000000000000000) >> 39;

            //println!("level1: {:x}", level1);
            let entry = self.mem_read_phys(cr3 + level1 * 8, 8);
            let entry = *zero::read::<u64>(&entry) >> 12 << 12;
            //println!("entry 1: {:x}", entry);

            let entry = self.mem_read_phys(entry + level2 * 8, 8);
            let entry = *zero::read::<u64>(&entry) >> 12 << 12;
            //println!("entry 2: {:x}", entry);

            let entry = self.mem_read_phys(entry + level3 * 8, 8);
            let entry = *zero::read::<u64>(&entry) >> 12 << 12;
            //println!("entry 3: {:x}", entry);

            /*println!("level4: {:x}, {:x}", entry, level4 * 8);
            let entry = self.mem_read_phys(entry + level4 * 8, 8);
            let entry = *zero::read::<u64>(&entry) >> 12 << 12;
            println!("entry 4: {:x}", entry);*/

            
            //println!("address: {:x}", address);
            //println!("entry: {:x}", entry + page_address);

            entry + page_address
        }
    }

    pub fn mem_read_byte(&mut self, address: u64) -> u8 {
        let address = self.translate_virtual_to_physical_address(address);
        self.mem_read_byte_phys(address)
    }

    fn mem_read_byte_phys(&mut self, address: u64) -> u8 {
        let page_number = address / PAGE_SIZE;
        let page = self.get_page(page_number);
        let page_offset = address % PAGE_SIZE;
        page[page_offset as usize]
    }

    pub fn mem_read(&mut self, address: u64, length: u64) -> Vec<u8> {
        let address = self.translate_virtual_to_physical_address(address);
        self.mem_read_phys(address, length)
    }

    fn mem_read_phys(&mut self, address: u64, length: u64) -> Vec<u8> {
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
        let address = self.translate_virtual_to_physical_address(address);
        self.mem_write_phys(address, data)
    }

    fn mem_write_phys(&mut self, address: u64, data: &[u8]) {
        const MEMORY_OFFSET: u64 = 0xB8000;
        if address >= MEMORY_OFFSET && address <= (MEMORY_OFFSET + 80 * 25 * 2) && address % 2 == 0{
            println!("VIDEO: {}", data[0] as char);
        }

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
}
