use std::fs::File;

use crate::vm::{instruction::Instruction, opcode::Opcode};

use super::{
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
    fn new(tokens: &'a Vec<Token>, file_name: String) -> Self {
        Self {
            tokens,
            cur_idx: 0,
            symbol_table: SymbolTable::new(),
            file_name,
        }
    }
    fn pass_one(&mut self) {
        while !self.reached_eof() {
            if self.consume_match(TokenType::Label) {
                let label_token = match self.previous() {
                    Some(t) => t,
                    None => continue,
                };
                let symbol = Symbol::new(label_token.clone(), self.consume_num_bytes_dir());
                self.symbol_table.insert(&symbol);
            }
        }
    }
    fn pass_two(&mut self) {
        let mut writer = File::create(self.file_name.clone() + &String::from(".bin"))
            .expect("Could not create binary file");
        loop {
            if let Some(directive) = self.consume_next_directive() {
                match directive.write(&mut writer) {
                    Err(e) => panic!(
                        "Error writing directive {:?}\nTo file: {}",
                        directive, self.file_name
                    ),
                    Ok(_) => (),
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
                    Ok(_) => (),
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
            Some(&self.tokens[self.cur_idx])
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
        let opcode_type = if self.consume_first_match(&token_types) {
            match self.previous() {
                Some(t) => t.token_type.clone(),
                None => panic!("Expected token not found"),
            }
        } else {
            return Err(AssemblerErr::ExpectedOpcode);
        };
        match opcode_type {
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
            TokenType::Jmr => {
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
            | TokenType::Bgt
            | TokenType::Bgt
            | TokenType::Bgt
            | TokenType::Bal => {
                let op_1 = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => t.clone(),
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                };
                let rs = match Self::i32_try_from_rg_str(&op_1.lexeme) {
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
                let opcode = match Opcode::try_from(opcode_type) {
                    Ok(o) => o,
                    Err(_) => return Err(AssemblerErr::ExpectedOpcode),
                };
                Ok(Instruction {
                    opcode,
                    op1: rs,
                    op2: offset as i32,
                })
            }
            // TokenType::Bgt,
            // TokenType::Blt,
            // TokenType::Brz,
            // TokenType::Bal,
            TokenType::Mov => {
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
                // let rs = if self.consume_match(TokenType::Rg) {
                //     match self.previous() {
                //         Some(t) => {
                //             match Self::from_rg_str(&t.lexeme) {
                //                 Ok(i) => i,
                //                 Err(_) => return Err(AssemblerErr::InvalidRg(op_1.lexeme.clone())),
                //             },
                //         }
                //         None => panic!("Expected token not found"),
                //     }
                // } else {
                //     return Err(AssemblerErr::ExpectedTokenType(TokenType::Rg));
                // };
                Ok(Instruction {
                    opcode: Opcode::Mov,
                    op1: 0,
                    op2: 0,
                })
            }
            // TokenType::Movi,
            // TokenType::Lda,
            // TokenType::Str,
            // TokenType::Ldr,
            // TokenType::Stb,
            // TokenType::Ldb,
            // TokenType::Push,
            // TokenType::Pop,
            // TokenType::Peek,
            // TokenType::And,
            // TokenType::Or,
            // TokenType::Not,
            // TokenType::Cmp,
            // TokenType::Cmpi,
            // TokenType::Add,
            // TokenType::Adi,
            // TokenType::Sub,
            // TokenType::Mul,
            // TokenType::Muli,
            // TokenType::Div,
            // TokenType::Divi,
            // TokenType::Alci,
            // TokenType::Allc,
            // TokenType::Trp,
            _ => return Err(AssemblerErr::InvalidToken),
        }
    }
    fn consume_num_bytes_dir(&mut self) -> usize {
        let token_types = [TokenType::BytDir, TokenType::IntDir, TokenType::StrDir];
        if !self.consume_first_match(&token_types) {
            return 0;
        }

        let dir_type = match self.previous() {
            Some(t) => t.clone(),
            None => return 0,
        };

        let token_types = [TokenType::CharImm, TokenType::IntImm, TokenType::StrImm];
        if self.consume_first_match(&token_types) {
            return match self.previous() {
                Some(t) => {
                    if t.token_type == TokenType::StrImm {
                        t.lexeme.len() - 2 as usize
                    } else {
                        match dir_type.token_type {
                            TokenType::BytDir => 1,
                            TokenType::IntDir => 4,
                            TokenType::StrDir => 1,
                            _ => 0,
                        }
                    }
                }
                None => 0,
            };
        }

        match dir_type.token_type {
            TokenType::BytDir => 1,
            TokenType::IntDir => 4,
            TokenType::StrDir => 1,
            _ => 0,
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
    InvalidToken,
    ExpectedOpcode,
    InvalidRg(String),
    NonexistentLabel(String),
}
