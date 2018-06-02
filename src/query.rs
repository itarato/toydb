use std::fmt;

pub enum Query {
  Create(CreateQuery),
  Select(SelectQuery),
}

pub enum Type {
  Int,
  Varchar(u8),
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self {
        Type::Int => write!(f, "Int"),
        Type::Varchar(n) => write!(f, "Varchar of size {}", n),
      }
    }
}

#[derive(Debug)]
pub struct FieldDef {
  name: String,
  config: Type,
}

impl FieldDef {
  pub fn new(name: String, config: Type) -> FieldDef {
    FieldDef {
      name,
      config,
    }
  }
}

#[derive(Debug)]
pub struct CreateQuery {
  table: String,
  fields: Vec<FieldDef>,
}

impl CreateQuery {
  pub fn new(table: String, fields: Vec<FieldDef>) -> CreateQuery {
    CreateQuery {
      table,
      fields,
    }
  }
}

#[derive(Debug)]
pub struct SelectQuery {

}
