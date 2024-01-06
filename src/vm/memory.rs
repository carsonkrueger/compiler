use crate::util::{
    endianness::{as_i32_be, i32_bytes_be},
    reportable::Reportable,
};
use std::fs::File;
use std::io::{self, Read};

const mem_capacity: usize = 102400;

pub struct Memory {
    bytes: [u8; mem_capacity],
    data_seg_start: usize,
    code_seg_start: usize,
    heap_start: usize,
    heap_size: usize,
    stack_size: usize,
    // code_seg_size: usize,
}

impl Memory {
    pub fn new(file_path: &String) -> Self {
        let mut file = File::open(file_path).expect("Could not open binary file");

        let mut mem = Self {
            bytes: [0; 102400],
            data_seg_start: 4,
            code_seg_start: 4,
            heap_start: file
                .metadata()
                .expect("Could not get metadata of binary file")
                .len() as usize,
            heap_size: 0,
            stack_size: 0,
        };

        file.read_exact(&mut mem.bytes);
        let first_bytes = [mem.bytes[0], mem.bytes[1], mem.bytes[2], mem.bytes[3]];
        let init_pc = as_i32_be(&first_bytes) as usize;
        mem.code_seg_start = init_pc;

        mem
    }
    pub fn capacity() -> usize {
        mem_capacity
    }
    fn in_data_seg(&self, idx: usize) -> bool {
        idx >= self.data_seg_start && idx < self.code_seg_start
    }
    pub fn in_code_seg(&self, idx: usize) -> bool {
        idx >= self.code_seg_start && idx < self.heap_start
    }
    fn in_free_memory(&self, idx: usize) -> bool {
        idx >= self.next_heap_idx() && idx <= self.next_stack_dix()
    }
    fn in_heap(&self, idx: usize) -> bool {
        idx >= self.heap_start && idx < self.next_heap_idx()
    }
    fn in_stack(&self, idx: usize) -> bool {
        idx > self.next_stack_dix() && idx < self.bytes.len()
    }
    fn in_bounds(&self, idx: usize) -> bool {
        idx >= 0 && idx < self.bytes.len()
    }
    fn next_heap_idx(&self) -> usize {
        self.heap_start + self.heap_size
    }
    fn next_stack_dix(&self) -> usize {
        self.bytes.len() - 1 - self.stack_size
    }
    fn get_u8(&self, idx: usize) -> u8 {
        self.bytes[idx]
    }
    fn get_i32(&self, idx: usize) -> i32 {
        let bytes = [
            self.bytes[idx],
            self.bytes[idx + 1],
            self.bytes[idx + 2],
            self.bytes[idx + 3],
        ];
        as_i32_be(&bytes)
    }
    pub fn set_i32(&mut self, idx: usize, int: i32) -> Result<(), MemoryErr> {
        if !self.in_code_seg(idx) || !self.in_code_seg(idx + 3) {
            return Err(MemoryErr::SetInsideCodeSegBounds(idx));
        }
        let bytes = i32_bytes_be(int);
        self.bytes[idx] = bytes[0];
        self.bytes[idx + 1] = bytes[1];
        self.bytes[idx + 2] = bytes[2];
        self.bytes[idx + 3] = bytes[3];
        Ok(())
    }
    pub fn set_u8(&mut self, idx: usize, byte: u8) -> Result<(), MemoryErr> {
        if !self.in_code_seg(idx) {
            return Err(MemoryErr::SetInsideCodeSegBounds(idx));
        }
        self.bytes[idx] = byte;
        Ok(())
    }
    pub fn get_any_i32(&self, idx: usize) -> Result<i32, MemoryErr> {
        if !self.in_bounds(idx) || !self.in_bounds(idx + 3) {
            return Err(MemoryErr::OutOfMemoryBounds(idx));
        }
        Ok(self.get_i32(idx))
    }
    pub fn get_any_u8(&self, idx: usize) -> Result<u8, MemoryErr> {
        if self.in_bounds(idx) {
            return Err(MemoryErr::OutOfMemoryBounds(idx));
        }
        Ok(self.get_u8(idx))
    }
    pub fn get_data_seg_u8(&self, idx: usize) -> Result<u8, MemoryErr> {
        if !self.in_data_seg(idx) {
            return Err(MemoryErr::OutOfDataSegBounds(idx));
        }
        Ok(self.get_u8(idx))
    }
    pub fn get_data_seg_i32(&self, idx: usize) -> Result<i32, MemoryErr> {
        if !self.in_data_seg(idx) || !self.in_data_seg(idx + 3) {
            return Err(MemoryErr::OutOfDataSegBounds(idx));
        }
        Ok(self.get_i32(idx))
    }
    pub fn get_code_seg_i32(&self, idx: usize) -> Result<i32, MemoryErr> {
        if !self.in_code_seg(idx) || !self.in_code_seg(idx + 3) {
            return Err(MemoryErr::OutOfCodeSegBounds(idx));
        }
        Ok(self.get_i32(idx))
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            bytes: [0; 102400],
            data_seg_start: 4,
            code_seg_start: 4,
            heap_start: 4,
            stack_size: 0,
            heap_size: 0,
        }
    }
}

#[derive(Debug)]
pub enum MemoryErr {
    OutOfMemoryBounds(usize),
    OutOfDataSegBounds(usize),
    OutOfCodeSegBounds(usize),
    SetInsideCodeSegBounds(usize),
    StackOverflow(usize),
    StackUnderflow(usize),
    HeapOverflow(usize),
    HeapUnderflow(usize),
}

impl Reportable for MemoryErr {
    fn report(&self) -> String {
        match self {
            MemoryErr::HeapOverflow(p) => format!("Memory error at position: {}\n{:?}", p, self),
            MemoryErr::HeapUnderflow(p) => format!("Memory error at position: {}\n{:?}", p, self),
            MemoryErr::OutOfCodeSegBounds(p) => {
                format!("Memory error at position: {}\n{:?}", p, self)
            }
            MemoryErr::OutOfDataSegBounds(p) => {
                format!("Memory error at position: {}\n{:?}", p, self)
            }
            MemoryErr::OutOfMemoryBounds(p) => {
                format!("Memory error at position: {}\n{:?}", p, self)
            }
            MemoryErr::SetInsideCodeSegBounds(p) => {
                format!("Memory error at position: {}\n{:?}", p, self)
            }
            MemoryErr::StackOverflow(p) => format!("Memory error at position: {}\n{:?}", p, self),
            MemoryErr::StackUnderflow(p) => format!("Memory error at position: {}\n{:?}", p, self),
        }
    }
}
