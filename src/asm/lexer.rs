use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

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
        if self.patterns.i_int.is_match(&lexeme) {
            Some(TokenType::IntImm)
        } else if self.patterns.i_str.is_match(&lexeme) {
            Some(TokenType::StrImm)
        } else if self.patterns.i_char.is_match(&lexeme) {
            Some(TokenType::CharImm)
        } else if self.patterns.comma.is_match(&lexeme) {
            Some(TokenType::Comma)
        } else if self.patterns.int_dir.is_match(&lexeme) {
            Some(TokenType::IntDir)
        } else if self.patterns.byt_dir.is_match(&lexeme) {
            Some(TokenType::BytDir)
        } else if self.patterns.str_dir.is_match(&lexeme) {
            Some(TokenType::StrDir)
        } else if self.patterns.rg.is_match(&lexeme) {
            Some(TokenType::Rg)
        } else if self.patterns.pc.is_match(&lexeme) {
            Some(TokenType::Pc)
        } else if self.patterns.sp.is_match(&lexeme) {
            Some(TokenType::Sp)
        } else if self.patterns.hp.is_match(&lexeme) {
            Some(TokenType::Hp)
        } else if self.patterns.jmp.is_match(&lexeme) {
            Some(TokenType::Jmp)
        } else if self.patterns.jmr.is_match(&lexeme) {
            Some(TokenType::Jmr)
        } else if self.patterns.bnz.is_match(&lexeme) {
            Some(TokenType::Bnz)
        } else if self.patterns.bgt.is_match(&lexeme) {
            Some(TokenType::Bgt)
        } else if self.patterns.blt.is_match(&lexeme) {
            Some(TokenType::Blt)
        } else if self.patterns.brz.is_match(&lexeme) {
            Some(TokenType::Brz)
        } else if self.patterns.bal.is_match(&lexeme) {
            Some(TokenType::Bal)
        } else if self.patterns.mov.is_match(&lexeme) {
            Some(TokenType::Mov)
        } else if self.patterns.movi.is_match(&lexeme) {
            Some(TokenType::Movi)
        } else if self.patterns.lda.is_match(&lexeme) {
            Some(TokenType::Lda)
        } else if self.patterns.str.is_match(&lexeme) {
            Some(TokenType::Str)
        } else if self.patterns.ldr.is_match(&lexeme) {
            Some(TokenType::Ldr)
        } else if self.patterns.stb.is_match(&lexeme) {
            Some(TokenType::Stb)
        } else if self.patterns.ldb.is_match(&lexeme) {
            Some(TokenType::Ldb)
        } else if self.patterns.push.is_match(&lexeme) {
            Some(TokenType::Push)
        } else if self.patterns.pop.is_match(&lexeme) {
            Some(TokenType::Pop)
        } else if self.patterns.peek.is_match(&lexeme) {
            Some(TokenType::Peek)
        } else if self.patterns.and.is_match(&lexeme) {
            Some(TokenType::And)
        } else if self.patterns.or.is_match(&lexeme) {
            Some(TokenType::Or)
        } else if self.patterns.not.is_match(&lexeme) {
            Some(TokenType::Not)
        } else if self.patterns.cmp.is_match(&lexeme) {
            Some(TokenType::Cmp)
        } else if self.patterns.cmpi.is_match(&lexeme) {
            Some(TokenType::Cmpi)
        } else if self.patterns.add.is_match(&lexeme) {
            Some(TokenType::Add)
        } else if self.patterns.adi.is_match(&lexeme) {
            Some(TokenType::Adi)
        } else if self.patterns.sub.is_match(&lexeme) {
            Some(TokenType::Sub)
        } else if self.patterns.mul.is_match(&lexeme) {
            Some(TokenType::Mul)
        } else if self.patterns.muli.is_match(&lexeme) {
            Some(TokenType::Muli)
        } else if self.patterns.div.is_match(&lexeme) {
            Some(TokenType::Div)
        } else if self.patterns.divi.is_match(&lexeme) {
            Some(TokenType::Divi)
        } else if self.patterns.alci.is_match(&lexeme) {
            Some(TokenType::Alci)
        } else if self.patterns.allc.is_match(&lexeme) {
            Some(TokenType::Allc)
        } else if self.patterns.trp.is_match(&lexeme) {
            Some(TokenType::Trp)
        } else if self.patterns.comment.is_match(&lexeme) {
            Some(TokenType::Comment)
        } else if self.patterns.label.is_match(&lexeme) {
            Some(TokenType::Label)
        } else if self.patterns.label_op.is_match(&lexeme) {
            Some(TokenType::LabelOp)
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
        // while self.unprocessed_lexeme.is_empty() {
        //     let line = match self.next_line() {
        //         Some(l) => l,
        //         None => return None,
        //     };
        //     self.split_line_into_lexeme(&line);
        // }

        let lexeme = loop {
            match self.next_lexeme() {
                Some(l) => match self.lexeme_type(&l) {
                    Some(TokenType::Comment) => continue,
                    Some(t) => break l,
                    None => panic!("Invalid token: {} on line: {}", l, self.line_index),
                },
                None => {
                    let line = match self.next_line() {
                        Some(l) => l,
                        None => return None,
                    };
                    self.split_line_into_lexeme(&line);
                }
            }
        };

        if let Some(l) = self.lexeme_type(&lexeme) {
            match l {
                TokenType::Comment => return None,
                _ => (),
            };
        }

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
