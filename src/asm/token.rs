#[derive(Debug)]
pub enum TokenType {
    label,
    i_int,
    i_char,
    i_str,
    comma,
    int_dir,
    byt_dir,
    str_dir,
    rg,
    pc,
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
    str,
    ldr,
    stb,
    ldb,
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
    trp,
}

#[derive(Debug)]
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
