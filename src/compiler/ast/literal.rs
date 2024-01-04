use crate::compiler::eval::EvalErr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f32),
}

impl std::ops::Add for Literal {
    type Output = Result<Literal, EvalErr>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(i1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Int(i1 + i2)),
                Literal::Float(f2) => Ok(Literal::Float(i1 as f32 + f2)),
                _ => Err(EvalErr::InvalidAdd),
            },
            Literal::Float(f1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Float(f1 + i2 as f32)),
                Literal::Float(f2) => Ok(Literal::Float(f1 + f2)),
                _ => Err(EvalErr::InvalidAdd),
            },
            _ => Err(EvalErr::InvalidAdd),
        }
    }
}

impl std::ops::Sub for Literal {
    type Output = Result<Literal, EvalErr>;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(i1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Int(i1 - i2)),
                Literal::Float(f2) => Ok(Literal::Float(i1 as f32 - f2)),
                _ => Err(EvalErr::InvalidSub),
            },
            Literal::Float(f1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Float(f1 - i2 as f32)),
                Literal::Float(f2) => Ok(Literal::Float(f1 - f2)),
                _ => Err(EvalErr::InvalidSub),
            },
            _ => Err(EvalErr::InvalidSub),
        }
    }
}

impl std::ops::Mul for Literal {
    type Output = Result<Literal, EvalErr>;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(i1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Int(i1 * i2)),
                Literal::Float(f2) => Ok(Literal::Float(i1 as f32 * f2)),
                _ => Err(EvalErr::InvalidMul),
            },
            Literal::Float(f1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Float(f1 * i2 as f32)),
                Literal::Float(f2) => Ok(Literal::Float(f1 * f2)),
                _ => Err(EvalErr::InvalidMul),
            },
            _ => Err(EvalErr::InvalidMul),
        }
    }
}

impl std::ops::Div for Literal {
    type Output = Result<Literal, EvalErr>;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(i1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Int(i1 / i2)),
                Literal::Float(f2) => Ok(Literal::Float(i1 as f32 / f2)),
                _ => Err(EvalErr::InvalidDiv),
            },
            Literal::Float(f1) => match rhs {
                Literal::Int(i2) => Ok(Literal::Float(f1 / i2 as f32)),
                Literal::Float(f2) => Ok(Literal::Float(f1 / f2)),
                _ => Err(EvalErr::InvalidDiv),
            },
            _ => Err(EvalErr::InvalidDiv),
        }
    }
}

impl std::cmp::PartialOrd for Literal {
    fn ge(&self, other: &Self) -> bool {
        match self {
            Literal::Float(f1) => {
                *f1 >= match other {
                    Literal::Float(f2) => *f2,
                    Literal::Int(i2) => *i2 as f32,
                    _ => return false,
                }
            }
            Literal::Int(i1) => {
                *i1 >= match other {
                    Literal::Float(f2) => *f2 as i32,
                    Literal::Int(i2) => *i2,
                    _ => return false,
                }
            }
            _ => false,
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match self {
            Literal::Float(f1) => {
                *f1 > match other {
                    Literal::Float(f2) => *f2,
                    Literal::Int(i2) => *i2 as f32,
                    _ => return false,
                }
            }
            Literal::Int(i1) => {
                *i1 > match other {
                    Literal::Float(f2) => *f2 as i32,
                    Literal::Int(i2) => *i2,
                    _ => return false,
                }
            }
            _ => false,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self {
            Literal::Float(f1) => {
                *f1 <= match other {
                    Literal::Float(f2) => *f2,
                    Literal::Int(i2) => *i2 as f32,
                    _ => return false,
                }
            }
            Literal::Int(i1) => {
                *i1 <= match other {
                    Literal::Float(f2) => *f2 as i32,
                    Literal::Int(i2) => *i2,
                    _ => return false,
                }
            }
            _ => false,
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match self {
            Literal::Float(f1) => {
                *f1 < match other {
                    Literal::Float(f2) => *f2,
                    Literal::Int(i2) => *i2 as f32,
                    _ => return false,
                }
            }
            Literal::Int(i1) => {
                *i1 < match other {
                    Literal::Float(f2) => *f2 as i32,
                    Literal::Int(i2) => *i2,
                    _ => return false,
                }
            }
            _ => false,
        }
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other {
            Some(std::cmp::Ordering::Less)
        } else if self > other {
            Some(std::cmp::Ordering::Less)
        } else if self == other {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl std::cmp::Ord for Literal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self < other {
            std::cmp::Ordering::Less
        } else if self > other {
            std::cmp::Ordering::Greater
        } else if self == other {
            std::cmp::Ordering::Equal
        } else {
            panic!("Invalid comparison between {:?} and {:?}", self, other);
        }
    }
    // fn cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

    // }
}

impl std::cmp::Eq for Literal {
    fn assert_receiver_is_total_eq(&self) {}
}
