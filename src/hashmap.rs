use std::collections::HashMap;

/// A macro to create a `Vec<String>` from a list of string literals.
/// This is a simple example to show the basic structure of a macro.
#[macro_export]
macro_rules! vec_str {
    // The `$(...)*` syntax is used for repetition.
    // It will match zero or more expressions, separated by commas.
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // For each expression matched, push it into the vector as a String.
            $(
                temp_vec.push(String::from($x));
            )*
            temp_vec
        }
    };
}

/// A macro to calculate the average of a series of numbers.
/// This demonstrates handling a variable number of arguments and performing calculations.
#[macro_export]
macro_rules! avg {
    // Match one or more expressions separated by commas.
    ( $( $x:expr ),+ ) => {
        {
            let mut sum = 0.0;
            let mut count = 0.0;
            // This block will be expanded for each expression passed to the macro.
            $(
                sum += $x as f64;
                count += 1.0;
            )+
            // The final expression in the block is the return value.
            sum / count
        }
    };
}

/// A macro for creating a `HashMap` from a list of key-value pairs.
/// This is similar to how `vec!` works but for HashMaps.
#[macro_export]
macro_rules! hashmap {
    // The pattern matches key-value pairs separated by commas.
    // `$key:expr => $value:expr` defines the structure of each pair.
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut map = HashMap::new();
            // This block repeats for each key-value pair.
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}


fn main() {
    // --- 1. Using the `vec_str!` macro ---
    // This looks cleaner than manually pushing to a vector.
    let string_vec = vec_str!["hello", "world", "from", "a", "macro"];
    println!("vec_str! result: {:?}", string_vec);
    println!("--------------------");

    // --- 2. Using the `avg!` macro ---
    // Calculate the average of a few numbers.
    let average1 = avg![1, 2, 3, 4, 5];
    println!("Average of integers: {}", average1);

    // It also works with floating-point numbers.
    let average2 = avg![10.5, 20.0, 5.5];
    println!("Average of floats: {}", average2);
    println!("--------------------");

    // --- 3. Using the `hashmap!` macro ---
    // Create a HashMap with string keys and integer values.
    let person_ages = hashmap! {
        "Alice" => 30,
        "Bob" => 25,
        "Charlie" => 35
    };
    println!("hashmap! result: {:?}", person_ages);

    // You can also create an empty one.
    let empty_map: HashMap<i32, i32> = hashmap!{};
    println!("Empty hashmap: {:?}", empty_map);
}
