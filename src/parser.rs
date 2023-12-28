use core::panic;

use crate::{
    ast::{BinaryOp, Expr, Literal, UnaryOp},
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

/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary | primary ;
/// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
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
    fn consume_match(&mut self, token_type: TokenType) -> bool {
        let bool = self.peek().token_type == token_type;
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
        match self.equality() {
            Ok(e) => e,
            Err(e) => panic!("Error parsing expression"),
        }
    }
    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = match self.comparison() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let mut op = BinaryOp::EqEq; // just made multiply default op. If no valid op is found, then returns Err
        let types = [TokenType::BangEq, TokenType::EqEq];

        while (self.consume_first_match(&types)) {
            op = match BinaryOp::try_from(&self.previous().token_type) {
                Ok(o) => o,
                Err(e) => return Err(()),
            };

            let rhs = match self.comparison() {
                Ok(u) => Box::new(u),
                Err(e) => return Err(e),
            };

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            };
        }

        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = match self.term() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let mut op = BinaryOp::Gt; // just made multiply default op. If no valid op is found, then returns Err
        let types = [
            TokenType::Gt,
            TokenType::GtEq,
            TokenType::Lt,
            TokenType::LtEq,
        ];

        while (self.consume_first_match(&types)) {
            op = match BinaryOp::try_from(&self.previous().token_type) {
                Ok(o) => o,
                Err(e) => return Err(()),
            };

            let rhs = match self.term() {
                Ok(u) => Box::new(u),
                Err(e) => return Err(e),
            };

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            };
        }

        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, ()> {
        let mut expr = match self.factor() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let mut op = BinaryOp::Plus; // just made multiply default op. If no valid op is found, then returns Err
        let types = [TokenType::Minus, TokenType::Minus];

        while (self.consume_first_match(&types)) {
            op = match BinaryOp::try_from(&self.previous().token_type) {
                Ok(o) => o,
                Err(e) => return Err(()),
            };

            let rhs = match self.factor() {
                Ok(u) => Box::new(u),
                Err(e) => return Err(e),
            };

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            };
        }

        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expr = match self.unary() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let mut op = BinaryOp::Mult; // just made multiply default op. If no valid op is found, then returns Err
        let types = [TokenType::Slash, TokenType::Star];

        while (self.consume_first_match(&types)) {
            op = match BinaryOp::try_from(&self.previous().token_type) {
                Ok(o) => o,
                Err(e) => return Err(()),
            };

            let rhs = match self.unary() {
                Ok(u) => Box::new(u),
                Err(e) => return Err(e),
            };

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            };
        }

        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr, ()> {
        let types = [TokenType::Minus, TokenType::Bang];
        if self.consume_first_match(&types) {
            if let Ok(op) = UnaryOp::try_from(&self.previous().token_type) {
                let rhs = match self.unary() {
                    Ok(u) => Box::new(u),
                    Err(e) => return Err(e),
                };
                return Ok(Expr::Unary { op, rhs });
            }
        }

        match self.primary() {
            Ok(p) => Ok(p),
            Err(e) => Err(()),
        }
    }
    fn primary(&mut self) -> Result<Expr, ()> {
        // let types = [
        //     TokenType::Num,
        //     TokenType::Nil,
        //     TokenType::True,
        //     TokenType::False,
        // ];
        // if self.consume_first_match(&types) {
        //     let t = self.previous();
        //     match t.token_type {
        //         TokenType::Num => Ok(Expr::LiteralExpr(Literal::Float(
        //             t.lexeme.parse::<f32>().unwrap(),
        //         ))),
        //         TokenType::Nil => Ok(Expr::LiteralExpr(Literal::Nil)),
        //         TokenType::True => Ok(Expr::LiteralExpr(Literal::Bool(true))),
        //         TokenType::False => Ok(Expr::LiteralExpr(Literal::Bool(false))),
        //         _ => Err(()),
        //     }
        // } else {
        //     Err(())
        // }

        if self.consume_match(TokenType::Num) {
            let t = self.previous();
            Ok(Expr::LiteralExpr(Literal::Float(
                t.lexeme.parse::<f32>().unwrap(),
            )))
        } else if self.consume_match(TokenType::Nil) {
            let t = self.previous();
            Ok(Expr::LiteralExpr(Literal::Nil))
        } else if self.consume_match(TokenType::True) {
            let t = self.previous();
            Ok(Expr::LiteralExpr(Literal::Bool(true)))
        } else if self.consume_match(TokenType::False) {
            let t = self.previous();
            Ok(Expr::LiteralExpr(Literal::Bool(false)))
        } else {
            Err(())
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = Expr;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_idx < self.tokens.len() {
            return Some(self.expression());
        }
        None
    }
}
