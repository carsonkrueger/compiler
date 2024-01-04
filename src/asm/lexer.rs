use std::{collections::VecDeque, fs::File, io::BufReader};

use crate::asm::{
    patterns::Patterns,
    token::{Token, TokenType},
};

pub struct Lexer {
    buf: BufReader<File>,
    unprocessed_lexeme: VecDeque<String>,
    line_index: usize,
    patterns: Patterns,
}

impl Lexer {
    pub fn new(file_path: &String) -> Self {
        Self {
            buf: BufReader::new(File::open(file_path).expect("Could not open file")),
            unprocessed_lexeme: VecDeque::new(),
            line_index: 0,
            patterns: Patterns::new(),
        }
    }
    fn split_line_into_lexeme(&mut self, line: &String) {
        let mut lexeme_iter = self.patterns.any.find_iter(&line);
        let mut m = lexeme_iter.next();
        while m.is_some() {
            self.unprocessed_lexeme
                .push_back(m.unwrap().as_str().to_owned());
            m = lexeme_iter.next();
        }
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
    fn lexeme_type(&self, lexeme: &String) -> Option<TokenType> {
        if self.patterns.label.is_match(&lexeme) {
            Some(TokenType::label)
        }
        if self.patterns.i_int.is_match(&lexeme) {
            Some(TokenType::i_int)
        }
        if self.patterns.i_str.is_match(&lexeme) {
            Some(TokenType::i_str)
        }
        if self.patterns.i_char.is_match(&lexeme) {
            Some(TokenType::i_char)
        }
        if self.patterns.comma.is_match(&lexeme) {
            Some(TokenType::comma)
        }
        if self.patterns.int_dir.is_match(&lexeme) {
            Some(TokenType::int_dir)
        }
        if self.patterns.byt_dir.is_match(&lexeme) {
            Some(TokenType::byt_dir)
        }
        if self.patterns.str_dir.is_match(&lexeme) {
            Some(TokenType::str_dir)
        }
        if self.patterns.rg.is_match(&lexeme) {
            Some(TokenType::rg)
        }
        if self.patterns.pc.is_match(&lexeme) {
            Some(TokenType::pc)
        }
        if self.patterns.jmp.is_match(&lexeme) {
            Some(TokenType::jmp)
        }
        if self.patterns.jmr.is_match(&lexeme) {
            Some(TokenType::jmr)
        }
        if self.patterns.bnz.is_match(&lexeme) {
            Some(TokenType::bnz)
        }
        if self.patterns.bgt.is_match(&lexeme) {
            Some(TokenType::bgt)
        }
        if self.patterns.blt.is_match(&lexeme) {
            Some(TokenType::blt)
        }
        if self.patterns.brz.is_match(&lexeme) {
            Some(TokenType::brz)
        }
        if self.patterns.bal.is_match(&lexeme) {
            Some(TokenType::bal)
        }
        if self.patterns.mov.is_match(&lexeme) {
            Some(TokenType::mov)
        }
        if self.patterns.movi.is_match(&lexeme) {
            Some(TokenType::movi)
        }
        if self.patterns.lda.is_match(&lexeme) {
            Some(TokenType::lda)
        }
        if self.patterns.str.is_match(&lexeme) {
            Some(TokenType::str)
        }
        if self.patterns.ldr.is_match(&lexeme) {
            Some(TokenType::ldr)
        }
        if self.patterns.stb.is_match(&lexeme) {
            Some(TokenType::stb)
        }
        if self.patterns.ldb.is_match(&lexeme) {
            Some(TokenType::ldb)
        }
        if self.patterns.push.is_match(&lexeme) {
            Some(TokenType::push)
        }
        if self.patterns.pop.is_match(&lexeme) {
            Some(TokenType::pop)
        }
        if self.patterns.peek.is_match(&lexeme) {
            Some(TokenType::peek)
        }
        if self.patterns.and.is_match(&lexeme) {
            Some(TokenType::and)
        }
        if self.patterns.or.is_match(&lexeme) {
            Some(TokenType::or)
        }
        if self.patterns.not.is_match(&lexeme) {
            Some(TokenType::not)
        }
        if self.patterns.cmp.is_match(&lexeme) {
            Some(TokenType::cmp)
        }
        if self.patterns.cmpi.is_match(&lexeme) {
            Some(TokenType::cmpi)
        }
        if self.patterns.add.is_match(&lexeme) {
            Some(TokenType::add)
        }
        if self.patterns.adi.is_match(&lexeme) {
            Some(TokenType::adi)
        }
        if self.patterns.sub.is_match(&lexeme) {
            Some(TokenType::sub)
        }
        if self.patterns.mul.is_match(&lexeme) {
            Some(TokenType::mul)
        }
        if self.patterns.muli.is_match(&lexeme) {
            Some(TokenType::muli)
        }
        if self.patterns.div.is_match(&lexeme) {
            Some(TokenType::div)
        }
        if self.patterns.divi.is_match(&lexeme) {
            Some(TokenType::divi)
        }
        if self.patterns.alci.is_match(&lexeme) {
            Some(TokenType::acli)
        }
        if self.patterns.allc.is_match(&lexeme) {
            Some(TokenType::allc)
        }
        if self.patterns.trp.is_match(&lexeme) {
            Some(TokenType::trp)
        } else {
            None
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        let lexeme = self.next_lexeme().unwrap().as_str().to_owned();
        let token_type = self.lexeme_type(&lexeme);
        None
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        while self.unprocessed_lexeme.is_empty() {
            let line = match self.next_line() {
                Some(l) => l,
                None => return None,
            };
            self.split_line_into_lexeme(&line);
        }

        let lexeme = match self.next_lexeme() {
            Some(l) => l,
            None => return None,
        };

        Some(Token {
            lexeme: lexeme.to_owned(),
            token_type: match self.lexeme_type(&lexeme) {
                Some(t) => t,
                None => panic!("Invalid token: {} on line: {}", lexeme, self.line_index),
            },
            line: self.line_index,
        })
    }
}
