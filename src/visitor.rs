use crate::ast::Expr;

pub trait ExprVisitor {
    fn visit_literal_expr<T>(&self, expr: &Expr) -> T;
}

pub trait Evaluate {
    fn accept(&self, v: &impl ExprVisitor);
}
