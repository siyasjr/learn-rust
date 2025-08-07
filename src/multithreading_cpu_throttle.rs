/// A simple function to simulate constant CPU usage by running an infinite loop.
/// Used to demonstrate how multithreading can throttle CPU when run in multiple threads.

use std::thread;

fn busy_loop() {
    loop {
        // Some meaningless CPU work
        let _ = 2 * 2;
    }
}

fn main() {
    // Spawn 3 threads to consume ~300% CPU
    for _ in 0..3 {
        thread::spawn(|| {
            busy_loop();
        });
    }

    // Keep main thread alive
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
