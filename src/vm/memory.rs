use crate::util::endianness::{self, as_u32_be};
use core::num;

pub struct Memory {
    bytes: [u8; 102400],
    code_seg_start: usize,
    data_seg_size: usize,
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
        let init_pc = as_u32_be(&bytes[0]);
        Self {
            bytes,
            code_seg_start: init_pc,
            data_seg_size: init_pc - 4,
            // code_seg_size:
        }
    }
    fn get_bytes(&self, start: usize, num_bytes: usize) -> Result<Vec<u8>, MemoryErr> {
        let end = start + num_bytes;
        if start < 0 || end >= self.bytes.len() {
            return Err(MemoryErr::OutOfMemoryBounds);
        }
        Ok(self
            .bytes
            .iter()
            .skip(start)
            .take(num_bytes)
            .collect::<Vec<u8>>())
    }
}
