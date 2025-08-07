/* This program defines a trait Summary with a default implementation of summarize().

    User, Fix, and String implement Summary (relying on the default implementation).

    The function notify accepts any type that implements Summary and prints its summary. */

pub trait Summary{
    fn summarize(&self)-> String{
        return  String::from("Summarized");
    }
} 


struct User {
    name: String,
    age: u32, 
    address: String,
}

impl Summary for User {
    
}


struct Fix;
impl Summary for Fix {}
impl Summary for String {}



 fn notify(user: impl Summary){    // Anything any stucts that impliments the summary trait 
        println!("{}" ,user.summarize());
}

fn main(){

    let user1 = User{
        name: String::from("Anas"),
        age: 21,
        address: String::from("15/32"),
    };

    notify(user1);

    notify(String::from("AJ"));
}


// out: Summarized
//      Summarized