// fib.rs
use std::env;

struct Fib {
    a: u128,
    b: u128,
}

impl Fib {
    fn new() -> Self { Fib { a: 0, b: 1 } }
}

impl Iterator for Fib {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.a;
        self.a = self.b;
        self.b = n + self.b;
        Some(n)
    }
}

fn main() {
    let n: usize = env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(10);
    for (i, val) in Fib::new().take(n).enumerate() {
        println!("{}: {}", i, val);
    }
}
