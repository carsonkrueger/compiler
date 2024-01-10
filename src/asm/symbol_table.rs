use std::collections::HashMap;

use super::{symbol::Symbol, token::Token};

pub struct SymbolTable {
    table: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    fn contains(&self, string: &String) -> bool {
        self.table.contains_key(string)
    }
    pub fn insert(&mut self, symbol: &Symbol) -> Result<(), SymbolTableErr> {
        match self
            .table
            .insert(symbol.token.lexeme.clone(), symbol.clone())
        {
            None => Ok(()),
            Some(_) => Err(SymbolTableErr::AlreadyExists(symbol.clone())),
        }
    }
    pub fn get(&self, string: &String) -> Option<&Symbol> {
        self.table.get(string)
    }
    pub fn get_expect(&self, string: &String) -> &Symbol {
        match self.table.get(string) {
            Some(s) => s,
            None => panic!("Symbol does not exist in symbol table: {}", string),
        }
    }
}

pub enum SymbolTableErr {
    AlreadyExists(Symbol),
}
