use engine;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Default)]
pub struct TableSyncer {}

impl TableSyncer {
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
