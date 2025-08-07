/*This function demonstrates how lifetime annotations work in structs.
 It shows that assigning a reference (&str) from a shorter-lived variable 
 (like last_name inside a block) to a struct field results in a lifetime error,
 because the borrowed value does not live long enough. */

struct User<'a, 'b>{
    first_name: &'a str,
    last_name: &'b str
}

fn main(){
    let user;
    let first_name = String::from("Anas");
    {
        let last_name = String::from("Zaman");
        user = User{
            first_name: &first_name,
            last_name: &last_name, // `last_name` does not live long enough
                                   //  borrowed value does not live long enough
        }

    }
    println!("{}", user.first_name)
}