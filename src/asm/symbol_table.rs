use std::collections::HashMap;

use super::{symbol::Symbol, token::Token};

pub struct SymbolTable<'a> {
    table: HashMap<String, Symbol<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    fn contains(&self, string: &String) -> bool {
        self.table.contains_key(string)
    }
    pub fn insert(&mut self, symbol: &'a Symbol) -> Result<(), SymbolTableErr> {
        match self
            .table
            .insert(symbol.token.lexeme.clone(), symbol.clone())
        {
            None => Ok(()),
            Some(_) => Err(SymbolTableErr::AlreadyExists(&symbol)),
        }
    }
    pub fn get(&self, string: &String) -> Option<&Symbol> {
        self.table.get(string)
    }
}

pub enum SymbolTableErr<'a> {
    AlreadyExists(&'a Symbol<'a>),
}
