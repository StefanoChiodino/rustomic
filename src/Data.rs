use chrono::prelude::DateTime;
use chrono::offset::local::Local;

pub struct Data<T> {
    pub date_time: DateTime<Local>,
    pub value: T,
}
