// This function uses iter_mut().next() to get a mutable reference to the first element of a vector, then increments it using pattern matching. It finally prints the updated vector.
fn main(){
    let mut v1 = vec![6,8,9];

    let mut v1_iter = v1.iter_mut();

    let mut num1 = v1_iter.next();

    match num1 {
    Some(ref mut val) =>  {**val += 1 }, 
    None => println!("No value found")
    
    }
    println!("{:?}" , v1); 
}


// out: [7, 8, 9]
