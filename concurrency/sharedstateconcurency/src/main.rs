// SHARED STATE CONCURENCY
//
// Using Mutexes to Allow Access to Data from One Thread at a Time
//
// Mutexes have a reputation for being difficult to use because you
// have to remember two rules:
//
//  You must attempt to acquire the lock before using the data.
//  When you’re done with the data that the mutex guards, you must unlock the data
//      so other threads can acquire the lock.
//
//mutex -> mutual exclusion

use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result is: {}", *counter.lock().unwrap());
}
