// This Rust code demonstrates basic multithreading using std::thread::spawn along with ownership transfer using move. Here's a detailed explanation:
use std::thread;

fn main(){
    let v = vec![1,2,3];

    let handle = thread::spawn( move || {
        println!("Here is a vector {:?}", v);
    });
    handle.join().unwrap();
}

// out: Here is a vector [1, 2, 3]
