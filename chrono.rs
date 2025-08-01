use chrono::{Local, Utc};

fn main(){
    let now = Utc::now();
    let local = Local::now();
    println!("Current date and time in UTC is {}", now);
    println!("Current date and time in IST is {}", local);

}

// external libraries from crates.io -- UTC and Local time using chrono