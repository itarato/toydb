use engine;
use query;
use serde_json;

#[derive(Default, Debug)]
pub struct EngineOperator {
    engine: engine::Engine,
}

impl EngineOperator {
    pub fn execute(&mut self, query: query::Query) {
        info!("Execute query");

        match query {
            query::Query::Create(q) => {
                let _ = self.engine.create_table(q);
            }
            query::Query::Select(q) => {
                info!("Exec query {:#?}", q);
                let res = self.engine.select(q);

                if let Ok(res) = res {
                    let res = serde_json::to_string(&res).unwrap();
                    println!("{:#?}", res);
                }
            }
            query::Query::Insert(q) => {
                let _ = self.engine.insert(q);
            }
            query::Query::Describe(_) => {
                self.engine.describe_db();
            }
        }
    }
}
