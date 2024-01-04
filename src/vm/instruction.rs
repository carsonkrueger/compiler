use super::{opcode::Opcode, cpu::Cpu};

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub op1: i32,
    pub op2: i32
}

impl Instruction {
    fn mov(&self, cpu: &mut Cpu) {
        cpu.registers[self.op1] = cpu.registers[self.op2];
    }
}