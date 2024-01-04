use super::opcode::Opcode;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub op1: i32,
    pub op2: i32
}