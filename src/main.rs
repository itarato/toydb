#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

mod dbserver;
mod engine;
mod engine_operator;
mod index;
mod query;
mod query_parser;
mod util;

use clap::{App, Arg};
use std::cell::Cell;
use std::sync::Mutex;

lazy_static! {
    static ref IS_VERBOSE: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
}

fn is_verbose() -> bool {
    IS_VERBOSE.lock().unwrap().get()
}

fn main() {
    env_logger::init();

    let matches = App::new("ToyDB")
        .version("0.1")
        .author("Peter Arato <it.arato@gmail.com>")
        .arg(
            Arg::with_name("dump")
                .short("d")
                .long("dump")
                .value_name("DUMP")
                .help("Database dump to start with")
                .takes_value(true),
        )
        .arg(Arg::with_name("v").short("v").help("Verbose mode"))
        .get_matches();

    IS_VERBOSE.lock().unwrap().set(matches.is_present("v"));

    info!("DB is starting");

    let dbs: dbserver::DBServer = Default::default();

    if let Some(file_name) = matches.value_of("dump") {
        info!("Got file-argument: {}", file_name);
        let _ = dbs.read_file(&file_name.to_owned());
    }

    dbs.run();
}
