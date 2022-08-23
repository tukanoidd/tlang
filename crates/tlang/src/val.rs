use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Val {
    Number(i32),
    Unit,
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Number(n) => {
                write!(f, "{}", n)
            }
            Val::Unit => {
                write!(f, "Unit")
            }
        }
    }
}
