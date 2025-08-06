
fn main(){

    let v1 = vec![12,13,14,15];
     
    for val in v1.iter(){           // v1.iter() borrows v1 as reference( immutable) . cannot mutate. If muation needed use iter_mut()
        println!("{}", val)
    }
    println!("{:?}", v1);           // can use the original v1 because v1.iter() did not cconsume v1 but borrowed it.  
}


