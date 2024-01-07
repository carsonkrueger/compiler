use super::token::Token;

#[derive(Debug, Clone, Copy)]
pub struct Symbol<'a> {
    pub token: &'a Token,
    pub offset: usize,
}

impl<'a> Symbol<'a> {
    fn new(token: &'a Token, offset: usize) -> Self {
        Self { token, offset }
    }
}
