use query;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Eq, Hash)]
#[serde(tag = "type", content = "val")]
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

impl Val {
    pub fn from(raw: String, data_type: &query::Type) -> Option<Val> {
        match data_type {
            &query::Type::Int => Val::wrap_raw_int(raw),
            &query::Type::Varchar(n) => Val::wrap_raw_varchar(raw, n),
        }
    }

    fn wrap_raw_int(raw: String) -> Option<Val> {
        let num_res = u32::from_str_radix(&raw[..], 10);

        if num_res.is_err() {
            return None;
        }

        Some(Val::U32(num_res.unwrap()))
    }

    fn wrap_raw_varchar(raw: String, len: u8) -> Option<Val> {
        Some(Val::Varchar(raw[0..(len as usize)].to_owned()))
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Val) -> bool {
        match (self, other) {
            (Val::U32(l), Val::U32(r)) => l == r,
            (Val::Varchar(l), Val::Varchar(r)) => l == r,
            _ => false,
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Val) -> Option<Ordering> {
        match (self, other) {
            (Val::U32(l), Val::U32(r)) => Some(l.cmp(r)),
            (Val::Varchar(l), Val::Varchar(r)) => Some(l.cmp(r)),
            _ => None,
        }
    }
}

pub trait VerboseMode {
    fn is_verbose(&self) -> bool;
}