use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    // Create a struct instance
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // Serialize struct to JSON string
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", serialized);

    // Deserialize JSON string back to struct
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized struct: {:?}", deserialized);
}
