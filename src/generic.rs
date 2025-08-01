/*enum Result<T, E> {
    Ok(T),
    Err(E),
}*/

struct Generic<T> {
    x: T,
    y: T,
}

fn main(){
    let generic: Generic = Generic{
        x:5,
        y,5
    };

    println!("Generic points{},{}," , generic.x, generic.y);
}

//Learning generics syntax and simple implementation in Rust.