//This function demonstrates manual iteration over a mutable iterator using while let and .next(), showing how Rust handles iterator logic under the hood, unlike the syntactic sugar of a for loop.
// Rust expects us to write iter like in this logic, but the for loop hides most of the complexity beneath. 

fn main(){
    let mut v1 = vec![7,8,9];

    let mut v1_iter = v1.iter_mut();

    while let Some(val) = v1_iter.next(){  //Returns Option <&mut i32> // Iterating over an iterator // 
        println!("{}", val)
    }
}


/* 
fn main(){
    let mut v1 = vec![7,8,9];

    let mut v1_iter = v1.iter_mut();

    let num1 = v1_iter.next();      // Returns Option <&mut i32> / Option <None>
    let num2 = v1_iter.next();      // Iterating over an iterator     
    let num3 = v1_iter.next();
    let num4 = v1_iter.next();

    println!("{:?}, {:?}, {:?}, {:?}" , num1, num2, num3, num4); 
}


// Out : Some(7), Some(8), Some(9), None

*/