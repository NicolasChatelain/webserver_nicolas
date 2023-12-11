use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workday {
    day: String,
    minutes: i32,
}

impl Workday {
    pub fn new(day: String, minutes: i32) -> Workday {
        Workday{day, minutes}
    }
}