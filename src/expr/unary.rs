use crate::expr::Expr;

enum Operator {
    Negate,
    Bang,
}

pub struct Unary {
    op: Operator,
    right: Expr,
}
