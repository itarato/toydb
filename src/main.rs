#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod dbserver;
mod engine;
mod engine_operator;
mod query;
mod query_parser;
mod util;
mod index;

use std::env;

fn main() {
    env_logger::init();

    info!("DB is starting");

    let dbs = dbserver::DBServer::new();

    if let Some(file_name) = env::args().nth(1) {
        info!("Got file-argument: {}", file_name);
        let _ = dbs.read_file(&file_name);
    }

    dbs.run();
}
