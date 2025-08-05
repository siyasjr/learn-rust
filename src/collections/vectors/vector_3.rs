// W=rite a function that takes a vec as a input and returns a vector with only even values . say 1...10;z

    fn main(){

    let  vec = vec![1,2,3,4,5,6,7,8,9,10];
    println!("Filtered vector: {:?}" ,filter_even(&vec));
    println!("Original vector: {:?}" , vec)
 
}


fn filter_even(vec:&Vec<i32>) -> Vec<i32> {

    let mut ans = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            ans.push(*val);

        }
        
    }
    return ans;

   
}