use crate::util::endianness::{self, as_i32_be, i32_bytes};

pub struct Memory {
    bytes: [u8; 102400],
    data_seg_start: usize,
    code_seg_start: usize,
    heap_start: usize,
    heap_size: usize,
    stack_size: usize,
    // code_seg_size: usize,
}

#[derive(Debug)]
pub enum MemoryErr {
    OutOfMemoryBounds,
    OutOfDataSegBounds,
    OutOfCodeSegBounds,
    SetInsideCodeSegBounds,
    StackOverflow,
    StackUnderflow,
    HeapOverflow,
    HeapUnderflow,
}

impl Memory {
    pub fn new(bytes: [u8; 102400]) -> Self {
        let first_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let init_pc = as_i32_be(&first_bytes) as usize;
        Self {
            bytes,
            data_seg_start: 4,
            code_seg_start: init_pc,
            heap_start: init_pc,
            heap_size: init_pc,
            stack_size: bytes.len() - 1,
        }
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
            return Err(MemoryErr::SetInsideCodeSegBounds);
        }
        let bytes = i32_bytes(int);
        self.bytes[idx] = bytes[0];
        self.bytes[idx + 1] = bytes[1];
        self.bytes[idx + 2] = bytes[2];
        self.bytes[idx + 3] = bytes[3];
        Ok(())
    }
    pub fn set_u8(&mut self, idx: usize, int: u8) -> Result<(), MemoryErr> {
        if !self.in_code_seg(idx) {
            return Err(MemoryErr::SetInsideCodeSegBounds);
        }
        self.bytes[idx] = int;
        Ok(())
    }
    pub fn get_any_i32(&self, idx: usize) -> Result<i32, MemoryErr> {
        if !self.in_bounds(idx) || !self.in_bounds(idx + 3) {
            return Err(MemoryErr::OutOfMemoryBounds);
        }
        Ok(self.get_i32(idx))
    }
    pub fn get_any_u8(&self, idx: usize) -> Result<u8, MemoryErr> {
        if self.in_bounds(idx) {
            return Err(MemoryErr::OutOfMemoryBounds);
        }
        Ok(self.get_u8(idx))
    }
    pub fn get_data_seg_u8(&self, idx: usize) -> Result<u8, MemoryErr> {
        if !self.in_data_seg(idx) {
            return Err(MemoryErr::OutOfDataSegBounds);
        }
        Ok(self.get_u8(idx))
    }
    pub fn get_data_seg_i32(&self, idx: usize) -> Result<i32, MemoryErr> {
        if !self.in_data_seg(idx) || !self.in_data_seg(idx + 3) {
            return Err(MemoryErr::OutOfDataSegBounds);
        }
        Ok(self.get_i32(idx))
    }
    pub fn get_code_seg_i32(&self, idx: usize) -> Result<i32, MemoryErr> {
        if !self.in_code_seg(idx) || !self.in_code_seg(idx + 3) {
            return Err(MemoryErr::OutOfCodeSegBounds);
        }
        Ok(self.get_i32(idx))
    }
}
