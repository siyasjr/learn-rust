// Write a function that takes a vec as a input and returns a vector with only even values . say 1...10
// Approch 2
fn main(){

    let mut vec = vec![1,2,3,4,5,6,7,8,9,10];
    println!("{:?}", even_values(&mut vec) );
}

fn even_values(vec: &mut Vec<i32>)-> &Vec<i32> {

    
    for val in (0..vec.len()).rev() {
        if val % 2 != 0 {
            vec.remove(val);

        }
    }
    return vec; 

}