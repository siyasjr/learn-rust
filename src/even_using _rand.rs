#[derive(Debug)]
use rand::Rng;


enum Oddoreven {
        Odd,
        Even,
    }


fn main(){
    let mut rng = rng(); // renamed from thread_rng()
    let num: i32 = rng.random_range(0..1000); // renamed from gen_range()
    println!("{} is {:?}", num,is_even(num) );
}

fn is_even(num: i32) -> Oddoreven {

    if num % 2 == 0 {
        return Oddoreven::Even;
        
    }
    return Oddoreven::Odd
}