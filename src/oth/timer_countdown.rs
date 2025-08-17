
use std::{env, thread, time};

fn main() {
    let secs: u64 = env::args().nth(1).unwrap_or("5".into()).parse().unwrap();
    for i in (1..=secs).rev() {
        println!("{}...", i);
        thread::sleep(time::Duration::from_secs(1));
    }
    println!("Time's up!");
}
