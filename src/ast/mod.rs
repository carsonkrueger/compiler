use crate::token::token_type::TokenType;

pub enum Expr {
    Bool(bool),
    Int(i32),
    Float(f32),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

enum BinaryOp {
    Plus,
    Minus,
    Mult,
    Div,
    Eq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    EqEq,
    BangEq,
}

impl TryFrom<&TokenType> for BinaryOp {
    type Error = ();
    fn try_from(value: &TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Plus => Ok(BinaryOp::Plus),
            TokenType::Minus => Ok(BinaryOp::Minus),
            TokenType::Star => Ok(BinaryOp::Mult),
            TokenType::Slash => Ok(BinaryOp::Div),
            TokenType::Eq => Ok(BinaryOp::Eq),
            TokenType::Gt => Ok(BinaryOp::Gt),
            TokenType::Lt => Ok(BinaryOp::Lt),
            TokenType::GtEq => Ok(BinaryOp::GtEq),
            TokenType::LtEq => Ok(BinaryOp::LtEq),
            TokenType::EqEq => Ok(BinaryOp::EqEq),
            TokenType::BangEq => Ok(BinaryOp::BangEq),
            _ => Err(()),
        }
    }
}

enum UnaryOp {
    Bang,
    Negate,
}
