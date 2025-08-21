use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        print!("Enter text (type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed = input.trim();
        if trimmed == "exit" {
            break;
        }
        println!("You entered: {}", trimmed);
    }
}
