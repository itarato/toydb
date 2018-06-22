#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate hyper;

mod dbclient;
mod query;
mod query_parser;
mod repl;

fn main() {
  env_logger::init();

  info!("DB is starting");

  let repl = repl::Repl::new();

  repl.start();
}
