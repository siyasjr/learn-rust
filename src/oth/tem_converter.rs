// temp_conv.rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: temp_conv <c2f|f2c> <value>");
        return;
    }

    let mode = &args[1];
    let val: f64 = args[2].parse().unwrap();

    match mode.as_str() {
        "c2f" => println!("{}°C = {:.2}°F", val, val * 9.0/5.0 + 32.0),
        "f2c" => println!("{}°F = {:.2}°C", val, (val - 32.0) * 5.0/9.0),
        _ => eprintln!("Unknown mode, use c2f or f2c"),
    }
}
