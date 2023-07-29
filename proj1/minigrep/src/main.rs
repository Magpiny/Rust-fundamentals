/*
*An IO project:
* Building a commandline program
*/
use std::env;
use std::fs;

fn main() {
    test12();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[2];
    // let query = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file!");

    println!("With text: \n {contents}");
}

#[allow(unused)]
fn test12() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let program_path = &args[0];
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {} in {}", query, file_path);
    println!("PROGRAM PATH: {} \n", program_path);
}
