use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut op = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut a).unwrap();
    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut op).unwrap();
    println!("Enter second number:");
    io::stdin().read_line(&mut b).unwrap();

    let x: f64 = a.trim().parse().unwrap();
    let y: f64 = b.trim().parse().unwrap();

    match op.trim() {
        "+" => println!("Result: {}", x + y),
        "-" => println!("Result: {}", x - y),
        "*" => println!("Result: {}", x * y),
        "/" => {
            if y != 0.0 {
                println!("Result: {}", x / y)
            } else {
                println!("Error: Division by zero")
            }
        }
        _ => println!("Unknown operator"),
    }
}
