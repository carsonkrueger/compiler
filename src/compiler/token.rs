#[derive(Debug, PartialEq)]
pub enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semi,
    Slash,
    Star,

    Bang,
    BangEq,
    Eq,
    EqEq,
    Gt,
    GtEq,
    Lt,
    LtEq,

    Identifier,
    Str,
    Num,
    Bool,

    And,
    Or,
    Class,
    Else,
    If,
    Fn,
    For,
    While,
    Nil,
    Print,
    Return,
    This,
    True,
    False,
    Interface,

    Eof,
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
