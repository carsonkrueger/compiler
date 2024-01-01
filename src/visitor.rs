use crate::ast::{Expr, Literal};

pub trait ExprVisitor {
    fn visit_literal_expr(expr: &Expr) -> &Literal;
}

pub trait Evaluate {
    fn accept(&self, v: &impl ExprVisitor);
}
