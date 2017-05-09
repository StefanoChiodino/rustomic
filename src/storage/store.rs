impl<T> Storage<T> {
    pub fn store(&mut self, key: String, value: T) {
        self.store_at_date_time(key, value, Local::now());
    }

    pub fn store_at_date_time(&mut self, key: String, value: T, date_time: DateTime<Local>) {
        let mut vector = self.datum.entry(key).or_insert(Vec::new());

        let data = Data {
            date_time: date_time,
            value: value,
        };

        vector.push(data);
    }
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
fn can_store_at_specific_time() {
    let mut storage = Storage::new();
    storage.store_at_date_time("key".to_string(), 1, Local::now());
}