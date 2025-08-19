use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    // A list of books
    let library = vec![
        Book { title: "The Rust Book".to_string(), author: "Steve Klabnik".to_string(), pages: 552 },
        Book { title: "Programming in Rust".to_string(), author: "Jim Blandy".to_string(), pages: 620 },
    ];

    // Serialize the vector of books into JSON
    let serialized = serde_json::to_string_pretty(&library).unwrap();
    println!("Serialized JSON:\n{}", serialized);

    // Deserialize back into a vector of Book
    let deserialized: Vec<Book> = serde_json::from_str(&serialized).unwrap();
    println!("\nDeserialized struct: {:?}", deserialized);
}
