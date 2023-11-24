pub trait Visitor {
    fn visit_unary_expr();
}

pub trait VisitorAcceptor {
    fn accept(v: impl Visitor);
}
