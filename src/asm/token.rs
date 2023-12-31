#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Label,
    LabelOp,
    IntImm,
    CharImm,
    StrImm,
    Comma,
    IntDir,
    BytDir,
    StrDir,
    Rg,
    Pc,
    Sp,
    Hp,
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
    Ldr,
    Stb,
    Ldb,
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
    Trp,
    Comment,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}
