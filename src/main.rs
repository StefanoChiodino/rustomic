#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rocket_contrib;


pub mod api;

fn main() {
    api::start();
}
