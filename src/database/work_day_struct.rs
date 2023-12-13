use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workday {
    day: String,
    minutes: i32,
}

impl Workday {
    pub fn new(day: String, minutes: i32) -> Workday {
        Workday{day, minutes}
    }
}