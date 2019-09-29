#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod dbclient;
mod query;
mod query_parser;
mod repl;

fn main() {
  env_logger::init();
  info!("DB is starting");
  repl::Repl::new().start();
}
