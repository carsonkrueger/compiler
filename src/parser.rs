use core::panic;

use crate::{
    ast::{BinaryOp, Expr, Literal, UnaryOp},
    token::{token_type::TokenType, Token},
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    cur_idx: usize,
}

pub enum ParseErr {
    InvalidExpr,
    MissingRParen,
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
    fn peek(&self) -> Result<&Token, ParseErr> {
        if self.cur_idx < self.tokens.len() {
            Ok(&self.tokens[self.cur_idx])
        } else {
            Err(ParseErr::InvalidExpr)
        }
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.cur_idx - 1]
    }
    fn advance(&mut self) {
        self.cur_idx += 1;
    }
    /// consumes and advances IF current token matches token_type argument. Returns true if successfully consumed token.
    fn consume_match(&mut self, token_type: TokenType) -> bool {
        let bool = match self.peek() {
            Ok(t) => t.token_type == token_type,
            Err(_) => false,
        };

        if bool {
            self.advance();
        }
        bool
    }
    /// consumes and advances IF current token matches any token_type in the token_types list argument. Returns true if successfully consumed token.
    fn consume_first_match(&mut self, token_types: &[TokenType]) -> bool {
        for typ in token_types {
            let bool = match self.peek() {
                Ok(t) => t.token_type == *typ,
                Err(_) => return false,
            };

            if bool {
                self.advance();
                return true;
            }
        }
        false
    }
    fn expression(&mut self) -> Result<Expr, ParseErr> {
        match self.equality() {
            Ok(e) => Ok(e),
            Err(e) => Err(e),
        }
    }
    fn equality(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = match self.comparison() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let mut op = BinaryOp::EqEq; // just made multiply default op. If no valid op is found, then returns Err
        let types = [TokenType::BangEq, TokenType::EqEq];

        while (self.consume_first_match(&types)) {
            op = match BinaryOp::try_from(&self.previous().token_type) {
                Ok(o) => o,
                Err(_) => return Err(ParseErr::InvalidExpr),
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
    fn comparison(&mut self) -> Result<Expr, ParseErr> {
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
                Err(_) => return Err(ParseErr::InvalidExpr),
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
    fn term(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = match self.factor() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let mut op = BinaryOp::Plus; // just made multiply default op. If no valid op is found, then returns Err
        let types = [TokenType::Minus, TokenType::Minus];

        while (self.consume_first_match(&types)) {
            op = match BinaryOp::try_from(&self.previous().token_type) {
                Ok(o) => o,
                Err(_) => return Err(ParseErr::InvalidExpr),
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
    fn factor(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = match self.unary() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let mut op = BinaryOp::Mult; // just made multiply default op. If no valid op is found, then returns Err
        let types = [TokenType::Slash, TokenType::Star];

        while (self.consume_first_match(&types)) {
            op = match BinaryOp::try_from(&self.previous().token_type) {
                Ok(o) => o,
                Err(_) => return Err(ParseErr::InvalidExpr),
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
    fn unary(&mut self) -> Result<Expr, ParseErr> {
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
            Err(e) => Err(e),
        }
    }
    fn primary(&mut self) -> Result<Expr, ParseErr> {
        if self.consume_match(TokenType::Num) {
            let t = self.previous();
            Ok(Expr::LiteralExpr(Literal::Float(
                t.lexeme.parse::<f32>().unwrap(),
            )))
        } else if self.consume_match(TokenType::Nil) {
            Ok(Expr::LiteralExpr(Literal::Nil))
        } else if self.consume_match(TokenType::True) {
            Ok(Expr::LiteralExpr(Literal::Bool(true)))
        } else if self.consume_match(TokenType::False) {
            Ok(Expr::LiteralExpr(Literal::Bool(false)))
        } else if self.consume_match(TokenType::LParen) {
            let expr = match self.expression() {
                Ok(e) => e,
                Err(e) => return Err(e),
            };
            if !self.consume_match(TokenType::RParen) {
                Err(ParseErr::MissingRParen)
            } else {
                Ok(Expr::Grouping(Box::new(expr)))
            }
        } else {
            Err(ParseErr::InvalidExpr)
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = Expr;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_idx < self.tokens.len() {
            match self.expression() {
                Ok(e) => return Some(e),
                Err(_) => return None,
            }
        }
        None
    }
}
