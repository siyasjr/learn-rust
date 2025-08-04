//simple interactive cli  adder using std::io

use std::io;

fn main(){
    let mut input: String = String::from("");
    println!("Enter the first number");


    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Enter a valid number");

    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b:i32 = input.trim().parse().expect("Enter a valid number");

    let result = a + b ;
    println!("{} + {} = {}", a,b,result);
 

}