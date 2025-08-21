use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    println!("Enter second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read line");

    let a: i32 = num1.trim().parse().expect("Not a number");
    let b: i32 = num2.trim().parse().expect("Not a number");

    println!("Sum = {}", a + b);
}
