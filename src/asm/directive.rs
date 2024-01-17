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
        match &self {
            Directive::Str(s) => {
                writer.write(&[s.len() as u8]);
                ()
            }
            _ => (),
        };
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
                    Ok(Directive::Int(0))
                }
            }
            TokenType::StrDir => {
                if let Some(t) = dir_value {
                    match t.token_type {
                        TokenType::StrImm => {
                            Ok(Directive::Str(Directive::parse_str(t.lexeme.clone())))
                        }
                        _ => Err(()),
                    }
                } else {
                    Ok(Directive::Str(String::from("")))
                }
            }
            _ => Err(()),
        }
    }
    fn parse_char(mut string: String) -> Result<u8, ()> {
        string.remove(0);
        string.remove(string.len() - 1);
        match string.as_str() {
            r"\n" => return Ok('\n' as u8),
            r"\t" => return Ok('\t' as u8),
            r"\r" => return Ok('\r' as u8),
            _ => match string.parse::<char>() {
                Ok(i) => Ok(i as u8),
                Err(e) => {
                    println!("Error parsing char as u8: {}", e);
                    Err(())
                }
            },
        }
    }
    fn parse_int(mut string: String) -> Result<i32, ()> {
        string.remove(0);
        match string.parse::<i32>() {
            Ok(i) => Ok(i),
            Err(e) => {
                println!("Error parsing i32: {}", e);
                Err(())
            }
        }
    }
    fn parse_str(mut string: String) -> String {
        if string.len() == 2 {
            return String::from("");
        }
        string.remove(0);
        string.remove(string.len() - 1);
        string
    }
    // fn parse_str(string: String) -> String {
    //     let pattern = Regex::new
    // }
}
