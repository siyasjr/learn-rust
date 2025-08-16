// wc.rs
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Usage: wc <filename>");
    let content = fs::read_to_string(&filename).expect("Failed to read file");

    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();

    println!("{} {} {} {}", lines, words, chars, filename);
}
