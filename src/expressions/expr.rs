use super::{binary::BinaryExpr, unary::UnaryExpr};

pub enum Expr {
    // Literal(Literal),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Grouping(Box<Expr>),
}
