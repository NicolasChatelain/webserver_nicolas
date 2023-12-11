mod work_day_struct;

use std::fs::File;
use std::io::Write;
use work_day_struct::Workday;

use mongodb::sync::Client;
use mongodb::bson::Document;
use mongodb::sync::Database;

use serde::Serialize;

fn connect_to_database() -> Database {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let database = client.database("Rust-times");
    database
}

pub fn get_total_time() {
    let db = connect_to_database();
    let collection = db.collection::<Document>("workhours");

    let cursor = collection.find(None, None).unwrap();

    let mut workdays: Vec<Workday> = Vec::new();
    let mut file = File::create("data.json").unwrap();

    for result in cursor {
        match result {
            Ok(document) => {

                let day = document.get_str("day").unwrap();
                let minutes = document.get_i32("minutes").unwrap();
                let workday = Workday::new(day.to_string(), minutes);

                workdays.push(workday);
            },
            Err(e) => eprintln!("{e}"),
        }
    }

    let json_vec = serde_json::to_string(&workdays).unwrap();
    file.write_all(json_vec.as_bytes()).unwrap();

}

// fn calculate_total_time(minutes_vector: Vec<i32>) -> String {
//     let total_minutes: i32 = minutes_vector.iter().sum();
//     let hours = total_minutes / 60;
//     let minutes = total_minutes % 60;
//
//     format!("{} hours and {} minutes", hours, minutes)
// }