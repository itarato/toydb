#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate hyper;

mod dbserver;
mod engine;
mod engine_operator;
mod query;
mod query_parser;
mod util;

fn main() {
    env_logger::init();

    info!("DB is starting");

    dbserver::DBServer::new().run();
}
