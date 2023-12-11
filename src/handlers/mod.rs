use std::{fs, thread};
use std::net::TcpStream;
use std::io::*;
use std::str;
use std::time::Duration;

use super::database;


pub fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();
    let mut request: &str = "";

    if let Ok(utf8_str) = str::from_utf8(&buffer) {
        if let Some(request_first_line) = utf8_str.lines().next() {
            request = request_first_line;
        }
    } else {
        eprintln!("Invalid UTF8 data.")
    }

    let response = handle_request(request);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_request(request: &str) -> String {

    let (status, filename) = match request {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /style.css HTTP/1.1" => ("HTTP/1.1 200 OK", "style.css"),
        "GET /slowrequest HTTP/1.1" => {
            thread::sleep(Duration::from_secs(7));
            ("HTTP/1.1 200 OK", "slowrequest.html")
        },
        "GET /about HTTP/1.1" => ("HTTP/1.1 200 OK", "about.html"),
        "GET /time HTTP/1.1" => {
            database::get_total_time();
            ("HTTP/1.1 200 OK", "time.html")
        },
        "GET /times.js HTTP/1.1" => ("HTTP/1.1 200 OK", "times.js"),
        "GET /data.json HTTP/1.1" => ("HTTP/1.1 200 OK", "data.json"),
        _ => ("HTTP/1.1 404 NOT FOUND", "NotFound.html")
    };

    let body = fs::read_to_string(filename).unwrap();
    let body_length = body.len();

    let http_response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",status, body_length, body);

    http_response
}
