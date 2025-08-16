// rand_num.rs
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let rand = (seed % 101) as u32;
    println!("Random number (0-100): {}", rand);
}
