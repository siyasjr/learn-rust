//This creates a vector, calculates the sum of its elements using .iter().sum(), and then iterates over the vector again by

fn main(){
    let v1 = vec![1,2,3,4,5,6];
    let iter = v1.iter();
    let sum: i32 = iter.sum();

    println!("The sum is {}", sum);

     let iter2 = v1.iter() ;  // cannot access iter if used : let iter 2 = iter;  . Because sum() consumes iter.  But can create new iter from v1 ie v1.iter() and then use it.
     for val in iter2 { // 
        println!("{}", val)
    }   
}


/* Out: 
The sum is 21
1
2
3
4
5
6
 */