use crate::util::reportable::Reportable;

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
    fn jmp(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        cpu.pc = self.op1 as usize;
        Ok(())
    }
    fn jmr(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        cpu.pc = int as usize;
        Ok(())
    }
    fn bnz(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        if int != 0 {
            cpu.pc = self.op2 as usize;
        }
        Ok(())
    }
    fn bgt(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        if int > 0 {
            cpu.pc = self.op2 as usize;
        }
        Ok(())
    }
    fn blt(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        if int < 0 {
            cpu.pc = self.op2 as usize;
        }
        Ok(())
    }
    fn brz(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        if int == 0 {
            cpu.pc = self.op2 as usize;
        }
        Ok(())
    }
    fn mov(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
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
        let r1 = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let r2 = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.set_i32(r2.get_i32() as usize, r1.get_i32());
        Ok(())
    }
    fn ldr(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.get_data_seg_i32(self.op2 as usize);
        Ok(())
    }
    fn ldr2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let r2 = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.get_any_i32(r2.get_i32() as usize);
        Ok(())
    }
    fn stb(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let r1 = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.set_u8(self.op2 as usize, r1.get_u8());
        Ok(())
    }
    fn stb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let rs = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let rg = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        cpu.memory.set_u8(rg.get_i32() as usize, rs.get_u8());
        Ok(())
    }
    fn ldb(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let byte = match cpu.memory.get_data_seg_u8(self.op2 as usize) {
            Ok(b) => b,
            Err(e) => e.report_panic()
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_u8(byte);
        Ok(())
    }
    fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let byte = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r.get_u8(),
            Err(e) => return Err(e),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_u8(byte);
        Ok(())
    }
    fn cmp(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let rs_int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let rd_int = rd.get_i32();
        if rd_int == rs_int {
            rd.set_i32(0);
        } else if rd_int > rs_int {
            rd.set_i32(1);
        } else if rd_int < rs_int {
            rd.set_i32(-1);
        }
        Ok(())
    }
    fn cmpi(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let rd_int = rd.get_i32();
        if rd_int == self.op2 {
            rd.set_i32(0);
        } else if rd_int > self.op2 {
            rd.set_i32(1);
        } else if rd_int < self.op2 {
            rd.set_i32(-1);
        }
        Ok(())
    }
    fn add(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_i32(rd.get_i32() + int);
        Ok(())
    }
    fn adi(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_i32(rd.get_i32() + self.op2);
        Ok(())
    }
    fn sub(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_i32(rd.get_i32() - int);
        Ok(())
    }
    fn mul(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_i32(rd.get_i32() * int);
        Ok(())
    }
    fn muli(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_i32(rd.get_i32() * self.op2);
        Ok(())
    }
    fn div(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return Err(e),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_i32(rd.get_i32() / int);
        Ok(())
    }
    fn divi(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        rd.set_i32(rd.get_i32() / self.op2);
        Ok(())
    }
    fn trp0(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {

    }
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
    // fn ldb2(&self, cpu: &mut Cpu) -> Result<(), CpuErr> {
}

pub enum InstructionErr {
    InvalidRegister,
}

impl InstructionErr {}
