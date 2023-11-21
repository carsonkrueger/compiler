use regex::Matches;

use crate::token::Token;
use crate::{patterns::Patterns, token_type::TokenType};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Lexer {
    tokens: Vec<Token>,
    buf: BufReader<File>,
    // file_contents: String,
    unprocessed_lexeme: VecDeque<String>,

    line_index: usize,
    patterns: Patterns,
}

impl Lexer {
    pub fn new(file_path: &String) -> Self {
        Self {
            tokens: Vec::new(),
            buf: BufReader::new(File::open(file_path).expect("Could not open file")),
            // file_contents: fs::read_to_string(file_path),
            // lexeme_iter: None,
            // cur_line: String::new(),
            unprocessed_lexeme: VecDeque::new(),
            line_index: 0,
            patterns: Patterns::new(),
        }
    }
    pub fn run(&mut self) {
        let mut cur_line = match self.next_line() {
            Some(l) => l.as_str().to_owned(),
            None => return (),
        };

        self.split_line(&cur_line);

        for lexeme in &self.unprocessed_lexeme {
            println!("{}", lexeme);
        }
    }
    fn split_line(&mut self, line: &String) {
        let mut lexeme_iter = self.patterns.any.find_iter(&line);
        let mut matches = lexeme_iter.next();

        // while !cur_match.is_none() {
        for m in matches {
            let cur_lexeme = m.as_str().to_owned();
            self.unprocessed_lexeme.push_back(cur_lexeme);
        }
    }
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn next_line(&mut self) -> Option<String> {
        let mut s = String::new();
        if let Ok(0) = self.buf.read_line(&mut s) {
            return None;
        }
        self.line_index += 1;
        Some(s)
    }
    fn next_lexeme(&mut self) -> Option<String> {
        self.unprocessed_lexeme.pop_front()
    }
    fn lexeme_type(&self, lexeme: String) -> Option<TokenType> {
        if self.patterns.l_paren.is_match(&lexeme) {
            Some(TokenType::LParen)
        } else if self.patterns.r_paren.is_match(&lexeme) {
            Some(TokenType::RParen)
        } else if self.patterns.l_brace.is_match(&lexeme) {
            Some(TokenType::LBrace)
        } else if self.patterns.r_brace.is_match(&lexeme) {
            Some(TokenType::RBrace)
        } else if self.patterns.comma.is_match(&lexeme) {
            Some(TokenType::Comma)
        } else if self.patterns.dot.is_match(&lexeme) {
            Some(TokenType::Dot)
        } else if self.patterns.minus.is_match(&lexeme) {
            Some(TokenType::Minus)
        } else if self.patterns.plus.is_match(&lexeme) {
            Some(TokenType::Plus)
        } else if self.patterns.semi.is_match(&lexeme) {
            Some(TokenType::Semi)
        } else if self.patterns.slash.is_match(&lexeme) {
            Some(TokenType::Slash)
        } else if self.patterns.star.is_match(&lexeme) {
            Some(TokenType::Star)
        } else if self.patterns.bang.is_match(&lexeme) {
            Some(TokenType::Bang)
        } else if self.patterns.bang_eq.is_match(&lexeme) {
            Some(TokenType::BangEq)
        } else if self.patterns.eq.is_match(&lexeme) {
            Some(TokenType::Eq)
        } else if self.patterns.eq_eq.is_match(&lexeme) {
            Some(TokenType::EqEq)
        } else if self.patterns.gt.is_match(&lexeme) {
            Some(TokenType::Gt)
        } else if self.patterns.gt_eq.is_match(&lexeme) {
            Some(TokenType::GtEq)
        } else if self.patterns.lt.is_match(&lexeme) {
            Some(TokenType::Lt)
        } else if self.patterns.lt_eq.is_match(&lexeme) {
            Some(TokenType::LtEq)
        } else if self.patterns.str.is_match(&lexeme) {
            Some(TokenType::Str)
        } else if self.patterns.num.is_match(&lexeme) {
            Some(TokenType::Num)
        } else if self.patterns.and.is_match(&lexeme) {
            Some(TokenType::And)
        } else if self.patterns.or.is_match(&lexeme) {
            Some(TokenType::Or)
        } else if self.patterns.class.is_match(&lexeme) {
            Some(TokenType::Class)
        } else if self.patterns.else_.is_match(&lexeme) {
            Some(TokenType::Else)
        } else if self.patterns.if_.is_match(&lexeme) {
            Some(TokenType::If)
        } else if self.patterns.identifier.is_match(&lexeme) {
            Some(TokenType::Identifier)
        } else {
            None
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        let lexeme = self.next_lexeme().unwrap().as_str().to_owned();
        let token_type = self.lexeme_type(lexeme);
        None
    }
}
