use engine;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

#[derive(Debug, Default)]
pub struct TableSyncer {}

impl TableSyncer {
    pub fn read_tables(&self) -> Result<HashMap<String, engine::Table>, ()> {
        let mut tables: HashMap<String, engine::Table> = HashMap::new();
        for entry in fs::read_dir("./db/").unwrap() {
            let entry = entry.unwrap();
            let file_name: String = entry.file_name().into_string().unwrap();
            let (base, ext) = dbg!(file_name.split_at(file_name.find(".").unwrap()));
            if ext != ".tdb.table" {
                continue;
            }

            let path = entry.path();
            let path = path.to_str().unwrap();

            let mut raw: String = String::new();
            let mut f: File = File::open(path.clone()).unwrap();
            f.read_to_string(&mut raw).map_err(|_| ())?;

            let table_schema: engine::Schema = serde_json::from_str(raw.as_ref()).unwrap();
            println!("Schema found for {:?}: {:#?}", path, table_schema);

            let table = engine::Table::new_with_schema(table_schema);
            tables.insert(base.into(), table);
        }

        Ok(tables)
    }

    pub fn create(&self, name: String, table: &engine::Table) -> Result<(), ()> {
        // write table def
        let mut f_table_def = File::create(format!("./db/{}.tdb.table", name)).map_err(|_| ())?;
        let schema_json = serde_json::to_string(&table.schema).unwrap();
        f_table_def
            .write_all(schema_json.as_bytes())
            .map_err(|_| ())?;

        // write data
        let mut f_data = File::create(format!("./db/{}.tdb.data", name)).map_err(|_| ())?;
        for row in &table.data {
            f_data.write_all(row.as_ref()).map_err(|_| ())?;
        }

        Ok(())
    }
    // pub fn save(&self, table: &engine::Table) -> bool {
    //     false
    // }
}
