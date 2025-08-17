// reverse.rs
use std::env;

fn main() {
    let input = env::args().nth(1).expect("Usage: reverse <string>");
    let rev: String = input.chars().rev().collect();
    println!("{}", rev);
}
