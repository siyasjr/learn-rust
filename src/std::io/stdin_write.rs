use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout();
    write!(stdout, "This is written using write! macro\n").unwrap();
    stdout.flush().unwrap();
}
