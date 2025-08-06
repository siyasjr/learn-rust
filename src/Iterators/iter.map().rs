// This function demonstrates use of .iter() and .map() to transform a vector, then prints both the mapped and original values using separate iterators.
fn main (){
    let v1 = vec![1,2,3];
    let iter = v1.iter();
    let iter2 = iter.map(|x| x + 1 );
     println!("{:?}", &iter2);
    for val in iter2 {
        println!("{}", val);
    }

    let iter3 = v1.iter();
    for val in iter3 {
        println!("Hello there {} times", val)

    }

    
}