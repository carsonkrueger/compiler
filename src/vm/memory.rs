use crate::util::endianness::{self, as_i32_be};

pub struct Memory {
    bytes: [u8; 102400],
    data_seg_start: usize,
    code_seg_start: usize,
    heap_start: usize,
    heap_size: usize,
    stack_size: usize,
    // code_seg_size: usize,
}

pub enum MemoryErr {
    OutOfMemoryBounds,
    OutOfDataSegBounds,
    OutOfCodeSegBounds,
    StackOverflow,
    StackUnderflow,
    HeapOverflow,
    HeapUnderflow,
}

impl Memory {
    pub fn new(bytes: [u8; 102400]) -> Self {
        let init_pc = as_i32_be(&bytes[0]);
        Self {
            bytes,
            data_seg_start: 4,
            code_seg_start: init_pc,
            heap_start: init_pc,
            heap_size: init_pc,
            stack_size: bytes.len() - 1
        }
    }
    pub fn in_data_seg(&self, idx: usize) -> bool {
        idx >= self.data_seg_start && idx < self.code_seg_start
    }
    pub fn in_code_seg(&self, idx: usize) -> bool {
        idx >= self.code_seg_start && idx < self.heap_start
    }
    pub fn in_free_memory(&self, idx: usize) -> bool {
        idx >= self.next_heap_idx() && idx <= self.next_stack_dix()
    }
    pub fn in_heap(&self, idx: usize) -> bool {
        idx >= self.heap_start && idx < self.next_heap_idx()
    }
    pub fn in_stack(&self, idx: usize) -> bool {
        idx > self.next_stack_dix() && idx < self.bytes.len()
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
        as_i32_be(&self.bytes[idx])
    }
    pub fn get_data_seg_u8(&self, idx: usize) -> Result<u8, MemoryErr> {
        if idx < 4 || idx >= self.code_seg_start {
            Err(MemoryErr::OutOfDataSegBounds)
        } else {
            Ok(self.get_u8(idx))
        }
    }
    pub fn get_code_seg_i32(&self, idx: usize) -> i32 {
        if idx < self.code_seg_start || idx >= self.heap_start {
            Err(MemoryErr::OutOfCodeSegBounds)
        } else {
            Ok(self.get_u32(idx))
        }
    }
}
