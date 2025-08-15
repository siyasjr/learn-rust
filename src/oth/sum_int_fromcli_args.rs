// sum_args.rs
use std::env;

fn main() {
    let mut sum: i64 = 0;
    for arg in env::args().skip(1) {
        match arg.parse::<i64>() {
            Ok(n) => sum += n,
            Err(_) => eprintln!("Skipping non-int: {}", arg),
        }
    }
    println!("Sum = {}", sum);
}
