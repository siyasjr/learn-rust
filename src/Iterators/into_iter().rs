// This function consumes a vector using .into_iter(), enumerates over the values with their indices, and prints them. It highlights that the original vector cannot be used after being moved.

fn main(){
    let v1 = vec![10, 20, 30];
let v1_iter = v1.into_iter();  // v1 is moved here

for (i, val) in v1_iter.enumerate() {
    println!("Index: {}, Value: {}", i, val);
    // println!("{:?}",v1);  // causes error
}

}


// into_iter() same as using " for val in v1 {} ""


/* Out: 
 Index: 0, Value: 10
Index: 1, Value: 20
Index: 2, Value: 30*/