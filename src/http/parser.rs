use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use crate::http::structs::HttpRequest;

pub fn parse_http_request(stream: &TcpStream) -> HttpRequest {
    let reader = BufReader::new(stream);

    let mut lines_iter = reader.lines();
    let mut line = lines_iter.next().unwrap().unwrap();

    let mut arr = line.split(" ");
    let method = String::from(arr.next().unwrap_or("UNKNOWN"));
    let pathname = String::from(arr.next().unwrap_or("?"));
    let http_version = String::from(arr.next().unwrap_or("UNKNOWN_HTTP_VERSION"));

    let mut headers = HashMap::new();
    while line != "" {
        line = lines_iter.next().unwrap().unwrap();
        let mut arr = line.split(":");
        let key = arr.next().unwrap_or("").trim().to_ascii_lowercase();
        let val = arr.next().unwrap_or("").trim();
        if key != "" && val != "" {
            headers.insert(String::from(key), String::from(val));
        }
    }

    HttpRequest {
        method,
        pathname,
        http_version,
        headers,
    }
}
