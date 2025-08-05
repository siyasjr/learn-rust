
// Stores struct instances in a vector and iterates over them to print user details.

struct User {
    name: String,
    age: i32,
}

fn main(){
    let users = vec![ 
        User{ name: String::from("Fayyaz"), age:27 },
        User{ name: String::from("Faris"), age: 23 },


    ];
    for user in users{
    println!("{} is {} years old" , user.name, user.age);

    }
}    