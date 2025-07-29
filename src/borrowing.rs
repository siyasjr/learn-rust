fn main() {
    let mut s1: String = String::from("Hello ");
    {
    let s2: &mut String = &mut s1;
    s2.push_str("World");
    println!("{}", s2);
    }
    
    
    println!("{}", s1);
    
    
}