use data::Data;

pub struct Storage<T> {
    data: Vec<Data<T>>,
}

impl<T> Storage<T> {
    pub fn new() -> Storage<T> {
        Storage { data: Vec::new() }
    }

    pub fn store(&mut self, key: String, value: T) {
        let data = Data {
            key: key,
            value: value,
        };
        self.data.push(data);
    }

    pub fn retrieve(&mut self, _: String) -> Option<&T> {
        let last = self.data.last();
        last.map(|ref mut x| &x.value)
        /*match last {
            Some(t) => return t.value,
            None => return None,
        }*/
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
