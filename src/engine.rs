use query;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
  schema: Vec<query::FieldDef>,
  data: Vec<Box<[u8]>>,
}

impl Database {
  pub fn new(schema: Vec<query::FieldDef>) -> Database {
    Database {
      schema,
      data: vec![],
    }
  }
}

#[derive(Debug, Default)]
pub struct Engine {
  pub dbs: HashMap<String, Database>,
}

impl Engine {
  pub fn create_table(&mut self, q: query::CreateQuery) -> Result<(), ()> {
    self.dbs.insert(q.table, Database::new(q.fields));
    Ok(())
  }

  pub fn insert(&mut self, query: query::InsertQuery) -> Result<(), ()> {
    Ok(())
  }
}
