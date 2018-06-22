use engine_operator;
use futures::{future, Future, Stream};
use hyper::rt;
use hyper::service::service_fn;
use hyper::{self, Body, Method, Request, Response, Server, StatusCode};
use query_parser;
use std::fs::File;
use std::io::{self, prelude::*};
use std::str;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct DBServer {
  engine_operator: Arc<Mutex<engine_operator::EngineOperator>>,
  query_parser: Arc<query_parser::QueryParser>,
}

impl DBServer {
  pub fn new() -> DBServer {
    Default::default()
  }

  pub fn run(&self) {
    let addr = ([127, 0, 0, 1], 8421).into();
    let eo = self.engine_operator.clone();
    let qp = self.query_parser.clone();
    rt::run(rt::lazy(move || {
      let eo = eo.clone();
      let qp = qp.clone();
      let new_service = move || {
        let eo = eo.clone();
        let qp = qp.clone();
        service_fn(move |req| prepare_response(req, eo.clone(), qp.clone()))
      };
      let server = Server::bind(&addr).serve(new_service).map_err(|_| ());
      server
    }));
  }

  pub fn read_file(&self, file_name: &String) -> Result<(), io::Error> {
    let mut f = File::open(file_name)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    buffer
      .split('\n')
      .map(|l| l.trim())
      .filter(|l| l.len() > 0)
      .for_each(|l| {
        execute_raw_command(
          &l.to_owned(),
          self.engine_operator.clone(),
          self.query_parser.clone(),
        );
        ()
      });

    Ok(())
  }
}

fn execute_raw_command(
  raw: &String,
  engine_operator: Arc<Mutex<engine_operator::EngineOperator>>,
  query_parser: Arc<query_parser::QueryParser>,
) {
  match query_parser.parse(&raw) {
    Ok(query) => {
      let mut engine_operator = engine_operator.lock().unwrap();
      engine_operator.execute(query);
    }
    Err(_) => {}
  };
}

fn prepare_response(
  req: Request<Body>,
  engine_operator: Arc<Mutex<engine_operator::EngineOperator>>,
  query_parser: Arc<query_parser::QueryParser>,
) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
  match req.method() {
    &Method::POST => {
      let fut = req.into_body().concat2().and_then(move |chunk| {
        let value = str::from_utf8(chunk.as_ref()).unwrap().to_owned();

        execute_raw_command(&value, engine_operator, query_parser);

        future::ok(Response::new(Body::empty()))
      });
      Box::new(fut)
    }
    _ => {
      let body = Body::empty();
      Box::new(future::ok(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body(body)
          .unwrap(),
      ))
    }
  }
}
