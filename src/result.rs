use std::fs;

fn main() {
    let res = fs::read_to_string("/home/siy/rust/learn-rust/src/example.txt");

    match res {
        Ok(content) => println!("{}", content),

        
        Err(err) => println!("The error is::{}", err), 
        
    }

    println!("This is an extra line");
    

}
//learned how Result enum works