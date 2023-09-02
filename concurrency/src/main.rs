/*
* Concurrent programming, where different parts of a program execute independently,
* and parallel programming, where different parts of a program execute at the same time,
*
* Here are the topics we’ll cover in this chapter:

    How to create threads to run multiple pieces of code at the same time
    Message-passing concurrency, where channels send messages between threads
    Shared-state concurrency, where multiple threads have access to some piece of data
    The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as
        well as types provided by the standard library

*
*   To create a new thread, we call the thread::spawn function and pass it a closure
*   containing the code we want to run in the new thread.
*
*/

use std::{
    thread::{sleep, spawn},
    time::Duration,
};

// Waiting for All Threads to Finish Using join Handles
fn finish_all_threads() {
    // -> Saving a JoinHandle from thread::spawn to guarantee the thread is run to completion
    let handle = spawn(|| {
        for i in 1..10 {
            println!("HELLO number {} from spawned thread", i);
            sleep(Duration::from_millis(1));
        }
    });

    // main thread
    for i in 1..5 {
        println!("HELLO number {} from the MAIN thread", i);
        sleep(Duration::from_millis(1));
    }

    handle.join().unwrap_or_default();
}

// Using move Closures with threads
// passing values between thread using 'move' in Closures
fn movbtwnthreads() {
    let values: Vec<i32> = vec![1, 2, 3, 4];

    let handle = spawn(move || {
        for v in &values {
            println!("Element {}", v);
        }
    });
    // drop(values); -> value moved inside closure so this won't work!
    handle.join().unwrap_or_default();
}

fn main() {
    // spawned thread
    /* spawn(|| {
        for k in 1..10 {
            println!("HI number {} from the spawned thread", k);
            sleep(Duration::from_millis(1));
        }
    });

    // main thread
    for j in 1..5 {
        println!("HI number {} from the main thread", j);
        sleep(Duration::from_millis(1));
    }
    */

    // the spawned thread stops immediately the main thread stops and this is How
    // we fix it to make sure the spawned thread runs to completion
    finish_all_threads();
    println!("\n\n");
    movbtwnthreads();
}
