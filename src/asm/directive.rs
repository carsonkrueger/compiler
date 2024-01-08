use crate::asm::token::{Token, TokenType};
use std::io::{Seek, Write};

pub enum Directive {
    Byt(u8),
    Int(i32),
    Str(String),
}

impl Directive {
    fn write<W: Write + Seek>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.seek(std::io::SeekFrom::End(0))?;
        let bytes = match &self {
            Directive::Int(i) => i.to_le_bytes().as_slice().to_owned(),
            Directive::Byt(b) => b.to_le_bytes().as_slice().to_owned(),
            Directive::Str(s) => s.as_bytes().to_owned(),
        };
        writer.write_all(&bytes)?;
        Ok(())
    }
}

impl TryFrom<Token> for Directive {
    type Error = ();
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.token_type {
            TokenType::BytDir => {}
            TokenType::IntDir => TokenType::StrDir,
        }
    }
}
