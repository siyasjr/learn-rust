use rand::{Rng, rng};
#[derive(Debug)]

enum OddorEven {
    Odd, 
    Even,
}

fn main (){
    let mut rng = rng();
    let num: i32 = rng.random_range(0..5000);
    println!("{} is {:?}", num, is_even(num) );
}

fn is_even(num: i32) -> OddorEven{
    if num % 2 ==0 {
        return OddorEven::Even;
    }
    return OddorEven::Odd
 }

