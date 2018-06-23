use query;
use std::collections::HashMap;
use std::{fmt, str};
use util;

type Schema = HashMap<String, ColumnInfo>;
type Row = Vec<u8>;

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
    schema: Schema,
    data: Vec<Row>,
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
        let mut row: Row = vec![0; schema_size];

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

fn restructure_field_def_list(field_defs: Vec<query::FieldDef>) -> Schema {
    let mut schema: Schema = HashMap::new();

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
    buf: &mut Row,
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

fn are_conditions_passing(
    row: &Row,
    schema: &Schema,
    conditions: &Vec<query::FieldCondition>,
) -> bool {
    for condition in conditions {
        let column_info = schema
            .get(&condition.field_name[..])
            .expect("Select condition has unknown field");

        let relation = query::Relation::from(&condition.relation);
        let value = util::Val::from(condition.value.clone(), &column_info.field_def.config);

        if relation.is_none() || value.is_none() {
            warn!("Condition cannot be parsed.");
            return false;
        }

        let orig = extract_row_value(row, &column_info);
        if orig.is_err() {
            error!("Value cannot be extracted");
            return false;
        }

        match relation.unwrap() {
            query::Relation::Eq => {
                if orig.unwrap() != value.unwrap() {
                    return false;
                }
            }
            query::Relation::Lt => {
                if !(orig.unwrap() < value.unwrap()) {
                    return false;
                }
            }
            query::Relation::Gt => {
                if !(orig.unwrap() > value.unwrap()) {
                    return false;
                }
            }
        }
    }

    true
}

fn extract_row_value(row: &Row, column_info: &ColumnInfo) -> Result<util::Val, ()> {
    match column_info.field_def.config {
        query::Type::Int => {
            let mut val = 0_u32;

            for i in 0..column_info.size {
                val |= (row[column_info.offs + i] as u32) << (i * 8);
            }

            Ok(util::Val::U32(val))
        }
        query::Type::Varchar(n) => Ok(util::Val::Varchar(
            str::from_utf8(&row[column_info.offs..(column_info.offs + (n as usize))])
                .unwrap()
                .to_owned(),
        )),
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
            if !are_conditions_passing(row, &db.schema, &query.conditions) {
                continue;
            }

            let mut row_vals: Vec<util::Val> = vec![];

            for column_name in &query.columns {
                let column_info = match db.schema.get(column_name) {
                    Some(ci) => ci,
                    None => {
                        // @TODO this check should happen before looping rows.
                        error!("Column not found");
                        continue;
                    }
                };
                row_vals.push(match column_info.field_def.config {
                    query::Type::Int => {
                        let mut val = 0_u32;

                        for i in 0..column_info.size {
                            val |= (row[column_info.offs + i] as u32) << (i * 8);
                        }

                        util::Val::U32(val)
                    }
                    query::Type::Varchar(n) => {
                        let slice = str::from_utf8(
                            &row[column_info.offs..(column_info.offs + (n as usize))],
                        ).unwrap();

                        let slice = match slice.find('\0') {
                            Some(n) => slice[0..n].to_owned(),
                            None => slice.to_owned(),
                        };

                        util::Val::Varchar(slice)
                    }
                });
            }

            res.push(row_vals);
        }

        Ok(res)
    }

    pub fn describe_db(&self) -> String {
        let mut out = String::new();

        for (name, db) in &self.dbs {
            out.push_str(format!("{}\n", name).as_str());
            for (column_name, column_info) in &db.schema {
                out.push_str(format!("\t{:12} : {:?}\n", column_name, column_info).as_str());
            }
        }

        out
    }
}
