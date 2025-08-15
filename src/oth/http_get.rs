// http_get.rs
use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;

fn main() {
    let host = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: http_get <host> (e.g. example.com)");
        exit(1);
    });

    let addr = format!("{}:80", host);
    let mut stream = TcpStream::connect(&addr).unwrap_or_else(|e| {
        eprintln!("Failed to connect {}: {}", addr, e);
        exit(1);
    });

    let req = format!("GET / HTTP/1.0\r\nHost: {}\r\n\r\n", host);
    stream.write_all(req.as_bytes()).unwrap();

    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
}
