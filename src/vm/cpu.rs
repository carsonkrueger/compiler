use super::{instruction::Instruction, opcode::Opcode, register::Register};
use crate::vm::memory::Memory;

const num_rgs: usize = 64;

pub struct Cpu {
    pub memory: Memory,
    pub registers: [Register; num_rgs],
    pub pc: usize,
    pub hp: usize,
    pub sp: usize,
}

impl Cpu {
    fn new(file_path: String) -> Self {
        let cpu = Self {
            memory: Memory::new(file_path),
            registers: [Register::default(); num_rgs],
            pc: 4,
            hp: 0,
            sp: 0,
        };
        cpu
    }
    fn run(&mut self) {
        self.pc = self
            .memory
            .get_any_i32(0)
            .expect("Could not fetch initial pc") as usize;

        loop {
            if !self.has_next_instruction() {
                panic!("Cannot fetch next instruction at: {}", self.pc);
            }
            let ints = self.fetch();
            let instruction = self.decode(&ints);
            self.pc += 12;
            match self.execute(&instruction) {
                ExecuteResult::Continue => continue,
                ExecuteResult::Exit => break,
                ExecuteResult::Error => panic!("Error executing instruction: {:?}", instruction),
            }
        }
    }
    fn fetch(&self) -> [i32; 3] {
        let mut ints: [i32; 3] = [0, 0, 0];
        ints[0] = self
            .memory
            .get_code_seg_i32(self.pc)
            .expect("Could not cpu.fetch() bytes");
        ints[1] = self
            .memory
            .get_code_seg_i32(self.pc + 4)
            .expect("Could not cpu.fetch() bytes");
        ints[2] = self
            .memory
            .get_code_seg_i32(self.pc + 8)
            .expect("Could not cpu.fetch() bytes");
        ints
    }
    fn decode(&self, ints: &[i32; 3]) -> Instruction {
        Instruction {
            opcode: Opcode::try_from(ints[0]).expect("Invalid opcode"),
            op1: ints[1],
            op2: ints[2],
        }
    }
    fn execute(&mut self, instruction: &Instruction) -> ExecuteResult {
        // unimplemented!()
        match instruction.opcode {
            Opcode::add => {
                instruction.add(self).expect("Error");
                ExecuteResult::Continue
            }
            _ => ExecuteResult::Error,
        };
        ExecuteResult::Continue
    }
    fn has_next_instruction(&self) -> bool {
        self.memory.in_code_seg(self.pc) && self.memory.in_code_seg(self.pc + 11)
    }
    pub fn valid_rg(&self, idx: usize) -> bool {
        idx >= 0 && idx < 64
    }
    // pub fn rg_at(&self, idx: usize) -> Result<Register, CpuErr> {
    //     if idx < 0 || idx > self.registers.len() {
    //         Err(CpuErr::RgOutOfBounds(idx))
    //     } else {
    //         Ok(self.registers[idx])
    //     }
    // }
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

#[derive(Debug)]
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
