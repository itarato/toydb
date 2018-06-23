#[derive(Debug, Default)]
pub struct DBClient;

use futures::{Future, Stream};
use hyper::header::HeaderValue;
use hyper::rt::{lazy, run};
use hyper::{self, Body, Client, Method, Request};
use std::str;

impl DBClient {
  pub fn send(&self, raw: &String) {
    let uri: hyper::Uri = "http://localhost:8421/".parse().unwrap();
    let mut req = Request::new(Body::from(raw.clone()));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req
      .headers_mut()
      .insert("Content-Type", HeaderValue::from_str("text/plain").unwrap());

    run(lazy(move || {
      Client::new()
        .request(req)
        .and_then(|res| res.into_body().concat2())
        .map(|chunk| {
          println!("{}", str::from_utf8(chunk.as_ref()).unwrap());
          ()
        })
        .map_err(|_| ())
    }));
  }
}
