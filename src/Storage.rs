use data::Data;
use std::collections::HashMap;
use chrono::offset::local::Local;
use chrono::prelude::DateTime;

pub struct Storage<T> {
    datum: HashMap<String, Vec<Data<T>>>,
}

impl<T> Storage<T> {
    pub fn new() -> Storage<T> {
        Storage { datum: HashMap::new() }
    }

    pub fn store(&mut self, key: String, value: T) {
        self.store_at_time(key, value, Local::now());
    }

    pub fn store_at_time(&mut self, key: String, value: T, date_time: DateTime<Local>) {
        let mut vector = self.datum.entry(key).or_insert(Vec::new());

        let data = Data {
            date_time: date_time,
            value: value,
        };

        vector.push(data);
    }

    pub fn retrieve(&mut self, key: String) -> Option<&T> {
        let vector = self.datum.get(&key);
        match vector {
            Some(v) => v.last().map(|ref mut x| &x.value),
            None => None,
        }
    }
}

#[test]
fn can_create_storage() {
    let _ = Storage::<i32>::new();
}

#[test]
fn can_store_data() {
    let mut storage = Storage::new();
    storage.store("key".to_string(), 1);
}

#[test]
fn can_store_multiple_data() {
    let mut storage = Storage::new();
    storage.store("key".to_string(), 1);
    storage.store("key".to_string(), 2);
}

#[test]
fn can_retrieve_latest_data() {
    let mut storage = Storage::new();
    storage.store("key".to_string(), 1);
    storage.store("key".to_string(), 2);

    let latest_value = storage.retrieve("key".to_string()).unwrap();

    assert_eq!(*latest_value, 2);
}

#[test]
fn retrieve_on_no_data_returns_none() {
    let mut storage = Storage::<i32>::new();

    let result = storage.retrieve("key".to_string());

    assert_eq!(result, None);
}

#[test]
fn can_store_at_specific_time() {
    let mut storage = Storage::new();
    storage.store_at_time("key".to_string(), 1, Local::now());
}

#[test]
fn can_retrieve_at_specific_time() {}
