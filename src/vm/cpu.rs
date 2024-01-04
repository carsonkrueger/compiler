use crate::vm::memory::Memory;
use super::{register::Register, instruction::Instruction, opcode::Opcode};

pub struct Cpu {
    memory: Memory,
    registers: [Register; 64],
    pc: usize,
    hp: usize,
    sp: usize
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
                ExecuteResult::Success => continue,
                ExecuteResult::Error => panic!("Error executing instruction: {:?}", instruction),
                ExecuteResult::Exit => break,
            }
        }
    }
    fn fetch(&self) -> [i32; 3] {
        let mut bytes: [i32; 3] = [0, 0, 0];
        bytes[0] = self.memory.get_code_seg_i32(self.pc);
        bytes[1] = self.memory.get_code_seg_i32(self.pc + 4);
        bytes[2] = self.memory.get_code_seg_i32(self.pc + 8);
        bytes
    }
    fn decode(&self, bytes: &[i32; 3]) -> Instruction {
        Instruction {
            opcode: Opcode::from(bytes[0]),
            op1: bytes[1],
            op2: bytes[2]
        }
    }
    fn execute(&self, instruction: &Instruction) -> ExecuteResult {
        unimplemented!()
    }
    fn has_next_instruction(&self) -> bool {
        self.memory.in_code_seg(self.pc) && self.memory.in_code_seg(self.pc + 11)
    }
}

pub enum ExecuteResult {
    Success,
    Error,
    Exit
}