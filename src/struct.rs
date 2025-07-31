struct User {
    name: String,
    age: u32,
    active: bool
}

fn main (){
    let user1 = User {
        name: String::from("Tyrian Lannister"),
        age: 32,  
        active: true
    };
println!("The age of {} is {}" , user1.name, age);




}