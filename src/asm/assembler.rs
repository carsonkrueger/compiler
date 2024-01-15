use core::panic;
use std::fmt::Display;
use std::io::Write;
use std::process::exit;
use std::{f32::consts::PI, fs::File};

use crate::vm::{instruction::Instruction, opcode::Opcode};

const INS_TOKEN_TYPES: [TokenType; 32] = [
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

const DIR_TOKEN_TYPES: [TokenType; 3] = [TokenType::BytDir, TokenType::IntDir, TokenType::StrDir];

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
    init_pc: i32,
    cur_line: usize,
}

impl<'a> Assembler<'a> {
    pub fn new(tokens: &'a Vec<Token>, file_name: &String) -> Self {
        Self {
            tokens,
            cur_idx: 0,
            symbol_table: SymbolTable::new(),
            file_name: file_name.clone(),
            init_pc: 4,
            cur_line: 0,
        }
    }
    pub fn run(&mut self) {
        self.pass_one();
        self.pass_two();
    }
    fn pass_one(&mut self) {
        let mut lc = 4;
        while !self.reached_eof() {
            if self.consume_match(TokenType::Label) {
                let mut label_token = match self.previous() {
                    Some(t) => t.clone(),
                    None => continue,
                };
                label_token.lexeme.remove(label_token.lexeme.len() - 1);
                let symbol = Symbol::new(label_token, lc);
                self.symbol_table.insert(&symbol);
            }
            // set init pc
            if self.peek_first_match(&INS_TOKEN_TYPES) && self.init_pc == 4 {
                self.init_pc = lc as i32;
            }
            let num = self.consume_num_bytes();
            lc += num;
        }
    }
    fn pass_two(&mut self) {
        self.reset();
        let mut writer = File::create(self.file_name.clone() + &String::from(".bin"))
            .expect("Could not create binary file");
        // write init pc
        writer.write_all(self.init_pc.to_le_bytes().as_slice());
        while !self.reached_eof() {
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
        while !self.reached_eof() {
            match self.consume_next_instruction() {
                Ok(instruction) => {
                    match instruction.write(&mut writer) {
                        Err(e) => panic!(
                            "Error writing instruction {:?} to {}",
                            instruction, self.file_name
                        ),
                        Ok(_) => (),
                    }
                    continue;
                }
                Err(e) => {
                    // println!("Error line: {}", self.peek().unwrap().line);
                    println!("Error creating instruction: {}", e);
                    exit(1);
                }
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
            self.cur_line = self.peek().expect("consume_match should not crash").line;
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
                self.cur_line = self.peek().expect("consume_match should not crash").line;
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
            Err(e) => panic!(
                "Could not create directive from: {:?} and {:?}",
                dir_type, dir_value
            ),
        }
    }
    fn consume_next_instruction(&mut self) -> Result<Instruction, AssemblerErr> {
        self.consume_match(TokenType::Label);
        let opcode_token = if self.consume_first_match(&INS_TOKEN_TYPES) {
            match self.previous() {
                Some(t) => t.clone(),
                None => panic!("Expected token not found"),
            }
        } else {
            return Err(AssemblerErr::ExpectedOpcode(self.cur_line));
        };
        match opcode_token.token_type {
            TokenType::Jmp => {
                let op_1 = if self.consume_match(TokenType::LabelOp) {
                    match self.previous() {
                        Some(t) => t,
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::LabelOp,
                        self.cur_line,
                    ));
                };
                let offset = match self.symbol_table.get(&op_1.lexeme) {
                    Some(s) => s.offset,
                    None => {
                        return Err(AssemblerErr::NonexistentLabel(
                            op_1.lexeme.clone(),
                            self.cur_line,
                        ))
                    }
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
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Rg,
                        self.cur_line,
                    ));
                };
                let rg = match Self::i32_try_from_rg_str(&op_1.lexeme) {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(AssemblerErr::InvalidRg(op_1.lexeme.clone(), self.cur_line))
                    }
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
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Rg,
                        self.cur_line,
                    ));
                };
                let rg = match Self::i32_try_from_rg_str(&op_1.lexeme) {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(AssemblerErr::InvalidRg(op_1.lexeme.clone(), self.cur_line))
                    }
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Comma,
                        self.cur_line,
                    ));
                }
                let op_2 = if self.consume_match(TokenType::LabelOp) {
                    match self.previous() {
                        Some(t) => t,
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::LabelOp,
                        self.cur_line,
                    ));
                };
                let offset = match self.symbol_table.get(&op_2.lexeme) {
                    Some(o) => o.offset,
                    None => {
                        return Err(AssemblerErr::NonexistentLabel(
                            op_2.lexeme.clone(),
                            self.cur_line,
                        ))
                    }
                };
                let opcode = match Opcode::try_from(opcode_token.token_type) {
                    Ok(o) => o,
                    Err(_) => return Err(AssemblerErr::ExpectedOpcode(self.cur_line)),
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
                    Err(_) => return Err(AssemblerErr::ExpectedOpcode(self.cur_line)),
                };
                let rd = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => match Self::i32_try_from_rg_str(&t.lexeme) {
                            Ok(i) => i,
                            Err(_) => {
                                return Err(AssemblerErr::InvalidRg(
                                    t.lexeme.clone(),
                                    self.cur_line,
                                ))
                            }
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Rg,
                        self.cur_line,
                    ));
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Comma,
                        self.cur_line,
                    ));
                }
                let rs = if self.consume_match(TokenType::Rg) {
                    match self.previous() {
                        Some(t) => match Self::i32_try_from_rg_str(&t.lexeme) {
                            Ok(i) => i,
                            Err(_) => {
                                return Err(AssemblerErr::InvalidRg(
                                    t.lexeme.clone(),
                                    self.cur_line,
                                ))
                            }
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Rg,
                        self.cur_line,
                    ));
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
                            Err(_) => {
                                return Err(AssemblerErr::InvalidRg(
                                    t.lexeme.clone(),
                                    self.cur_line,
                                ))
                            }
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Rg,
                        self.cur_line,
                    ));
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Comma,
                        self.cur_line,
                    ));
                }
                let token_types = [TokenType::IntImm, TokenType::CharImm];
                let imm = if self.consume_first_match(&token_types) {
                    let imm_token = match self.previous() {
                        Some(t) => t,
                        None => {
                            return Err(AssemblerErr::ExpectedTokenType(
                                TokenType::IntImm,
                                self.cur_line,
                            ))
                        }
                    };
                    match imm_token.token_type {
                        TokenType::IntImm => {
                            match Self::i32_try_from_int_imm_str(&imm_token.lexeme) {
                                Ok(i) => i,
                                Err(_) => {
                                    return Err(AssemblerErr::InvalidToken(
                                        imm_token.clone(),
                                        self.cur_line,
                                    ))
                                }
                            }
                        }
                        TokenType::CharImm => {
                            match Self::i32_try_from_char_imm_str(&imm_token.lexeme) {
                                Ok(i) => i,
                                Err(_) => {
                                    return Err(AssemblerErr::InvalidToken(
                                        imm_token.clone(),
                                        self.cur_line,
                                    ))
                                }
                            }
                        }
                        _ => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::IntImm,
                        self.cur_line,
                    ));
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
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Rg,
                        self.cur_line,
                    ));
                };
                let op_1 = match Self::i32_try_from_rg_str(&t_1.lexeme) {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(AssemblerErr::InvalidRg(
                            opcode_token.lexeme.clone(),
                            self.cur_line,
                        ))
                    }
                };
                if !self.consume_match(TokenType::Comma) {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::Comma,
                        self.cur_line,
                    ));
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
                                Err(_) => return Err(AssemblerErr::ExpectedOpcode(self.cur_line)),
                            };
                            match t.token_type {
                                TokenType::LabelOp => {
                                    self.symbol_table.get_expect(&t.lexeme).offset as i32
                                }
                                TokenType::Rg => match Self::i32_try_from_rg_str(&t.lexeme) {
                                    Ok(i) => i,
                                    Err(e) => {
                                        return Err(AssemblerErr::InvalidRg(
                                            t.lexeme.clone(),
                                            self.cur_line,
                                        ))
                                    }
                                },
                                _ => {
                                    return Err(AssemblerErr::ExpectedTokenType(
                                        TokenType::LabelOp,
                                        self.cur_line,
                                    ))
                                }
                            }
                        }
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::LabelOp,
                        self.cur_line,
                    ));
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
                                    return Err(AssemblerErr::InvalidToken(
                                        t.clone(),
                                        self.cur_line,
                                    ));
                                }
                                i
                            }
                            Err(_) => {
                                return Err(AssemblerErr::ExpectedTokenType(
                                    TokenType::IntImm,
                                    self.cur_line,
                                ))
                            }
                        },
                        None => panic!("Expected token not found"),
                    }
                } else {
                    return Err(AssemblerErr::ExpectedTokenType(
                        TokenType::IntImm,
                        self.cur_line,
                    ));
                };
                Ok(Instruction {
                    opcode: Opcode::Trp,
                    op1,
                    op2: 0,
                })
            }
            _ => return Err(AssemblerErr::InvalidToken(opcode_token, self.cur_line)),
        }
    }
    fn consume_num_bytes(&mut self) -> usize {
        // found directive token
        if self.consume_first_match(&DIR_TOKEN_TYPES) {
            let num_bytes = match self.previous() {
                Some(t) => {
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
            while !self.peek_first_match(&DIR_TOKEN_TYPES)
                && !self.peek_first_match(&INS_TOKEN_TYPES)
                && !self.peek_match(TokenType::Label)
                && !self.reached_eof()
            {
                self.advance();
            }
            num_bytes
        }
        // found opcode instruction token
        else if self.consume_first_match(&INS_TOKEN_TYPES) {
            while !self.peek_first_match(&DIR_TOKEN_TYPES)
                && !self.peek_first_match(&INS_TOKEN_TYPES)
                && !self.peek_match(TokenType::Label)
                && !self.reached_eof()
            {
                self.advance();
            }
            12
        } else {
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

#[derive(Debug)]
pub enum AssemblerErr {
    ExpectedTokenType(TokenType, usize),
    InvalidToken(Token, usize),
    ExpectedOpcode(usize),
    InvalidRg(String, usize),
    NonexistentLabel(String, usize),
}

impl Display for AssemblerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AssemblerErr::ExpectedTokenType(t, l) => {
                write!(f, "Expected token {:?} on line: {}", t, l)
            }
            AssemblerErr::ExpectedOpcode(l) => write!(f, "Expected any opcode on line: {}", l),
            AssemblerErr::InvalidRg(s, l) => write!(f, "Invalid register {} on line: {}", s, l),
            AssemblerErr::InvalidToken(t, l) => write!(f, "Invalid token {:?} on line: {}", t, l),
            AssemblerErr::NonexistentLabel(s, l) => {
                write!(f, "Label does not exist {:?} on line: {}", s, l)
            }
        }
    }
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
            Token {
                lexeme: String::from("JMP"),
                token_type: TokenType::Jmp,
                line: 10,
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
        assert!(asm.consume_first_match(&INS_TOKEN_TYPES));
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
