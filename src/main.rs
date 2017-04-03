// extern crate chrono;
// use chrono::datetime::DateTime;
// use chrono::offset::local::Local;

// mod data;
// use data::Data;

//mod storage;
//use storage::Storage;

extern crate iron;

use iron::prelude::*;
use iron::status;

extern crate router;
use router::Router;

struct StoreRequest<T> {
    key: String,
    value: T,
}

fn main() {
    let mut router = Router::new();
    router.post("/store/:key/:value", handler, "data");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let key = req.extensions
        .get::<Router>()
        .unwrap()
        .find("key")
        .unwrap_or("/");
    let value = req.extensions
        .get::<Router>()
        .unwrap()
        .find("value")
        .unwrap_or("/");
    Ok(Response::with((status::Ok, key)))
}
