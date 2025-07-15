fn main(){ 
    println!( "{}" , is_even(5662255545215566628846278628488520));
}

fn is_even(num: i128) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false
}
