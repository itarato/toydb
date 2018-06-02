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
        
      }
      query::Query::Select(q) => {}
    }
  }
}