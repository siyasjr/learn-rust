// This program creates a String, takes a string slice (&str) from index 0 to 5, and prints it. It demonstrates how to slice a String using byte indices.

fn main(){

    let st = String::from("Ajmal Siyas");
    let st1: &str = &st[0..5];
    println!("{}", st1);
}


// out: Ajmal