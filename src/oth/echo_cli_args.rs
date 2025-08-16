// echo_args.rs
use std::env;

fn main() {
    for (i, arg) in env::args().enumerate() {
        println!("arg[{}] = {}", i, arg);
    }
}
