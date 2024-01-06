use crate::util::reportable::Reportable;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    jmp,
    jmr,
    bnz,
    bgt,
    blt,
    brz,
    bal,
    mov,
    movi,
    lda,
    // stf,
    // stf2,
    // ldf,
    // ldf2,
    str,
    str2,
    ldr,
    ldr2,
    stb,
    stb2,
    ldb,
    ldb2,
    push,
    pop,
    peek,
    and,
    or,
    not,
    cmp,
    cmpi,
    add,
    adi,
    sub,
    mul,
    muli,
    div,
    divi,
    alci,
    allc,
    allc2,
    trp,
}

impl Into<i32> for Opcode {
    fn into(self) -> i32 {
        match self {
            Self::jmp => 1,
            Self::jmr => 2,
            Self::bnz => 3,
            Self::bgt => 4,
            Self::blt => 5,
            Self::brz => 6,
            Self::bal => 43,
            Self::mov => 7,
            Self::movi => 31,
            Self::lda => 8,
            // Self::stf => 44,
            // Self::stf2 => 45,
            // Self::ldf => 46,
            // Self::ldf2 => 47,
            Self::str => 9,
            Self::str2 => 22,
            Self::ldr => 10,
            Self::ldr2 => 23,
            Self::stb => 11,
            Self::stb2 => 24,
            Self::ldb => 12,
            Self::ldb2 => 25,
            Self::push => 40,
            Self::pop => 41,
            Self::peek => 42,
            Self::and => 18,
            Self::or => 19,
            Self::not => 39,
            Self::cmp => 20,
            Self::cmpi => 32,
            Self::add => 13,
            Self::adi => 14,
            Self::sub => 15,
            Self::mul => 16,
            Self::muli => 33,
            Self::div => 17,
            Self::divi => 34,
            Self::alci => 35,
            Self::allc => 36,
            Self::allc2 => 37,
            Self::trp => 21,
            // _ => return Err(OpcodeErr::InvalidOpcode),
        }
    }
}

impl TryFrom<i32> for Opcode {
    type Error = OpcodeErr;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::jmp,
            2 => Self::jmr,
            3 => Self::bnz,
            4 => Self::bgt,
            5 => Self::blt,
            6 => Self::brz,
            43 => Self::bal,
            7 => Self::mov,
            31 => Self::movi,
            8 => Self::lda,
            // 44 => Self::stf,
            // 45 => Self::stf2,
            // 46 => Self::ldf,
            // 47 => Self::ldf2,
            9 => Self::str,
            22 => Self::str2,
            10 => Self::ldr,
            23 => Self::ldr2,
            11 => Self::stb,
            24 => Self::stb2,
            12 => Self::ldb,
            25 => Self::ldb2,
            40 => Self::push,
            41 => Self::pop,
            42 => Self::peek,
            18 => Self::and,
            19 => Self::or,
            39 => Self::not,
            20 => Self::cmp,
            32 => Self::cmpi,
            13 => Self::add,
            14 => Self::adi,
            15 => Self::sub,
            16 => Self::mul,
            33 => Self::muli,
            17 => Self::div,
            34 => Self::divi,
            35 => Self::alci,
            36 => Self::allc,
            37 => Self::allc2,
            21 => Self::trp,
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
