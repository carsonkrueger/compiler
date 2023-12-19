pub mod binary;
pub mod literal;
pub mod unary;

use {binary::Binary, literal::Literal, unary::Unary};

pub enum Expr {
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Expr>),
}
