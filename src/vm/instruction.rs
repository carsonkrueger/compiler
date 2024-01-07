use crate::util::reportable::Reportable;

use super::{
    cpu::{Cpu, ExecuteResult, VMErr},
    opcode::Opcode,
};

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub op1: i32,
    pub op2: i32,
}

impl Instruction {
    pub fn jmp(&self, cpu: &mut Cpu) -> ExecuteResult {
        cpu.pc = self.op1 as usize;
        ExecuteResult::Continue
    }
    pub fn jmr(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        cpu.pc = int as usize;
        ExecuteResult::Continue
    }
    pub fn bnz(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        if int != 0 {
            cpu.pc = self.op2 as usize;
        }
        ExecuteResult::Continue
    }
    pub fn bgt(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        if int > 0 {
            cpu.pc = self.op2 as usize;
        }
        ExecuteResult::Continue
    }
    pub fn blt(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        if int < 0 {
            cpu.pc = self.op2 as usize;
        }
        ExecuteResult::Continue
    }
    pub fn brz(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        if int == 0 {
            cpu.pc = self.op2 as usize;
        }
        ExecuteResult::Continue
    }
    pub fn mov(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(rs) => rs.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op2 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(int);
        ExecuteResult::Continue
    }
    pub fn movi(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(self.op2);
        ExecuteResult::Continue
    }
    pub fn lda(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(self.op2);
        ExecuteResult::Continue
    }
    // fn stf(&self, cpu: &mut Cpu) -> ExecuteResult {
    //     let r1 = match cpu.rg_at_ref(self.op1 as usize) {
    //         Ok(r) => r,
    //         Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
    //     };
    //     cpu.memory.set_i32(self.op2 as usize, r1.get_f32() as i32);
    //     ExecuteResult::Continue
    // }
    // fn stf2(&self, cpu: &mut Cpu) -> ExecuteResult {
    //     let r1 = match cpu.rg_at_ref(self.op1 as usize) {
    //         Ok(r) => r,
    //         Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
    //     };
    //     let r2 = match cpu.rg_at_ref(self.op1 as usize) {
    //         Ok(r) => r,
    //         Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
    //     };
    //     cpu.memory
    //         .set_i32(r2.get_i32() as usize, r1.get_f32() as i32);
    //     ExecuteResult::Continue
    // }
    // fn ldf(&self, cpu: &mut Cpu) -> ExecuteResult {
    //     let r1 = match cpu.rg_at_ref(self.op1 as usize) {
    //         Ok(r) => r,
    //         Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
    //     };
    //     cpu.memory.get_data_seg_i32(self.op2 as usize);
    //     ExecuteResult::Continue
    // }
    // fn ldf2(&self, cpu: &mut Cpu) -> ExecuteResult {
    //     let r1 = match cpu.rg_at_ref(self.op1 as usize) {
    //         Ok(r) => r,
    //         Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
    //     };
    //     let r2 = match cpu.rg_at_ref(self.op1 as usize) {
    //         Ok(r) => r,
    //         Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
    //     };
    //     cpu.memory.get_any_i32(r2.get_i32() as usize);
    //     ExecuteResult::Continue
    // }
    pub fn str(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rs = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        match cpu.memory.set_i32(self.op2 as usize, rs.get_i32()) {
            Ok(_) => ExecuteResult::Continue,
            Err(e) => ExecuteResult::Error(VMErr::MemoryErr(e)),
        }
    }
    pub fn str2(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rs = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rg = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        match cpu.memory.set_i32(rg.get_i32() as usize, rs.get_i32()) {
            Ok(_) => ExecuteResult::Continue,
            Err(e) => ExecuteResult::Error(VMErr::MemoryErr(e)),
        }
    }
    pub fn ldr(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.memory.get_data_seg_i32(self.op2 as usize) {
            Ok(i) => i,
            Err(e) => return ExecuteResult::Error(VMErr::MemoryErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(int);
        ExecuteResult::Continue
    }
    pub fn ldr2(&self, cpu: &mut Cpu) -> ExecuteResult {
        let addr = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let int = match cpu.memory.get_any_i32(addr as usize) {
            Ok(i) => i,
            Err(e) => return ExecuteResult::Error(VMErr::MemoryErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(int);
        ExecuteResult::Continue
    }
    pub fn stb(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rs = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        match cpu.memory.set_u8(self.op2 as usize, rs.get_u8()) {
            Ok(_) => ExecuteResult::Continue,
            Err(e) => ExecuteResult::Error(VMErr::MemoryErr(e)),
        }
    }
    pub fn stb2(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rs = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rg = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        match cpu.memory.set_u8(rg.get_i32() as usize, rs.get_u8()) {
            Ok(_) => ExecuteResult::Continue,
            Err(e) => ExecuteResult::Error(VMErr::MemoryErr(e)),
        }
    }
    pub fn ldb(&self, cpu: &mut Cpu) -> ExecuteResult {
        let byte = match cpu.memory.get_data_seg_u8(self.op2 as usize) {
            Ok(b) => b,
            Err(e) => return ExecuteResult::Error(VMErr::MemoryErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_u8(byte);
        ExecuteResult::Continue
    }
    pub fn ldb2(&self, cpu: &mut Cpu) -> ExecuteResult {
        let byte = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r.get_u8(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_u8(byte);
        ExecuteResult::Continue
    }
    pub fn cmp(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rs_int = match cpu.rg_at_ref(self.op1 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd_int = rd.get_i32();
        if rd_int == rs_int {
            rd.set_i32(0);
        } else if rd_int > rs_int {
            rd.set_i32(1);
        } else if rd_int < rs_int {
            rd.set_i32(-1);
        }
        ExecuteResult::Continue
    }
    pub fn cmpi(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rs_int = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd_int = rd.get_i32();
        if rd_int == rs_int {
            rd.set_i32(0);
        } else if rd_int > rs_int {
            rd.set_i32(1);
        } else if rd_int < rs_int {
            rd.set_i32(-1);
        }
        ExecuteResult::Continue
    }
    pub fn add(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(rd.get_i32() + int);
        ExecuteResult::Continue
    }
    pub fn adi(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(rd.get_i32() + self.op2);
        ExecuteResult::Continue
    }
    pub fn sub(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(rd.get_i32() - int);
        ExecuteResult::Continue
    }
    pub fn mul(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(rd.get_i32() * int);
        ExecuteResult::Continue
    }
    pub fn muli(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(rd.get_i32() * self.op2);
        ExecuteResult::Continue
    }
    pub fn div(&self, cpu: &mut Cpu) -> ExecuteResult {
        let int = match cpu.rg_at_ref(self.op2 as usize) {
            Ok(r) => r.get_i32(),
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(rd.get_i32() / int);
        ExecuteResult::Continue
    }
    pub fn divi(&self, cpu: &mut Cpu) -> ExecuteResult {
        let rd = match cpu.rg_at_mut(self.op1 as usize) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        rd.set_i32(rd.get_i32() / self.op2);
        ExecuteResult::Continue
    }
    pub fn trp(&self, cpu: &mut Cpu) -> ExecuteResult {
        match self.op1 {
            0 => self.trp0(cpu),
            1 => self.trp1(cpu),
            2 => self.trp2(cpu),
            3 => self.trp3(cpu),
            4 => self.trp4(cpu),
            // 5 => self.trp5(cpu),
            // 6 => self.trp6(cpu),
            // 7 => self.trp7(cpu),
            _ => ExecuteResult::Error(VMErr::CpuErr(super::cpu::CpuErr::InvalidInstruction(
                self.clone(),
            ))),
        }
    }
    fn trp0(&self, cpu: &mut Cpu) -> ExecuteResult {
        ExecuteResult::Exit
    }
    fn trp1(&self, cpu: &mut Cpu) -> ExecuteResult {
        let r3 = match cpu.rg_at_ref(3) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        print!("{}", r3.get_i32());
        ExecuteResult::Continue
    }
    fn trp2(&self, cpu: &mut Cpu) -> ExecuteResult {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(s) => s,
            Err(_) => return ExecuteResult::Error(VMErr::IOError),
        };
        let r3 = match cpu.rg_at_mut(3) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let int = match line.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return ExecuteResult::Error(VMErr::IOError),
        };
        r3.set_i32(int);
        ExecuteResult::Continue
    }
    fn trp3(&self, cpu: &mut Cpu) -> ExecuteResult {
        let r3 = match cpu.rg_at_ref(3) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        print!("{}", r3.get_u8() as char);
        ExecuteResult::Continue
    }
    fn trp4(&self, cpu: &mut Cpu) -> ExecuteResult {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(s) => s,
            Err(_) => return ExecuteResult::Error(VMErr::IOError),
        };
        let r3 = match cpu.rg_at_mut(3) {
            Ok(r) => r,
            Err(e) => return ExecuteResult::Error(VMErr::CpuErr(e)),
        };
        let int = match line.parse::<u8>() {
            Ok(i) => i,
            Err(_) => return ExecuteResult::Error(VMErr::IOError),
        };
        r3.set_u8(int);
        ExecuteResult::Continue
    }
    // fn trp4(&self, cpu: &mut Cpu) -> ExecuteResult {}
    // fn trp5(&self, cpu: &mut Cpu) -> ExecuteResult {}
}
