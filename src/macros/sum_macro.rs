// main.rs

macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let mut total = 0;
            $(
                total += $x;
            )*
            total
        }
    };
}

fn main() {
    let total1 = sum!(1, 2, 3);
    let total2 = sum!(10, 20, 30, 40);

    println!("Total1: {}", total1);
    println!("Total2: {}", total2);
}
