#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate hyper;

use std::env;

mod dbclient;
mod query;
mod query_parser;
mod repl;

fn main() {
  env_logger::init();

  info!("DB is starting");

  let repl = repl::Repl::new();

  if let Some(file_name) = env::args().nth(1) {
    info!("Got file-argument: {}", file_name);
    let _ = repl.read_file(&file_name);
  }

  repl.start();
}
