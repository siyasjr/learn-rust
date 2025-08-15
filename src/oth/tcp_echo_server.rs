// tcp_echo_server.rs
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    println!("Echo server listening on 127.0.0.1:4000");

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Client connected: {}", s.peer_addr().unwrap());
                let mut buf = [0u8; 512];
                loop {
                    let n = match s.read(&mut buf) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => n,
                    };
                    s.write_all(&buf[..n]).ok();
                }
                println!("Client disconnected");
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
