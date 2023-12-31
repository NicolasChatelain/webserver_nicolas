use std::{fs, thread};
use std::net::TcpStream;
use std::io::*;
use std::str;
use std::time::Duration;
use crate::database;


pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0;1024];
    let _ = stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer);

    let headers_end = request_str.find("\r\n\r\n").unwrap_or(0);
    let headers = &request_str[..headers_end + 4];
    let body = &request_str[headers_end + 4..];

    let request_header = headers.lines().next().unwrap();
    let mut response= String::new();

    if request_header.starts_with("GET") {
        response = handle_get_request(request_header);
    } else if request_header.starts_with("POST") {
        response = handle_post_request(request_header, body);
    } else {
        eprintln!("BAD REQUEST: {}", request_header);
    }

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_get_request(request: &str) -> String {
    let (status, filename) = match request {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "client/html/index.html"),
        "GET /css/style.css HTTP/1.1" => ("HTTP/1.1 200 OK", "client/css/style.css"),
        "GET /slowrequest HTTP/1.1" => {
            thread::sleep(Duration::from_secs(3));
            ("HTTP/1.1 200 OK", "client/html/slowrequest.html")
        }
        "GET /about HTTP/1.1" => ("HTTP/1.1 200 OK", "client/html/about.html"),
        "GET /time HTTP/1.1" => {
            database::get_total_time();
            ("HTTP/1.1 200 OK", "client/html/time.html")
        }
        "GET /javascript/times.js HTTP/1.1" => ("HTTP/1.1 200 OK", "client/javascript/times.js"),
        "GET /data.json HTTP/1.1" => {
            database::get_total_time();
            ("HTTP/1.1 200 OK", "data.json")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "client/html/NotFound.html")
    };

    make_http_response(status, filename)
}

fn handle_post_request(request: &str, body: &str) -> String {

    let (status, filename) = match request {
        "POST /time/new HTTP/1.1" => {
            if database::post_new_time(body) {
                database::get_total_time();
                ("HTTP/1.1 201 CREATED", "client/html/time.html")
            } else {
                ("HTTP/1.1 500 INTERNAL SERVER ERROR", "client/html/time.html")
            }
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "client/html/NotFound.html")
    };

    make_http_response(status, filename)
}

fn make_http_response(status_line: &str, filename: &str) -> String {

    let body = fs::read_to_string(filename).unwrap();
    let body_length = body.len();

    let http_response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, body_length, body);

    http_response

}
