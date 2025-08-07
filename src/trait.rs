// This program defines a Summary trait with a summarize method. A User struct implements this trait to return a formatted string summarizing the user's name and age.

pub trait Summary {
    fn summarize(&self) -> String;
        
    }


struct User {
    name: String,
    age: i32,
}




impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old" , self.name, self.age);
    }
}

fn main(){

    let user1 = User {
    name: String::from("Ajmal"),
    age : 27,
};

println!("{}", user1.summarize());

}

// out: User Ajmal is 27 years old