// threaded_counter.rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} done", i);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final count = {}", *counter.lock().unwrap());
}
