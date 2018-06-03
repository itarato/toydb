use engine;
use query;

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
        println!("{:#?}", res);
      }
      query::Query::Insert(q) => {
        let _ = self.engine.insert(q);
      }
    }
  }
}
