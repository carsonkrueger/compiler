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
}

pub enum SymbolTableErr {
    AlreadyExists(Symbol),
}
