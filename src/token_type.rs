#[derive(Debug)]
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
