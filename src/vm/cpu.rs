use super::{instruction::Instruction, opcode::Opcode, register::Register};
use crate::vm::memory::Memory;

pub struct Cpu {
    pub memory: Memory,
    pub registers: [Register; 64],
    pub pc: usize,
    pub hp: usize,
    pub sp: usize,
}

impl Cpu {
    fn run(&mut self) {
        loop {
            if !self.has_next_instruction() {
                panic!("Cannot fetch next instruction at: {}", self.pc);
            }
            let bytes = self.fetch();
            let instruction = self.decode(&bytes);
            self.pc += 12;
            match self.execute(&instruction) {
                ExecuteResult::Continue => continue,
                ExecuteResult::Exit => break,
                ExecuteResult::Error => panic!("Error executing instruction: {:?}", instruction),
            }
        }
    }
    fn fetch(&self) -> [i32; 3] {
        let mut bytes: [i32; 3] = [0, 0, 0];
        bytes[0] = self
            .memory
            .get_code_seg_i32(self.pc)
            .expect("Could not cpu.fetch() bytes");
        bytes[1] = self
            .memory
            .get_code_seg_i32(self.pc + 4)
            .expect("Could not cpu.fetch() bytes");
        bytes[2] = self
            .memory
            .get_code_seg_i32(self.pc + 8)
            .expect("Could not cpu.fetch() bytes");
        bytes
    }
    fn decode(&self, bytes: &[i32; 3]) -> Instruction {
        Instruction {
            opcode: Opcode::try_from(bytes[0]).expect("Invalid opcode"),
            op1: bytes[1],
            op2: bytes[2],
        }
    }
    fn execute(&self, instruction: &Instruction) -> ExecuteResult {
        unimplemented!()
    }
    fn has_next_instruction(&self) -> bool {
        self.memory.in_code_seg(self.pc) && self.memory.in_code_seg(self.pc + 11)
    }
    pub fn valid_rg(&self, idx: usize) -> bool {
        idx >= 0 && idx < 64
    }
    pub fn rg_at(&self, idx: usize) -> Result<Register, CpuErr> {
        if idx < 0 || idx > self.registers.len() {
            Err(CpuErr::RgOutOfBounds(idx))
        } else {
            Ok(self.registers[idx])
        }
    }
    pub fn rg_at_ref(&self, idx: usize) -> Result<&Register, CpuErr> {
        if idx < 0 || idx > self.registers.len() {
            Err(CpuErr::RgOutOfBounds(idx))
        } else {
            Ok(&self.registers[idx])
        }
    }
    pub fn rg_at_mut(&mut self, idx: usize) -> Result<&mut Register, CpuErr> {
        if idx < 0 || idx > self.registers.len() {
            Err(CpuErr::RgOutOfBounds(idx))
        } else {
            Ok(&mut self.registers[idx])
        }
    }
}

pub enum CpuErr {
    RgOutOfBounds(usize),
}

impl CpuErr {
    pub fn report(&self) -> String {
        match self {
            CpuErr::RgOutOfBounds(i) => format!("Cannot fetch register at: {}", i),
        }
    }
    pub fn report_panic(&self) {
        self.report();
        panic!();
    }
    pub fn invalid_register(idx: usize) {
        println!("Cannot fetch register at: {}", idx);
    }
}

pub enum ExecuteResult {
    Continue,
    Exit,
    Error,
}
