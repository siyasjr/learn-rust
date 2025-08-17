// factorial.rs
use std::env;

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

fn main() {
    let n: u128 = env::args().nth(1).unwrap_or("5".into()).parse().unwrap();
    println!("{}! = {}", n, factorial(n));
}
