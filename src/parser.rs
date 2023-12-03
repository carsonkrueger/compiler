use crate::expressions::expr::Expr;
use crate::statements::stmt::Stmt;
use crate::{token::Token, token_type::TokenType};
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    cur_idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, cur_idx: 0 }
    }
    // will return a list of statements
    pub fn parse() -> Vec<Stmt> {
        unimplemented!()
    }
    fn current(&self) -> &Token {
        &self.tokens[self.cur_idx]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.cur_idx - 1]
    }
    fn advance(&mut self) {
        self.cur_idx += 1;
    }
    /// consumes and advances IF current token matches token_type argument. Returns true if succesfully consumed token.
    fn consume_match(&mut self, token_type: &TokenType) -> bool {
        let bool = &self.current().token_type == token_type;
        if bool {
            self.advance();
        }
        bool
    }
    fn expression() -> Expr {
        unimplemented!()
    }
    fn statement() -> Stmt {
        unimplemented!()
    }
}
