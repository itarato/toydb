use query;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
  schema: Vec<query::FieldDef>,
  data: Vec<Vec<u8>>,
}

impl Database {
  pub fn new(schema: Vec<query::FieldDef>) -> Database {
    Database {
      schema,
      data: vec![],
    }
  }

  pub fn raw_insert(&mut self, raw_inserts: HashMap<String, String>) -> Result<(), ()> {
    let schema_size = schema_byte_size(&self.schema);
    let mut row: Vec<u8> = vec![0; schema_size];

    let mut offs = 0_usize;
    for def in &self.schema {
      let size = size_of_type(&def.config);

      let raw_val = match raw_inserts.get(&def.name[..]) {
        Some(s) => s,
        None => "",
      };
      println!("Write val: {:#?}", raw_val);

      let _ = write_bytes(&mut row, size, offs, raw_val, &def.config).or_else(|e| {
        warn!("Data write error");
        Err(e)
      });
      offs += size;
    }

    self.data.push(row);

    Ok(())
  }
}

fn write_bytes(buf: &mut Vec<u8>, len: usize, offs: usize, raw: &str, data_type: &query::Type) -> Result<(), ()> {
  match data_type {
    query::Type::Int => {
      let uint_val: u128 = match u128::from_str_radix(raw, 10) {
        Ok(n) => n,
        Err(e) => {
          error!("Int cannot be parsed: {:?}", e);
          return Err(());
        }
      };
      for idx in 0..len {
        buf[offs + idx] = ((uint_val >> (idx * 8)) & 0xFF) as u8;
      }
    },
    query::Type::Varchar(_) => {
      let mut idx = 0_usize;
      for ch in raw.chars() {
        if idx >= len {
          warn!("String truncated");
          return Err(());
        }

        buf[offs + idx] = ch as u8;
        idx += 1;
      }
    }
  }
  Ok(())
}

fn schema_byte_size(schema: &Vec<query::FieldDef>) -> usize {
  let mut size = 0_usize;

  for field_def in schema {
    size += size_of_type(&field_def.config);
  }

  size
}

fn size_of_type(data_type: &query::Type) -> usize {
  match data_type {
    query::Type::Int => 4_usize,
    query::Type::Varchar(n) => *n as usize,
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
    match self.dbs.get_mut(&query.table_name[..]) {
      Some(db) => {
        let _ = db.raw_insert(query.raw_inserts);
      }
      None => {
        error!("Missing table: {}", query.table_name);
        return Err(());
      }
    }

    Ok(())
  }
}
