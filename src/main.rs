#[macro_use]
extern crate log;
extern crate env_logger;

mod query;
mod query_parser;
mod repl;

fn main() {
  env_logger::init();

  info!("DB is starting");

  let repl = repl::Repl::new();
  repl.start();
}
