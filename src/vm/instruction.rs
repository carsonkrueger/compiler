use super::{
    cpu::{Cpu, CpuErr},
    opcode::Opcode,
};

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub op1: i32,
    pub op2: i32,
}

impl Instruction {
    unsafe fn mov(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        }
        .get_i32();
        let r1 = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        r1.set_i32(int);
        Ok(())
    }
    fn movi(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        r1.set_i32(self.op2);
        Ok(())
    }
    fn lda(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        r1.set_i32(self.op2);
        Ok(())
    }
    fn str(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.set_i32(self.op2 as usize, r1.get_i32());
        Ok(())
    }
    fn str2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let r2 = match cpu.rg_at(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.set_i32(r2.get_i32() as usize, r1.get_i32());
        Ok(())
    }
    fn ldr(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.get_data_seg_i32(self.op2 as usize);
        Ok(())
    }
    fn ldr2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let r2 = match cpu.rg_at(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.get_any_i32(r2.get_i32() as usize);
        Ok(())
    }
    fn stb(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.set_u8(self.op2 as usize, r1.get_u8());
        Ok(())
    }
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
    // fn lda(&self, cpu: &mut Cpu) {}
}

pub enum InstructionErr {
    InvalidRegister,
}

impl InstructionErr {}
