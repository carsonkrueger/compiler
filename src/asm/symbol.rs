use crate::asm::token::Token;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub token: Token,
    pub offset: usize,
}

impl Symbol {
    pub fn new(token: Token, offset: usize) -> Self {
        Self { token, offset }
    }
}
