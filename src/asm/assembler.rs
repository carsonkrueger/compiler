use core::panic;
use std::{f32::consts::PI, fs::File};

use crate::vm::{instruction::Instruction, opcode::Opcode};

use crate::asm::{
    directive::Directive,
    symbol::Symbol,
    symbol_table::SymbolTable,
    token::{Token, TokenType},
};

pub struct Assembler<'a> {
    tokens: &'a Vec<Token>,
    cur_idx: usize,
    symbol_table: SymbolTable,
    file_name: String,
}

impl<'a> Assembler<'a> {
    pub fn new(tokens: &'a Vec<Token>, file_name: &String) -> Self {
        Self {
            tokens,
            cur_idx: 0,
            symbol_table: SymbolTable::new(),
            file_name: file_name.clone(),
        }
    }
    pub fn run(&mut self) {
        self.pass_one();
        self.pass_two();
    }
    fn pass_one(&mut self) {
        println!("starting pass one");
        let mut lc = 4;
        while !self.reached_eof() {
            if self.consume_match(TokenType::Label) {
                let label_token = match self.previous() {
                    Some(t) => t,
                    None => continue,
                };
                let symbol = Symbol::new(label_token.clone(), lc);
                self.symbol_table.insert(&symbol);
                println!("inserted: {:?}", symbol);
            } else {
                self.advance();
            }
            let num = self.consume_num_bytes();
            println!("adding {}", num);
            lc += num;
        }
        println!("finished pass one")
    }
    fn pass_two(&mut self) {
        println!("starting pass two");
        self.reset();
        let mut writer = File::create(self.file_name.clone() + &String::from(".bin"))
            .expect("Could not create binary file");
        loop {
            if let Some(directive) = self.consume_next_directive() {
                match directive.write(&mut writer) {
                    Err(e) => panic!(
                        "Error writing directive {:?}\nTo file: {}",
                        directive, self.file_name
                    ),
                    Ok(_) => println!("wrote directive {:?}", directive),
                }
                continue;
            }
            break;
        }
        loop {
            if let Ok(instruction) = self.consume_next_instruction() {
                match instruction.write(&mut writer) {
                    Err(e) => panic!(
                        "Error writing instruction {:?} to {}",
                        instruction, self.file_name
                    ),
                    Ok(_) => println!("wrote instruction {:?}", instruction),
                }
                continue;
            }
            break;
        }
    }
    fn advance(&mut self) {
        self.cur_idx += 1;
    }
    fn retract(&mut self) {
        self.cur_idx -= 1;
    }
    fn reset(&mut self) {
        self.cur_idx = 0;
    }
    fn reached_eof(&self) -> bool {
        self.cur_idx >= self.tokens.len()
    }
    fn peek(&self) -> Option<&Token> {
        if self.cur_idx < self.tokens.len() {
            Some(&self.tokens[self.cur_idx])
        } else {
            None
        }
    }
    fn previous(&self) -> Option<&Token> {
        if self.cur_idx > 0 || self.cur_idx <= self.tokens.len() {
            Some(&self.tokens[self.cur_idx - 1])
        } else {
            None
        }
    }
    fn consume_match(&mut self, token_type: TokenType) -> bool {
        let bool = match self.peek() {
            Some(t) => t.token_type == token_type,
            None => false,
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
                Some(t) => t.token_type == *typ,
                None => return false,
            };

            if bool {
                self.advance();
                return true;
            }
        }
        false
        // let typ = match self.peek() {
        //     Some(t) => t.token_type.to_owned(),
        //     None => return false,
        // };
        // token_types.iter().find(|x| **x == typ).is_some()
    }
    /// Does NOT consume, returning true if the current token matches
    fn peek_match(&mut self, token_type: TokenType) -> bool {
        match self.peek() {
            Some(t) => t.token_type == token_type,
            None => false,
        }
    }
    /// Does NOT consume, returning true if the current token matches any of the token_types
    fn peek_first_match(&mut self, token_types: &[TokenType]) -> bool {
        for typ in token_types {
            let bool = match self.peek() {
                Some(t) => t.token_type == *typ,
                None => return false,
            };
            if bool {
                return true;
            }
        }
        false
    }
    fn consume_next_directive(&mut self) -> Option<Directive> {
        let found_label = self.consume_match(TokenType::Label);

        let token_types = [TokenType::BytDir, TokenType::IntDir, TokenType::StrDir];
        let dir_type = if self.consume_first_match(&token_types) {
            match self.previous() {
                Some(t) => t.clone(),
                None => panic!("Could not fetch previous token in: next_directive()"),
            }
        } else {
            if found_label {
                self.retract();
            }
            return None;
        };

        let token_types = [TokenType::CharImm, TokenType::IntImm, TokenType::StrImm];
        let dir_value = if self.consume_first_match(&token_types) {
            self.previous()
        } else {
            None
        };

        match Directive::try_from(&dir_type, dir_value) {
            Ok(d) => Some(d),
            Err(_) => panic!(
                "Could not create directive from: {:?} and {:?}",
                dir_type, dir_value
            ),
        }
    }
    fn consume_next_instruction(&mut self) -> Result<Instruction, AssemblerErr> {
        self.consume_match(TokenType::Label);
        let token_types = [
            TokenType::Jmp,
            TokenType::Jmr,
            TokenType::Bnz,
            TokenType::Bgt,
            TokenType::Blt,
            TokenType::Brz,
            TokenType::Bal,
            TokenType::Mov,
            TokenType::Movi,
            TokenType::Lda,
            TokenType::Str,
            TokenType::Ldr,
            TokenType::Stb,
            TokenType::Ldb,
            TokenType::Push,
            TokenType::Pop,
            TokenType::Peek,
            TokenType::And,
            TokenType::Or,
            TokenType::Not,
            TokenType::Cmp,
            TokenType::Cmpi,
            TokenType::Add,
            TokenType::Adi,
            TokenType::Sub,
            TokenType::Mul,
            TokenType::Muli,
            TokenType::Div,
            TokenType::Divi,
            TokenType::Alci,
            TokenType::Allc,
            TokenType::Trp,
        ];
        let opcode_token = if self.consume_first_match(&token_types) {
            match self.previous() {
                Some(t) => t.clone(),
                None => panic!("Expected token not found"),
            }
        } else {
            return Err(AssemblerErr::ExpectedOpcode);
        };
        match opcode_token.token_type {
            TokenType::Jmp => {
                let op_1 = if self.consume_match(TokenType::LabelOp) {
                    match self.previous() {
                        Some(t) => t,
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::LabelOp));
                };
                let offset = match self.symbol_table.get(&op_1.lexeme) {
                    Some(s) => s.offset,
                    None => return Err(AssemblerErr::NonexistentLabel(op_1.lexeme.clone())),
                };
                Ok(Instruction {
                    opcode: Opcode::Jmp,
                    op1: offset as i32,
                    op2: 0,
                })
            }
            TokenType::Jmr | TokenType::Push | TokenType::Peek | TokenType::Pop => {
                let op_1 = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => t,
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                };
                let rg = match Self::i32_try_from_rg_str(&op_1.lexeme) {
                    Ok(i) => i,
                    Err(_) => return Err(AssemblerErr::InvalidRg(op_1.lexeme.clone())),
                };
                Ok(Instruction {
                    opcode: Opcode::Jmr,
                    op1: rg,
                    op2: 0,
                })
            }
            TokenType::Bnz
            | TokenType::Bgt
            | TokenType::Blt
            | TokenType::Brz
            | TokenType::Bal
            | TokenType::Lda => {
                let op_1 = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => t.clone(),
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                };
                let rg = match Self::i32_try_from_rg_str(&op_1.lexeme) {
                    Ok(i) => i,
                    Err(_) => return Err(AssemblerErr::InvalidRg(op_1.lexeme.clone())),
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Comma));
                }
                let op_2 = if self.consume_match(TokenType::LabelOp) {
                    match self.previous() {
                        Some(t) => t,
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::LabelOp));
                };
                let offset = match self.symbol_table.get(&op_2.lexeme) {
                    Some(o) => o.offset,
                    None => return Err(AssemblerErr::NonexistentLabel(op_2.lexeme.clone())),
                };
                let opcode = match Opcode::try_from(opcode_token.token_type) {
                    Ok(o) => o,
                    Err(_) => return Err(AssemblerErr::ExpectedOpcode),
                };
                Ok(Instruction {
                    opcode,
                    op1: rg,
                    op2: offset as i32,
                })
            }
            // TokenType::Bgt,
            // TokenType::Blt,
            // TokenType::Brz,
            // TokenType::Bal,
            TokenType::Mov
            | TokenType::And
            | TokenType::Or
            | TokenType::Not
            | TokenType::Cmp
            | TokenType::Allc
            | TokenType::Add
            | TokenType::Sub
            | TokenType::Mul
            | TokenType::Div => {
                let opcode = match Opcode::try_from(opcode_token.token_type) {
                    Ok(o) => o,
                    Err(_) => return Err(AssemblerErr::ExpectedOpcode),
                };
                let rd = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => match Self::i32_try_from_rg_str(&t.lexeme) {
                            Ok(i) => i,
                            Err(_) => return Err(AssemblerErr::InvalidRg(t.lexeme.clone())),
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Comma));
                }
                let rs = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => match Self::i32_try_from_rg_str(&t.lexeme) {
                            Ok(i) => i,
                            Err(_) => return Err(AssemblerErr::InvalidRg(t.lexeme.clone())),
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                };
                Ok(Instruction {
                    opcode,
                    op1: rd,
                    op2: rs,
                })
            }
            TokenType::Movi
            | TokenType::Cmpi
            | TokenType::Adi
            | TokenType::Muli
            | TokenType::Divi
            | TokenType::Alci => {
                let rd = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => match Self::i32_try_from_rg_str(&t.lexeme) {
                            Ok(i) => i,
                            Err(_) => return Err(AssemblerErr::InvalidRg(t.lexeme.clone())),
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Comma));
                }
                let token_types = [TokenType::IntImm, TokenType::CharImm];
                let imm = if self.consume_first_match(&token_types) {
                    let imm_token = match self.previous() {
                        Some(t) => t,
                        None => return Err(AssemblerErr::ExpectedTokenType(TokenType::IntImm)),
                    };
                    match imm_token.token_type {
                        TokenType::IntImm => {
                            match Self::i32_try_from_int_imm_str(&imm_token.lexeme) {
                                Ok(i) => i,
                                Err(_) => {
                                    return Err(AssemblerErr::InvalidToken(imm_token.clone()))
                                }
                            }
                        }
                        TokenType::CharImm => {
                            match Self::i32_try_from_char_imm_str(&imm_token.lexeme) {
                                Ok(i) => i,
                                Err(_) => {
                                    return Err(AssemblerErr::InvalidToken(imm_token.clone()))
                                }
                            }
                        }
                        _ => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::IntImm));
                };
                Ok(Instruction {
                    opcode: Opcode::Movi,
                    op1: rd,
                    op2: imm,
                })
            }
            TokenType::Str | TokenType::Ldr | TokenType::Stb | TokenType::Ldb => {
                let mut opcode = Opcode::Str;
                let t_1 = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => t,
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                };
                let op_1 = match Self::i32_try_from_rg_str(&opcode_token.lexeme) {
                    Ok(i) => i,
                    Err(_) => return Err(AssemblerErr::InvalidRg(opcode_token.lexeme.clone())),
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Comma));
                }
                let token_types = [TokenType::LabelOp, TokenType::Rg];
                let op_2 = if self.consume_first_match(&token_types) {
                    match self.previous() {
                        Some(t) => {
                            opcode = match Opcode::try_from((
                                opcode_token.token_type,
                                t.token_type.clone(),
                            )) {
                                Ok(o) => o,
                                Err(_) => return Err(AssemblerErr::ExpectedOpcode),
                            };
                            match t.token_type {
                                TokenType::LabelOp => {
                                    self.symbol_table.get_expect(&t.lexeme).offset as i32
                                }
                                TokenType::Rg => match Self::i32_try_from_rg_str(&t.lexeme) {
                                    Ok(i) => i,
                                    Err(e) => {
                                        return Err(AssemblerErr::InvalidRg(t.lexeme.clone()))
                                    }
                                },
                                _ => {
                                    return Err(AssemblerErr::ExpectedTokenType(TokenType::LabelOp))
                                }
                            }
                        }
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::LabelOp));
                };

                Ok(Instruction {
                    opcode,
                    op1: op_1,
                    op2: op_2,
                })
            }
            TokenType::Trp => {
                let op1 = if self.consume_match(TokenType::IntImm) {
                    match self.previous() {
                        Some(t) => match Self::i32_try_from_int_imm_str(&t.lexeme) {
                            Ok(i) => {
                                if i < 0 || i > 4 {
                                    return Err(AssemblerErr::InvalidToken(t.clone()));
                                }
                                i
                            }
                            Err(_) => {
                                return Err(AssemblerErr::ExpectedTokenType(TokenType::IntImm))
                            }
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::IntImm));
                };
                Ok(Instruction {
                    opcode: Opcode::Trp,
                    op1,
                    op2: 0,
                })
            }
            _ => return Err(AssemblerErr::InvalidToken(opcode_token)),
        }
    }
    fn consume_num_bytes(&mut self) -> usize {
        let dir_token_types = [TokenType::BytDir, TokenType::IntDir, TokenType::StrDir];
        let ins_token_types = [
            TokenType::Jmp,
            TokenType::Jmr,
            TokenType::Bnz,
            TokenType::Bgt,
            TokenType::Blt,
            TokenType::Brz,
            TokenType::Bal,
            TokenType::Mov,
            TokenType::Movi,
            TokenType::Lda,
            TokenType::Str,
            TokenType::Ldr,
            TokenType::Stb,
            TokenType::Ldb,
            TokenType::Push,
            TokenType::Pop,
            TokenType::Peek,
            TokenType::And,
            TokenType::Or,
            TokenType::Not,
            TokenType::Cmp,
            TokenType::Cmpi,
            TokenType::Add,
            TokenType::Adi,
            TokenType::Sub,
            TokenType::Mul,
            TokenType::Muli,
            TokenType::Div,
            TokenType::Divi,
            TokenType::Alci,
            TokenType::Allc,
            TokenType::Trp,
        ];
        // found directive token
        if self.consume_first_match(&dir_token_types) {
            let num_bytes = match self.previous() {
                Some(t) => {
                    println!("found directive: {:?}", t);
                    match t.token_type {
                        TokenType::BytDir => 1,
                        TokenType::IntDir => 4,
                        TokenType::StrDir => {
                            if self.consume_match(TokenType::StrImm) {
                                match self.previous() {
                                    Some(t2) => t2.lexeme.len() - 2 + 1, // - 2 for "" and + 1 for pascal byte
                                    None => 1,
                                }
                            } else {
                                1
                            }
                        }
                        _ => 0,
                    }
                }
                None => 0,
            };
            while !self.peek_first_match(&dir_token_types)
                && !self.peek_first_match(&ins_token_types)
                && !self.peek_match(TokenType::Label)
            {
                self.advance();
                if let Some(t) = self.previous() {
                    println!("consumed garbage: {:?}", t);
                }
            }
            num_bytes
        }
        // found opcode instruction token
        else if self.consume_first_match(&ins_token_types) {
            if let Some(t) = self.previous() {
                println!("found directive: {:?}", t);
            }
            while !self.peek_first_match(&dir_token_types)
                && !self.peek_first_match(&ins_token_types)
                && !self.peek_match(TokenType::Label)
            {
                self.advance();
                if let Some(t) = self.previous() {
                    println!("consumed garbage: {:?}", t);
                }
            }
            12
        } else {
            if let Some(t) = self.previous().to_owned() {
                println!("No matching byt or ins: {:?}", t);
            }
            0
        }
    }
    fn i32_try_from_rg_str(rg_str: &String) -> Result<i32, ()> {
        match rg_str.replace("R", "").parse::<i32>() {
            Ok(i) => Ok(i),
            Err(_) => Err(()),
        }
    }
    fn i32_try_from_char_imm_str(imm_str: &String) -> Result<i32, ()> {
        let mut imm_str = imm_str.clone();
        imm_str.remove(0);
        imm_str.remove(imm_str.len() - 1);
        match imm_str.parse::<char>() {
            Ok(ch) => Ok(ch as i32),
            Err(_) => Err(()),
        }
    }
    fn i32_try_from_int_imm_str(imm_str: &String) -> Result<i32, ()> {
        let mut imm_str = imm_str.clone();
        imm_str.remove(0);
        match imm_str.parse::<i32>() {
            Ok(i) => Ok(i),
            Err(_) => Err(()),
        }
    }
}

pub enum AssemblerErr {
    ExpectedTokenType(TokenType),
    InvalidToken(Token),
    ExpectedOpcode,
    InvalidRg(String),
    NonexistentLabel(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_match() {
        let tokens = vec![
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Jmp,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Ldb,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Label,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::BytDir,
                line: 0,
            },
        ];
        let mut asm = Assembler::new(&tokens, &String::from("test"));

        assert!(asm.consume_match(TokenType::Jmp));
        assert!(!asm.consume_match(TokenType::Label));
        assert!(asm.consume_match(TokenType::Ldb));
        assert!(asm.consume_match(TokenType::Label));
        assert!(!asm.consume_match(TokenType::Jmp));
        assert!(asm.consume_match(TokenType::BytDir));
        assert!(!asm.consume_match(TokenType::Stb));
    }

    #[test]
    fn test_consume_first_match() {
        let tokens = vec![
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Jmp,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Ldb,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Label,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::BytDir,
                line: 0,
            },
        ];
        let mut asm = Assembler::new(&tokens, &String::from("test"));

        assert!(asm.consume_first_match(&[TokenType::Jmp, TokenType::Label, TokenType::Add]));
        assert!(!asm.consume_first_match(&[TokenType::BytDir, TokenType::Label, TokenType::Add]));
        assert!(asm.consume_first_match(&[TokenType::Jmp, TokenType::Ldb, TokenType::Add]));
        assert!(asm.consume_first_match(&[TokenType::Jmp, TokenType::Label, TokenType::Add]));
        assert!(!asm.consume_first_match(&[TokenType::Jmp, TokenType::Label, TokenType::IntImm]));
        assert!(!asm.consume_first_match(&[
            TokenType::CharImm,
            TokenType::LabelOp,
            TokenType::IntImm
        ]));
        assert!(asm.consume_first_match(&[TokenType::Jmr, TokenType::BytDir, TokenType::Add]));
    }

    #[test]
    fn test_peek_match() {
        let tokens = vec![
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Jmp,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Ldb,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Label,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::BytDir,
                line: 0,
            },
        ];
        let mut asm = Assembler::new(&tokens, &String::from("test"));

        assert!(asm.peek_match(TokenType::Jmp));
        asm.advance();
        assert!(!asm.peek_match(TokenType::Label));
        assert!(asm.peek_match(TokenType::Ldb));
        asm.advance();
        assert!(asm.peek_match(TokenType::Label));
        asm.advance();
        assert!(!asm.peek_match(TokenType::LabelOp));
        assert!(!asm.peek_match(TokenType::IntDir));
        assert!(!asm.peek_match(TokenType::CharImm));
        assert!(asm.peek_match(TokenType::BytDir));
    }

    #[test]
    fn test_peek_first_match() {
        let tokens = vec![
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Jmp,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Ldb,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Label,
                line: 0,
            },
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::BytDir,
                line: 0,
            },
        ];
        let mut asm = Assembler::new(&tokens, &String::from("test"));

        assert!(asm.peek_first_match(&[TokenType::Jmp, TokenType::Label, TokenType::Add]));
        asm.advance();
        assert!(!asm.peek_first_match(&[TokenType::BytDir, TokenType::Label, TokenType::Add]));
        assert!(asm.peek_first_match(&[TokenType::Jmp, TokenType::Ldb, TokenType::Add]));
        asm.advance();
        assert!(asm.peek_first_match(&[TokenType::Jmp, TokenType::Label, TokenType::Add]));
        asm.advance();
        assert!(!asm.peek_first_match(&[TokenType::Jmp, TokenType::Label, TokenType::IntImm]));
        assert!(!asm.peek_first_match(&[
            TokenType::CharImm,
            TokenType::LabelOp,
            TokenType::IntImm
        ]));
        assert!(asm.consume_first_match(&[TokenType::Jmr, TokenType::BytDir, TokenType::Add]));
    }
}
