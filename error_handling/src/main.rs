/*
*   ERROR HANDLING IN RUST
*Rust groups errors into two major categories: recoverable and
    unrecoverable errors.
*Rust doesn’t have exceptions.
*Instead, it has the type Result<T, E> for recoverable errors AND
*the panic! macro that stops execution when the program encounters an unrecoverable error
*
*
*/
use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("-----------ERROR HANDLING IN RUST! ------------");

    //panic_backtrace(); // Error in the 1st call will stop the program from executing
    //panick_button(); // so, this code will not be executed
    // filehn();
    // expect_unwrap();

    read_username_from_file();
}
//Explicitly causing our program to crush
fn panick_button() {
    panic!("Crush and burn");
}

// panic being called in our program from another func somewhere
fn panic_backtrace() {
    let v = vec![1, 23, 34];
    let last = v[5]; // 40 does not exist!
    println!("last is {last}");
}

// Recoverable errors with result
// Result here is a generic type

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// case study: Opening a file returns a Result type in Rust
// Because the file could be there (Ok) or not (Err)
fn filehn() {
    // The return type of File::open is a Result<T, E>
    let getfile = File::open("hello.txt");

    let hellofile = match getfile {
        Ok(file) => file,
        Err(error) => panic!("File NOT Found {:?}", error),
    };
}

// Shortcuts for panic on Error: unwrap() and expect()

fn expect_unwrap() {
    // let getfile = File::open("hello.txt").unwrap();
    let getfile2 = File::open("hello.txt").expect("Error opening file!");
}

// Propagating Errors
// A function that returns errors to the calling code using match

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
