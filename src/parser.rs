use crate::{
    ast::Expr,
    token::{token_type::TokenType, Token},
};

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
    fn consume_first_match(&mut self, token_types: &[TokenType]) -> bool {
        for t in token_types {
            if &self.peek().token_type == t {
                self.advance();
                return true;
            }
        }
        false
    }
    fn expression(&mut self) -> Expr {
        unimplemented!()
    }
    fn literal(&mut self) -> Result<Expr, ()> {
        let types = [
            TokenType::Num,
            TokenType::Nil,
            TokenType::True,
            TokenType::False,
        ];
        if self.consume_first_match(&types) {
            let t = self.previous();
            match t.token_type {
                TokenType::Num => Ok(Expr::Float(t.lexeme.parse::<f32>().unwrap())),
                TokenType::Nil => Ok(Expr::Nil),
                TokenType::True => Ok(Expr::Bool(true)),
                TokenType::False => Ok(Expr::Bool(false)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

// impl Parsable for Parser<'_> {
//     type OutOk = Vec<Stmt>;
//     type OutErr = ParseError;
//     // will return a list of statements
//     fn parse() -> Result<Self::OutOk, Self::OutErr> {
//         unimplemented!()
//     }
// }
