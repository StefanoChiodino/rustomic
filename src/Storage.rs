use data::Data;
use std::collections::HashMap;
use chrono::offset::local::Local;

pub struct Storage<T> {
    datum: HashMap<String, Vec<Data<T>>>,
}

impl<T> Storage<T> {
    pub fn new() -> Storage<T> {
        Storage { datum: HashMap::new() }
    }

    pub fn store(&mut self, key: String, value: T) {
        let mut vector = self.datum.entry(key).or_insert(Vec::new());

        let data = Data {
            date_time: Local::now(),
            value: value,
        };

        vector.push(data);
    }

    pub fn retrieve(&mut self, key: String) -> Option<&T> {
        let last = self.datum.get(&key).unwrap().last();
        last.map(|ref mut x| &x.value)
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
