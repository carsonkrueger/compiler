use crate::util::reportable::Reportable;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Jmp,
    Jmr,
    Bnz,
    Bgt,
    Blt,
    Brz,
    Bal,
    Mov,
    Movi,
    Lda,
    Str,
    Str2,
    Ldr,
    Ldr2,
    Stb,
    Stb2,
    Ldb,
    Ldb2,
    Push,
    Pop,
    Peek,
    And,
    Or,
    Not,
    Cmp,
    Cmpi,
    Add,
    Adi,
    Sub,
    Mul,
    Muli,
    Div,
    Divi,
    Alci,
    Allc,
    Allc2,
    Trp,
    // stf,
    // stf2,
    // ldf,
    // ldf2,
}

impl Into<i32> for Opcode {
    fn into(self) -> i32 {
        match self {
            Self::Jmp => 1,
            Self::Jmr => 2,
            Self::Bnz => 3,
            Self::Bgt => 4,
            Self::Blt => 5,
            Self::Brz => 6,
            Self::Bal => 43,
            Self::Mov => 7,
            Self::Movi => 31,
            Self::Lda => 8,
            // Self::stf => 44,
            // Self::stf2 => 45,
            // Self::ldf => 46,
            // Self::ldf2 => 47,
            Self::Str => 9,
            Self::Str2 => 22,
            Self::Ldr => 10,
            Self::Ldr2 => 23,
            Self::Stb => 11,
            Self::Stb2 => 24,
            Self::Ldb => 12,
            Self::Ldb2 => 25,
            Self::Push => 40,
            Self::Pop => 41,
            Self::Peek => 42,
            Self::And => 18,
            Self::Or => 19,
            Self::Not => 39,
            Self::Cmp => 20,
            Self::Cmpi => 32,
            Self::Add => 13,
            Self::Adi => 14,
            Self::Sub => 15,
            Self::Mul => 16,
            Self::Muli => 33,
            Self::Div => 17,
            Self::Divi => 34,
            Self::Alci => 35,
            Self::Allc => 36,
            Self::Allc2 => 37,
            Self::Trp => 21,
            // _ => return Err(OpcodeErr::InvalidOpcode),
        }
    }
}

impl TryFrom<i32> for Opcode {
    type Error = OpcodeErr;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Jmp,
            2 => Self::Jmr,
            3 => Self::Bnz,
            4 => Self::Bgt,
            5 => Self::Blt,
            6 => Self::Brz,
            43 => Self::Bal,
            7 => Self::Mov,
            31 => Self::Movi,
            8 => Self::Lda,
            // 44 => Self::stf,
            // 45 => Self::stf2,
            // 46 => Self::ldf,
            // 47 => Self::ldf2,
            9 => Self::Str,
            22 => Self::Str2,
            10 => Self::Ldr,
            23 => Self::Ldr2,
            11 => Self::Stb,
            24 => Self::Stb2,
            12 => Self::Ldb,
            25 => Self::Ldb2,
            40 => Self::Push,
            41 => Self::Pop,
            42 => Self::Peek,
            18 => Self::And,
            19 => Self::Or,
            39 => Self::Not,
            20 => Self::Cmp,
            32 => Self::Cmpi,
            13 => Self::Add,
            14 => Self::Adi,
            15 => Self::Sub,
            16 => Self::Mul,
            33 => Self::Muli,
            17 => Self::Div,
            34 => Self::Divi,
            35 => Self::Alci,
            36 => Self::Allc,
            37 => Self::Allc2,
            21 => Self::Trp,
            op => return Err(OpcodeErr::InvalidOpcode(op)),
        })
    }
}

#[derive(Debug)]
pub enum OpcodeErr {
    InvalidOpcode(i32),
}

impl Reportable for OpcodeErr {
    fn report(&self) -> String {
        match &self {
            OpcodeErr::InvalidOpcode(o) => format!("Invalid opcode: {}", o),
        }
    }
}
