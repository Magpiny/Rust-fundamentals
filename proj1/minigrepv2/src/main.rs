//version 2
//of commandline app
//
//SEPARATION OF CONCERNS:: Separating logic from main
//Our logic code will rest in lib.rs, for easy testing.

use minigrep::Config;
use std::{env, process};

// --snip--
//The standard library provides the eprintln! macro that prints to the standard
//error stream,
fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "<-----Searching for the word '{}' in file {}--------> \n",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error!: {e}");
        process::exit(1)
    }
}
