macro_rules! repeat_twice {
    ($x:expr) => {

     {   println!("{}", $x); 
        println!("{}", $x); 
    }   
        
        
    };
}

fn main(){
    repeat_twice!("Rust macros are really coool!!");
}
