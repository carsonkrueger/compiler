use crate::token::token_type::TokenType;

pub enum Literal {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f32),
}

pub enum Expr {
    LiteralExpr(Literal),
    Unary {
        op: UnaryOp,
        rhs: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

pub enum BinaryOp {
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

pub enum UnaryOp {
    Bang,
    Negate,
}

impl TryFrom<&TokenType> for UnaryOp {
    type Error = ();
    fn try_from(value: &TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Bang => Ok(UnaryOp::Bang),
            TokenType::Minus => Ok(UnaryOp::Negate),
            _ => Err(()),
        }
    }
}
