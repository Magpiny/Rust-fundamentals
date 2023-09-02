// Using Message Passing to Transfer Data Between Threads
//
// A channel is a general programming concept by which data is sent from one thread to another.
// A channel has two halves: a transmitter and a receiver.
// One part of your code calls methods on the transmitter with the data you want to send,
// and another part checks the receiving end for arriving messages.
// A channel is said to be closed if either the transmitter or receiver half is dropped.

use std::sync::mpsc::channel; // mpsc -> multiple producer, single consumer.
use std::thread::{sleep, spawn};
use std::time::Duration;

// Sending Multiple Values and Seeing the Receiver Waiting
fn send_multiple() {
    let (tx, rx) = channel();

    spawn(move || {
        let vals = vec![
            String::from("Value 1"),
            String::from("Value 2"),
            String::from("Value 3"),
            String::from("Value 3"),
            String::from("Value 4"),
        ];
        for val in vals {
            tx.send(val).unwrap_or_default();
            sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Creating Multiple Producers by Cloning the transmitter
//  mpsr() -> multiple producer single receiver
//
fn mpsr() {
    let (tm, rv) = channel();

    let tx = tm.clone();

    spawn(move || {
        let values = vec![
            String::from("GREETINGS"),
            String::from("FROM"),
            String::from("FIRST"),
            String::from("THREAD"),
        ];
        for value in values {
            tx.send(value).unwrap_or_default();
            sleep(Duration::from_secs(3));
        }
    });

    spawn(move || {
        let values = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("second"),
            String::from("thread"),
        ];
        for value in values {
            tm.send(value).unwrap_or_default();
            sleep(Duration::from_secs(3));
        }
    });

    for received in rv {
        println!("Got: {}", received);
    }
}

fn main() {
    let (tm, rv) = channel();

    spawn(move || {
        let mssg = String::from("Hello there");
        tm.send(mssg).unwrap_or_default();
    });

    let received = rv.recv().unwrap_or_default();
    println!("Got: {} \n", received);

    send_multiple();

    // multiple producers single consumer
    mpsr();
}
