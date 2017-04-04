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

extern crate rustc_serialize;
use rustc_serialize::json;

use std::io::Read;

#[derive(RustcDecodable)]
struct StoreRequest {
    key: String,
    value: String,
}

#[derive(RustcEncodable)]
struct Response {
    succesful: bool,
    message: String,
}

fn main() {
    let server_address = "localhost:3000".to_string();

    start_server(server_address);
}

fn start_server(server_address: String) {
    let mut router = Router::new();
    router.post("/store", store, "store");

    Iron::new(router).http(server_address).unwrap();
}

/*
curl --request POST \
  --url http://localhost:3000/store \
  --header 'cache-control: no-cache' \
  --header 'content-type: application/json' \
  --header 'postman-token: 0e894bbb-9a7c-b369-aa56-c5263a453198' \
  --data '{\n   "key": "key",\n "value": "value"\n}'
  */
fn store(request: &mut Request) -> IronResult<iron::Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let store_request: StoreRequest = json::decode(&payload).unwrap();

    let response = Response {
        succesful: true,
        message: "Stored".to_string(),
    };
    let encoded_response = json::encode(&response).unwrap();

    Ok(iron::Response::with((status::Ok, encoded_response)))
}
