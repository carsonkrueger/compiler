use crate::expr::Expr;

enum Operator {
    Mult,
    Div,
    Plus,
    Minus,
    EqEq,
}

pub struct Binary {
    left: Expr,
    op: Operator,
    right: Expr,
}
