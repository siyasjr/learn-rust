// write_file.rs
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: write_file <filename> [text]");
        exit(1);
    });
    let text = env::args().nth(2).unwrap_or_else(|| "Hello from Rust!".into());

    if let Err(e) = fs::write(&filename, text) {
        eprintln!("Failed to write {}: {}", filename, e);
    } else {
        println!("Wrote to {}", filename);
    }
}
