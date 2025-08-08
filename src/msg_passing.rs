//Experiment with mpsc channel for basic message passing between threads


use std::{
    sync::mpsc,
    thread::spawn,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    spawn(move || {
        tx.send(String::from("Hello World")).unwrap(); // Handle send result
    });

    match rx.recv() {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Error reading data"),
    }
}
