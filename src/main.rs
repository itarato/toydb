#[macro_use]
extern crate log;
extern crate env_logger;

mod engine;
mod engine_operator;
mod query;
mod query_parser;
mod repl;
mod util;

fn main() {
    env_logger::init();

    info!("DB is starting");

    let repl = repl::Repl::new();
    repl.start();
}
