use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a sentence:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Length (chars): {}", input.trim().chars().count());
}
