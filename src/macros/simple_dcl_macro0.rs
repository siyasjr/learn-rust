//Function to demonstrate macro_rules! for defining and invoking a simple declarative macro in Rust
macro_rules! say_hello {
    () => {
        println!("Say Hello!!")
    };
}

fn main(){
    say_hello!();
}







