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
}

impl<'a> Assembler<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            cur_idx: 0,
            symbol_table: SymbolTable::new(),
        }
    }
    fn pass_one(&mut self) {
        while !self.reached_eof() {
            if self.consume_match(TokenType::Label) {
                let t = match self.previous() {
                    Some(t) => t,
                    None => continue,
                };
                let symbol = Symbol::new(t.clone(), self.num_bytes_dir());
                self.symbol_table.insert(&symbol);
            }
        }
    }
    fn pass_two(&mut self) {
        loop {
            if let Some(directive) = self.next_directive() {
                directive.write(writer)
            } else {
                break;
            }
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
    fn next_directive(&mut self) -> Option<Directive> {
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
    fn num_bytes_dir(&mut self) -> usize {
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
}
