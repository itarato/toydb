use query;
use std::collections::HashMap;
use std::str;
use util;

#[derive(Debug)]
struct ColumnInfo {
    name: String,
    offs: usize,
    size: usize,
    field_def: query::FieldDef,
}

impl ColumnInfo {
    fn new(name: String, offs: usize, size: usize, field_def: query::FieldDef) -> ColumnInfo {
        ColumnInfo {
            name,
            offs,
            size,
            field_def,
        }
    }
}

#[derive(Debug)]
pub struct Database {
    schema: HashMap<String, ColumnInfo>,
    data: Vec<Vec<u8>>,
}

impl Database {
    pub fn new(schema: Vec<query::FieldDef>) -> Database {
        Database {
            schema: restructure_field_def_list(schema),
            data: vec![],
        }
    }

    pub fn raw_insert(&mut self, raw_inserts: HashMap<String, String>) -> Result<(), ()> {
        let schema_size = self.schema_byte_size();
        let mut row: Vec<u8> = vec![0; schema_size];

        for (column_name, column_info) in &self.schema {
            match raw_inserts.get(&column_name[..]) {
                Some(raw) => {
                    let _ = write_bytes(
                        &mut row,
                        column_info.size,
                        column_info.offs,
                        raw,
                        &column_info.field_def.config,
                    ).or_else(|e| {
                        warn!("Data write error");
                        Err(e)
                    });
                }
                None => {}
            };
        }

        self.data.push(row);

        Ok(())
    }

    // @todo There is a better way to do this.
    fn schema_byte_size(&self) -> usize {
        let mut size = 0_usize;

        for (_, column_info) in &self.schema {
            size += column_info.size;
        }

        size
    }
}

fn restructure_field_def_list(field_defs: Vec<query::FieldDef>) -> HashMap<String, ColumnInfo> {
    let mut schema: HashMap<String, ColumnInfo> = HashMap::new();

    let mut offs = 0_usize;
    for field_def in field_defs {
        let size = size_of_type(&field_def.config);
        schema.insert(
            field_def.name.clone(),
            ColumnInfo::new(field_def.name.clone(), offs, size, field_def),
        );
        offs += size;
    }

    schema
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
            let mut row_vals: Vec<util::Val> = vec![];

            for (_, column_info) in &db.schema {
                row_vals.push(match column_info.field_def.config {
                    query::Type::Int => {
                        let mut val = 0_u32;

                        for i in 0..column_info.size {
                            val |= (row[column_info.offs + i] as u32) << (i * 8);
                        }

                        util::Val::U32(val)
                    }
                    query::Type::Varchar(n) => util::Val::Varchar(
                        str::from_utf8(&row[column_info.offs..(column_info.offs + (n as usize))])
                            .unwrap()
                            .to_owned(),
                    ),
                });
            }

            res.push(row_vals);
        }

        Ok(res)
    }
}
