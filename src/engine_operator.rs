use engine;
use query;
use serde_json;
use table_sync;

#[derive(Default, Debug)]
pub struct EngineOperator {
    engine: engine::Engine,
    table_syncer: table_sync::TableSyncer,
}

impl EngineOperator {
    pub fn init(&mut self) -> Result<(), ()> {
        self.engine.tables = dbg!(self.table_syncer.read_tables()?);
        Ok(())
    }

    pub fn execute(&mut self, query: query::Query) -> Result<String, ()> {
        info!("Execute query");

        match query {
            query::Query::Create(q) => {
                let _ = self.engine.create_table(q);
                self.sync_tables()?;
                Ok("".to_owned())
            }
            query::Query::Select(q) => {
                info!("Exec query {:#?}", q);
                let res = self.engine.select(q);

                if res.is_ok() {
                    let res = serde_json::to_string(&res.unwrap()).unwrap();
                    Ok(res)
                } else {
                    Err(())
                }
            }
            query::Query::Insert(q) => {
                let _ = self.engine.insert(q);
                self.sync_tables()?;
                Ok("".to_owned())
            }
            query::Query::Describe(_) => Ok(self.engine.describe_db()),
        }
    }

    fn sync_tables(&self) -> Result<(), ()> {
        for (table_name, table) in &self.engine.tables {
            self.table_syncer.create(table_name.clone(), &table)?;
        }
        Ok(())
    }
}
