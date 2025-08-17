// guess.rs
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn random() -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 10 + 1) as u32
}

fn main() {
    let secret = random();
    loop {
        print!("Enter your guess (1-10): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(n) if n == secret => {
                println!("You got it!");
                break;
            }
            Ok(_) => println!("Wrong, try again."),
            Err(_) => println!("Please enter a number."),
        }
    }
}
