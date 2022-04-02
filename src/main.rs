use crate::http::{HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

mod http;

fn main() -> std::io::Result<()> {
    let port = 8080;
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("HTTP server started, port: {}", port);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let response = handle_client(&stream);

        let mut response_buf: Vec<u8> = vec![];
        response_buf.append(
            &mut format!(
                "{} {} {}\r\n",
                response.http_version, response.status_code, response.status_message
            )
            .as_bytes()
            .to_vec(),
        );
        for (key, val) in response.headers {
            response_buf.append(&mut format!("{}: {}", key, val).as_bytes().to_vec());
        }
        match response.body {
            Some(body) => {
                response_buf.append(&mut "\r\n".as_bytes().to_vec());
                response_buf.append(&mut body.to_vec());
            }
            None => (),
        }
        stream.write(response_buf.as_slice()).unwrap();
    }
    Ok(())
}

fn handle_client(stream: &TcpStream) -> HttpResponse {
    let request = parse_request(stream);

    if request.http_version != "HTTP/1.1" {
        return HttpResponse {
            status_code: 505,
            status_message: String::from("HTTP Version Not Supported"),
            http_version: request.http_version,
            headers: HashMap::new(),
            body: None,
        };
    }

    if ![
        String::from("GET"),
        String::from("HEAD"),
        String::from("POST"),
    ]
    .contains(&request.method)
    {
        return HttpResponse {
            status_code: 405,
            status_message: String::from("Method Not Allowed"),
            http_version: request.http_version,
            headers: HashMap::new(),
            body: None,
        };
    }

    HttpResponse {
        status_code: 200,
        status_message: String::from("OK"),
        http_version: request.http_version,
        headers: HashMap::new(),
        body: Some(b"<h1>Hello, World!</h1>"),
    }
}

fn parse_request(stream: &TcpStream) -> HttpRequest {
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
