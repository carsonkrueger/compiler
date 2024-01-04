use super::literal::Literal;

use crate::compiler::{
    eval::{EvalErr, Evaluate},
    token::TokenType,
};

impl Evaluate<Result<Literal, EvalErr>> for Expr {
    fn eval(self) -> Result<Literal, EvalErr> {
        match self {
            Expr::LiteralExpr(le) => Ok(le),
            Expr::Unary { op, rhs } => match op {
                UnaryOp::Bang => !(*rhs),
                UnaryOp::Negate => -(*rhs),
            },
            _ => Err(EvalErr::InvalidBang),
        }
    }
}

// impl std::cmp::PartialOrd for Expr {
//     fn ge(&self, other: &Self) -> bool {
//         match self.eval() {
//             Ok(l1) => {
//                 l1 >= match other.eval() {
//                     Ok(l2) => l2,
//                     Err(e) => false,
//                 }
//             }
//             Err(e) => false,
//         }
//     }
// }

impl std::ops::Add for Expr {
    type Output = Result<Literal, EvalErr>;
    fn add(self, rhs: Self) -> Self::Output {
        match self.eval() {
            Ok(l) => {
                l + match rhs.eval() {
                    Ok(l2) => l2,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl std::ops::Sub for Expr {
    type Output = Result<Literal, EvalErr>;
    fn sub(self, rhs: Self) -> Self::Output {
        match self.eval() {
            Ok(l) => {
                l - match rhs.eval() {
                    Ok(l2) => l2,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl std::ops::Mul for Expr {
    type Output = Result<Literal, EvalErr>;
    fn mul(self, rhs: Self) -> Self::Output {
        match self.eval() {
            Ok(l) => {
                l * match rhs.eval() {
                    Ok(l2) => l2,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl std::ops::Div for Expr {
    type Output = Result<Literal, EvalErr>;
    fn div(self, rhs: Self) -> Self::Output {
        match self.eval() {
            Ok(l) => {
                l / match rhs.eval() {
                    Ok(l2) => l2,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl std::ops::Not for Expr {
    type Output = Result<Literal, EvalErr>;
    fn not(self) -> Self::Output {
        match self.eval() {
            Ok(l) => match l {
                Literal::Bool(b) => Ok(Literal::Bool(!b)),
                Literal::Nil => Ok(Literal::Bool(true)),
                _ => Err(EvalErr::InvalidBang),
            },
            Err(e) => Err(e),
        }
    }
}

impl std::ops::Neg for Expr {
    type Output = Result<Literal, EvalErr>;
    fn neg(self) -> Self::Output {
        match self.eval() {
            Ok(l) => match l {
                Literal::Float(f) => Ok(Literal::Float(-f)),
                Literal::Int(i) => Ok(Literal::Int(-i)),
                _ => Err(EvalErr::InvalidNegate),
            },
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
