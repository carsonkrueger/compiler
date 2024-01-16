use std::fmt::{format, Display};

use super::{
    instruction::{self, Instruction},
    memory::MemoryErr,
    opcode::{Opcode, OpcodeErr},
    register::Register,
};
use crate::vm::memory::Memory;

/// PC = 64, SL = 65, SB = 66, SP = 67, FP = 68, HP = 69
const NUM_RGS: usize = 70;

pub struct Cpu {
    pub memory: Memory,
    pub registers: [Register; NUM_RGS],
    // pub pc: usize,
    // pub hp: usize,
    // pub sp: usize,
}

impl Cpu {
    pub fn new(file_path: &String) -> Self {
        let cpu = Self {
            memory: Memory::new(file_path),
            registers: [Register::default(); NUM_RGS],
            // pc: 4,
            // hp: 0,
            // sp: 0,
        };
        cpu
    }
    pub fn run(&mut self) {
        let first_int = self
            .memory
            .get_any_i32(0)
            .expect("Could not fetch initial pc");
        self.set_pc(first_int);

        loop {
            if !self.has_next_instruction() {
                panic!("Cannot fetch next instruction at: {}", self.get_pc());
            }

            // fetch
            let ints = self.fetch();
            // decode
            let instruction = self.decode(&ints);
            // increment pc
            self.set_pc(self.get_pc() + 12);
            // execute
            match instruction.execute(self) {
                ExecuteResult::Continue => continue,
                ExecuteResult::Exit => break,
                ExecuteResult::Error(e) => {
                    panic!("Error at PC = {}\n{}", self.get_pc() - 12, e)
                }
            }
        }
    }
    fn fetch(&self) -> [i32; 3] {
        let mut ints: [i32; 3] = [0, 0, 0];
        ints[0] = self
            .memory
            .get_code_seg_i32(self.get_pc() as usize)
            .expect("Could not cpu.fetch() bytes");
        ints[1] = self
            .memory
            .get_code_seg_i32(self.get_pc() as usize + 4)
            .expect("Could not cpu.fetch() bytes");
        ints[2] = self
            .memory
            .get_code_seg_i32(self.get_pc() as usize + 8)
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
    // fn execute(&mut self, instruction: &Instruction) -> ExecuteResult {
    //     // unimplemented!()

    // }
    fn has_next_instruction(&self) -> bool {
        self.memory.in_code_seg(self.get_pc() as usize)
            && self.memory.in_code_seg(self.get_pc() as usize + 11)
    }
    pub fn valid_rg(&self, idx: usize) -> bool {
        idx < NUM_RGS
    }
    // pub fn rg_at(&self, idx: usize) -> Result<Register, CpuErr> {
    //     if idx < 0 || idx > self.registers.len() {
    //         Err(CpuErr::RgOutOfBounds(idx))
    //     } else {
    //         Ok(self.registers[idx])
    //     }
    // }
    pub fn rg_at_ref(&self, idx: usize) -> Result<&Register, CpuErr> {
        if idx > self.registers.len() {
            Err(CpuErr::RgOutOfBounds(idx))
        } else {
            Ok(&self.registers[idx])
        }
    }
    pub fn rg_at_mut(&mut self, idx: usize) -> Result<&mut Register, CpuErr> {
        if idx > self.registers.len() {
            Err(CpuErr::RgOutOfBounds(idx))
        } else {
            Ok(&mut self.registers[idx])
        }
    }
    pub fn get_pc(&self) -> i32 {
        self.rg_at_ref(Register::pc_idx()).unwrap().get_i32()
    }
    pub fn set_pc(&mut self, pc: i32) {
        self.rg_at_mut(Register::pc_idx()).unwrap().set_i32(pc)
    }
}

#[derive(Debug)]
pub enum CpuErr {
    RgOutOfBounds(usize),
    InvalidInstruction(Instruction),
}

impl Display for CpuErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CpuErr::RgOutOfBounds(i) => write!(f, "Cannot fetch register at: {}", i),
            CpuErr::InvalidInstruction(i) => write!(f, "Cannot execute instruction: {:?}", i),
        }
    }
}

pub enum VMErr {
    MemoryErr(MemoryErr),
    CpuErr(CpuErr),
    OpcodeErr(OpcodeErr),
    IOError { trp_num: usize },
}

impl Display for VMErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            VMErr::CpuErr(e) => write!(f, "{}", e),
            VMErr::MemoryErr(e) => write!(f, "{}", e),
            VMErr::OpcodeErr(e) => write!(f, "{}", e),
            VMErr::IOError { trp_num } => write!(f, "IO Error with TRP instruction {}", trp_num),
        }
    }
}

pub enum ExecuteResult {
    Continue,
    Exit,
    Error(VMErr),
}
