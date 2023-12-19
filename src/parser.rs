use crate::expr::Expr;
use crate::statements::stmt::Stmt;
use crate::token::{token_type::TokenType, Token};

pub trait Parsable {
    type OutErr;
    type OutOk;
    fn parse() -> Result<Self::OutOk, Self::OutErr>;
}

pub enum ParseError {
    Invalid,
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    cur_idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, cur_idx: 0 }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.cur_idx]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.cur_idx - 1]
    }
    fn advance(&mut self) {
        self.cur_idx += 1;
    }
    /// consumes and advances IF current token matches token_type argument. Returns true if successfully consumed token.
    fn consume_match(&mut self, token_type: &TokenType) -> bool {
        let bool = &self.peek().token_type == token_type;
        if bool {
            self.advance();
        }
        bool
    }
}

impl Parsable for Parser<'_> {
    type OutOk = Vec<Stmt>;
    type OutErr = ParseError;
    // will return a list of statements
    fn parse() -> Result<Self::OutOk, Self::OutErr> {
        unimplemented!()
    }
}
