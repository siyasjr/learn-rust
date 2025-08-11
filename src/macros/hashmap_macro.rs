// main.rs

use std::collections::HashMap;

macro_rules! map {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $val);
            )*
            m
        }
    };
}

fn main() {
    let my_map = map! {
        "name" => "Anas",
        "lang" => "Rust",
        "year" => 2025,
    };

    println!("{:?}", my_map);
}
