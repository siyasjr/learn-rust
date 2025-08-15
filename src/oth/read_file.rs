// cat.rs
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cat <filename>");
        exit(1);
    });

    match fs::read_to_string(&filename) {
        Ok(contents) => print!("{}", contents),
        Err(e) => eprintln!("Error reading {}: {}", filename, e),
    }
}
// cat.rs
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cat <filename>");
        exit(1);
    });

    match fs::read_to_string(&filename) {
        Ok(contents) => print!("{}", contents),
        Err(e) => eprintln!("Error reading {}: {}", filename, e),
    }
}
