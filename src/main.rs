#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::{JSON, Value};

#[derive(Deserialize)]
struct StoreRequest {
    key: String,
    value: String,
}

#[derive(Serialize,)]
struct Response {
    succesful: bool,
    message: String,
}

fn main() {
    rocket::ignite().mount("/", routes![store]).launch();
}

#[post("/store", data = "<request>")]
fn store(request: JSON<StoreRequest>) -> JSON<Response> {
    let store_request: StoreRequest = request.0;

    let response = Response {
        succesful: true,
        message: "Stored".to_string(),
    };

    JSON(response)
}
