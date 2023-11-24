use crate::expressions::expr::Expr;

enum Operator {
    Mult,
    Div,
    Plus,
    Minus,
}

pub struct BinaryExpr {
    left: Expr,
    op: Operator,
    right: Expr,
}
