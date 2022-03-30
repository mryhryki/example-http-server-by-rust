use std::io::Read;
use std::net::{TcpListener, TcpStream};

struct HttpRequest {
    method: String,
    pathname: String,
    http_version: String,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut cursor = 0;
    let mut buf = [0; 1024];
    let mut len = stream.read(&mut buf).unwrap();

    while len > 0 {
        for i in 0..len {
            if buf[i] == 13 && buf[i + 1] == 10 {
                println!("{:#?}", String::from_utf8(buf[cursor..(i)].to_vec()).unwrap());
                cursor = i + 2;
            }
        }
        len = stream.read(&mut buf).unwrap();
    }
}
