use query;
use std::collections::HashMap;
use std::str;
use util;

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

      match raw_inserts.get(&def.name[..]) {
        Some(raw) => {
          let _ = write_bytes(&mut row, size, offs, raw, &def.config).or_else(|e| {
            warn!("Data write error");
            Err(e)
          });
        }
        None => {}
      };

      offs += size;
    }

    self.data.push(row);

    Ok(())
  }
}

fn write_bytes(
  buf: &mut Vec<u8>,
  len: usize,
  offs: usize,
  raw: &str,
  data_type: &query::Type,
) -> Result<(), ()> {
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
    }
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

  pub fn select(&self, query: query::SelectQuery) -> Result<Vec<Vec<util::Val>>, ()> {
    let db = match self.dbs.get(&query.table[..]) {
      Some(db) => db,
      None => {
        warn!("No table: {}", &query.table);
        return Err(());
      }
    };

    let mut res: Vec<Vec<util::Val>> = vec![];

    for row in &db.data {
      let mut offs = 0_usize;
      let mut row_vals: Vec<util::Val> = vec![];

      for query::FieldDef {
        name: _,
        config: data_type,
      } in &db.schema
      {
        row_vals.push(match data_type {
          query::Type::Int => {
            let mut val = 0_u32;
            let size = size_of_type(data_type);

            for i in 0..size {
              val |= (row[offs + i] as u32) << (i * 8);
            }

            util::Val::U32(val)
          }
          query::Type::Varchar(n) => util::Val::Varchar(
            str::from_utf8(&row[offs..(offs + (*n as usize))])
              .unwrap()
              .to_owned(),
          ),
        });

        offs += size_of_type(data_type);
      }

      res.push(row_vals);
    }

    Ok(res)
  }
}
