// Write the logic to filter out all the odd values from a i32 vec and double the value and store it in a new vec

fn main(){
    let v1 = vec![1,2,3];
    let ans = filter_and_map(v1);
    println!("{:?}", ans)
}

fn filter_and_map(v: Vec<i32>) -> Vec<i32>{
    let v_iter = v.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);
    let new_vec = v_iter.collect(); // converts back the iterator to a vec. 
    new_vec
}


// out: [2, 6]