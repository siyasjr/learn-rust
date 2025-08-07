/*This program defines a User struct with a lifetime parameter 'a, ensuring the name reference lives long enough.
It safely borrows a String (first_name) and prints the name field. */


struct User {
    name: &str

}

fn main(){
    let first_name = String::from("Anas");
    let user = User{
        name: &first_name
    };

    println!("{}", user.name);
}


// out: Anas
