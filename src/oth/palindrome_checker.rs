
use std::env;

fn main() {
    let s = env::args().nth(1).expect("Usage: palindrome <string>");
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    let rev: String = cleaned.chars().rev().collect();

    if cleaned == rev {
        println!("\"{}\" is a palindrome", s);
    } else {
        println!("\"{}\" is not a palindrome", s);
    }
}
