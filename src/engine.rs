use query;
use std::collections::HashMap;

#[derive(Debug)]
struct Database {
  schema: Vec<query::FieldDef>,
  data: Vec<Box<[u8]>>,
}

#[derive(Debug, Default)]
pub struct Engine {
  dbs: HashMap<String, Database>,
}

impl Engine {}
