pub trait Evaluate<T> {
    fn eval(self) -> T;
}

#[derive(Debug)]
pub enum EvalErr {
    InvalidBang,
    InvalidNegate,
    InvalidMul,
    InvalidDiv,
    InvalidAdd,
    InvalidSub,
}
