// Function to understand:
// - Lifetimes of two input references
// - How to declare lifetime specifiers
// - How to combine them with trait bounds like `Display`

use std::fmt::Display;

fn announce<'a, T> (x: &'a str, y: &'a str,ann: T, ) -> &'a str where  T:Display, {

        println!("Announce! {ann}");
        if x.len() > y.len() {
            x
        }
        else {
            y
        }

    }

    fn main(){
        let short = String::from("short");
        let long = String::from("longest");
        let ann = String::from("Announcement!");
        let ans = announce(&short, &long,ann);
        println!("{}", ans)
        
    }

//  out: Announce! Announcement!
//       longest
