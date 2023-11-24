use crate::expressions::expr::Expr;

enum Operator {
    Negate,
    Bang,
}

pub struct UnaryExpr {
    op: Operator,
    right: Expr,
}
