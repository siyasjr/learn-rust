macro_rules! demo {
    ($name:ident, $val:expr, $typ:ty) => {
        let $name: $typ = $val; 
       
    };
}

fn main(){
demo!(age, 25, u8);

println!("The Age is {}", age);



}