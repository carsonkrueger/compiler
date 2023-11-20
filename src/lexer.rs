use regex::{Matches, Regex};

use crate::token::Token;
use crate::{patterns::Patterns, token_type::TokenType};
use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Lexer<'r, 'h> {
    tokens: Rc<RefCell<Vec<Token>>>,
    buf: Rc<RefCell<io::BufReader<File>>>,
    lexeme_iter: Rc<RefCell<Matches<'r, 'h>>>,
    line_index: Cell<usize>,
    patterns: Patterns,
}

impl Lexer<'_, '_> {
    pub fn new(file_path: &String) -> Self {
        // let mut buf = io::BufReader::new(
        //     File::open(file_path).expect(format!("Could not open file: {}", file_path).as_str()),
        // );

        // let mut s = String::new();
        // buf.read_line(&mut s);

        Self {
            tokens: Rc::new(RefCell::new(Vec::new())),
            buf: Rc::new(RefCell::new(io::BufReader::new(
            File::open(file_path).expect(format!("Could not open file: {}", file_path).as_str()),
        ))),
            lexeme_iter: Rc::new(RefCell::new(Regex::new(r#"(\(|\)|\{|\}|,|-?([0-9]+\.[0-9]+|[0-9]+)|\.|-|\+|;|\/|\*|!|!=|=|==|>|>=|<|<=|".*"|[a-zA-z_][a-zA-z_0-9]*|\S+)"#).unwrap().find_iter(&s).into())),
            line_index: Cell::new(1),
            patterns: Patterns::new(),
        }
    }
    fn add_token(&self, token: Token) {
        self.tokens.borrow_mut().push(token);
    }
    fn next_line(&self) -> Option<String> {
        let mut s = String::new();
        if let Ok(0) = self.buf.borrow_mut().read_line(&mut s) {
            return None;
        }
        self.line_index.set(self.line_index.get() + 1);
        Some(s)
    }
    fn next_lexeme(&self) -> Option<regex::Match<'_>> {
        self.lexeme_iter.borrow_mut().deref_mut().next()
    }
    pub fn next(&self) -> Option<TokenType> {
        let token = match self.next_lexeme() {
            Some(m) => m.as_str(),
            None => return None,
        };

        // let token_type: Option<TokenType> = {
        if self.patterns.l_paren.is_match(&token) {
            Some(TokenType::LParen)
        } else if self.patterns.r_paren.is_match(&token) {
            Some(TokenType::RParen)
        } else if self.patterns.l_brace.is_match(&token) {
            Some(TokenType::LBrace)
        } else if self.patterns.r_brace.is_match(&token) {
            Some(TokenType::RBrace)
        } else if self.patterns.comma.is_match(&token) {
            Some(TokenType::Comma)
        } else if self.patterns.dot.is_match(&token) {
            Some(TokenType::Dot)
        } else if self.patterns.minus.is_match(&token) {
            Some(TokenType::Minus)
        } else if self.patterns.plus.is_match(&token) {
            Some(TokenType::Plus)
        } else if self.patterns.semi.is_match(&token) {
            Some(TokenType::Semi)
        } else if self.patterns.slash.is_match(&token) {
            Some(TokenType::Slash)
        } else if self.patterns.star.is_match(&token) {
            Some(TokenType::Star)
        } else if self.patterns.bang.is_match(&token) {
            Some(TokenType::Bang)
        } else if self.patterns.bang_eq.is_match(&token) {
            Some(TokenType::BangEq)
        } else if self.patterns.eq.is_match(&token) {
            Some(TokenType::Eq)
        } else if self.patterns.eq_eq.is_match(&token) {
            Some(TokenType::EqEq)
        } else if self.patterns.gt.is_match(&token) {
            Some(TokenType::Gt)
        } else if self.patterns.gt_eq.is_match(&token) {
            Some(TokenType::GtEq)
        } else if self.patterns.lt.is_match(&token) {
            Some(TokenType::Lt)
        } else if self.patterns.lt_eq.is_match(&token) {
            Some(TokenType::LtEq)
        } else if self.patterns.str.is_match(&token) {
            Some(TokenType::Str)
        } else if self.patterns.num.is_match(&token) {
            Some(TokenType::Num)
        } else if self.patterns.and.is_match(&token) {
            Some(TokenType::And)
        } else if self.patterns.or.is_match(&token) {
            Some(TokenType::Or)
        } else if self.patterns.class.is_match(&token) {
            Some(TokenType::Class)
        } else if self.patterns.else_.is_match(&token) {
            Some(TokenType::Else)
        } else if self.patterns.if_.is_match(&token) {
            Some(TokenType::If)
        } else if self.patterns.identifier.is_match(&token) {
            Some(TokenType::Identifier)
        } else {
            None
        }
        // };
    }
}
