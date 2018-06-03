use std::fmt;
use std::collections::HashMap;

pub enum Query {
  Create(CreateQuery),
  Select(SelectQuery),
  Insert(InsertQuery),
}

impl fmt::Debug for Query {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      Query::Create(q) => write!(f, "Create query [{:#?}]", q),
      Query::Select(q) => write!(f, "Select query [{:#?}]", q),
      Query::Insert(q) => write!(f, "Insert query [{:#?}]", q),
    }
  }
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
  pub name: String,
  pub config: Type,
}

impl FieldDef {
  pub fn new(name: String, config: Type) -> FieldDef {
    FieldDef { name, config }
  }
}

#[derive(Debug)]
pub struct CreateQuery {
  pub table: String,
  pub fields: Vec<FieldDef>,
}

impl CreateQuery {
  pub fn new(table: String, fields: Vec<FieldDef>) -> CreateQuery {
    CreateQuery { table, fields }
  }
}

#[derive(Debug)]
pub struct SelectQuery {
  pub table: String,
  pub columns: Vec<String>,
}

impl SelectQuery {
  pub fn new(table: String, columns: Vec<String>) -> SelectQuery {
    SelectQuery {
      table,
      columns,
    }
  }
}

#[derive(Default, Debug)]
pub struct InsertQuery {
  pub table_name: String,
  pub raw_inserts: HashMap<String, String>,
}

impl InsertQuery {
  pub fn new(table_name: String, raw_inserts: HashMap<String, String>) -> InsertQuery {
    InsertQuery {
      table_name,
      raw_inserts,
    }
  }
}
