mod work_day_struct;

use std::fs::File;
use std::io::Write;
use work_day_struct::Workday;

use mongodb::sync::Client;
use mongodb::bson::{doc, Document};
use mongodb::sync::Database;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Value;

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
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    let json_vec = serde_json::to_string(&workdays).unwrap();
    file.write_all(json_vec.as_bytes()).unwrap();
}

pub fn post_new_time(body: &str) {

    let trimmed_body = body.trim_matches(char::from(0)); // door de 'fixed size' byte vector (1024) moet ik de extra values weg trimmen.
    let parsed_data: Result<Value, _> = serde_json::from_str(trimmed_body);

    match parsed_data {
        Ok(json_value) => {

            let document: Document = doc! {
                "day": json_value["day"].as_str().unwrap_or_default(),
                "minutes": json_value["minutes"].as_str().unwrap_or("0").parse::<i32>().unwrap_or_default(),
            };


            let db = connect_to_database();
            let result = db.collection::<Document>("workhours").insert_one(document, None).unwrap();
            println!("{}", result.inserted_id);
        }
        Err(e) => {
            eprintln!("ERROR PARSING JSON: {}", e);
            return;
        }
    }

}
