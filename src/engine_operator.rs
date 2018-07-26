use engine;
use query;
use serde_json;

#[derive(Default, Debug)]
pub struct EngineOperator {
    engine: engine::Engine,
}

impl EngineOperator {
    pub fn execute(&mut self, query: query::Query) -> Result<String, ()> {
        info!("Execute query");

        match query {
            query::Query::Create(q) => {
                let _ = self.engine.create_table(q);
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
                Ok("".to_owned())
            }
            query::Query::Describe(_) => Ok(self.engine.describe_db()),
        }
    }
}
