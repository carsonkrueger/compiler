use regex::Regex;

use crate::asm::token::{Token, TokenType};
use std::io::{Seek, Write};

#[derive(Debug)]
pub enum Directive {
    Byt(u8),
    Int(i32),
    Str(String),
}

impl Directive {
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.seek(std::io::SeekFrom::End(0))?;
        let bytes = match &self {
            Directive::Int(i) => i.to_le_bytes().as_slice().to_owned(),
            Directive::Byt(b) => b.to_le_bytes().as_slice().to_owned(),
            Directive::Str(s) => s.as_bytes().to_owned(),
        };
        writer.write_all(&bytes)?;
        Ok(())
    }
    pub fn try_from(dir_type: &Token, dir_value: Option<&Token>) -> Result<Self, ()> {
        match dir_type.token_type {
            TokenType::BytDir => {
                if let Some(t) = dir_value {
                    match t.token_type {
                        TokenType::CharImm => match Directive::parse_char(t.lexeme.clone()) {
                            Ok(i) => Ok(Directive::Byt(i)),
                            Err(_) => Err(()),
                        },
                        TokenType::IntImm => match Directive::parse_int(t.lexeme.clone()) {
                            Ok(i) => Ok(Directive::Byt(i as u8)),
                            Err(_) => Err(()),
                        },
                        _ => Err(()),
                    }
                } else {
                    Ok(Directive::Byt(0))
                }
            }
            TokenType::IntDir => {
                if let Some(t) = dir_value {
                    match t.token_type {
                        TokenType::CharImm => match Directive::parse_char(t.lexeme.clone()) {
                            Ok(i) => Ok(Directive::Int(i as i32)),
                            Err(_) => Err(()),
                        },
                        TokenType::IntImm => match Directive::parse_int(t.lexeme.clone()) {
                            Ok(i) => Ok(Directive::Int(i)),
                            Err(_) => Err(()),
                        },
                        _ => Err(()),
                    }
                } else {
                    Ok(Directive::Byt(0))
                }
            }
            TokenType::StrImm => {
                if let Some(t) = dir_value {
                    match t.token_type {
                        TokenType::StrImm => Ok(Directive::Str(t.lexeme.replace("\"", ""))),
                        _ => Err(()),
                    }
                } else {
                    Ok(Directive::Byt(0))
                }
            }
            _ => Err(()),
        }
    }
    fn parse_char(string: String) -> Result<u8, ()> {
        let pattern = Regex::new(r"[^']+").unwrap();
        let m = match pattern.find(&string) {
            Some(m) => m,
            None => return Err(()),
        };
        match m.as_str().parse::<u8>() {
            Ok(i) => Ok(i),
            Err(_) => Err(()),
        }
    }
    fn parse_int(string: String) -> Result<i32, ()> {
        let pattern = Regex::new(r"-?[0-9]+").unwrap();
        let m = match pattern.find(&string) {
            Some(m) => m,
            None => return Err(()),
        };
        match m.as_str().parse::<i32>() {
            Ok(i) => Ok(i),
            Err(_) => Err(()),
        }
    }
    // fn parse_str(string: String) -> String {
    //     let pattern = Regex::new
    // }
}
