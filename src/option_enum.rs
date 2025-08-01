// to handle nullability

 /*pub enum Option<T>{
    Some(T),
    None,
 }*/


  fn main(){
    let str: String = String::from("I challnge you to find the first 'a' ");
    let res:Option<usize> = find_first_a(str);
    match res {
        Some(ind) => println!("The index of the first 'a' is {}",ind ),
        None => println!("No 'a' exists in the given string"),
    }

 }

 fn find_first_a(str: String) -> Option<usize>{
    for (index, character) in str.chars().enumerate() {
        if character == 'a'{
            return Some(index as usize);
    }
}
    return None
 }
