use std::fmt;

pub enum Val {
    U32(u32),
    Varchar(String),
}

impl fmt::Debug for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Val::U32(v) => write!(f, "{}", v),
            Val::Varchar(v) => write!(f, "{}", v),
        }
    }
}
