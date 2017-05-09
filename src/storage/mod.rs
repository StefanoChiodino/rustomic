use data::Data;
use std::collections::HashMap;
use chrono::offset::local::Local;
use chrono::prelude::DateTime;
use chrono::Duration;

pub struct Storage<T> {
    datum: HashMap<String, Vec<Data<T>>>,
}

impl<T> Storage<T> {
    pub fn new() -> Storage<T> {
        Storage { datum: HashMap::new() }
    }
}

#[test]
fn can_create_storage() {
    let _ = Storage::<i32>::new();
}