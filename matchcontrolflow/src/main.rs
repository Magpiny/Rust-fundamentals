//The match Control Flow Construct
//match allows you to compare a value against a series of patterns
//and then execute code based on which pattern matches. Patterns can be
//made up of literal values, variable names, wildcards, and many other
//things
#![allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum Counties {
    Nairobi,
    Mombasa,
    Kisumu,
}

enum PlateNumber {
    CountyCode,
    County(Counties),
}

// First we list the match keyword followed by an expression, which in
// this case is the value coin.
// Next are the match arms. An arm has two parts: a pattern and some code.
// The first arm here has a pattern that is the value Coin::Penny and then
// the => operator that separates the pattern and the code to run.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Hello, Match!");

    //Pattern matching
    ifletelse();

    match_int(12);
}

// Concise flow with if let
// The if let syntax lets you combine if and let into a less verbose way to handle
// values that match one pattern while ignoring the rest.
//
// The syntax if let takes a pattern and an expression separated by an equal sign.
// It works the same way as a match, where the expression is given to the match and
// the pattern is its first arm. In this case, the pattern is Some(max), and the max
// binds to the value inside the Some. We can then use max in the body of the if let
// block in the same way we used max in the corresponding match arm. The code in the
// if let block isn’t run if the value doesn’t match the pattern.
//

// using match
// A match that only cares about executing code when the value is Some
fn matchfn() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

//Using if let for the above code
fn ifletfn() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

//Using if let else
fn ifletelse() {
    let county = PlateNumber::CountyCode;
    let mut count = 0;
    if let PlateNumber::County(jimbo) = county {
        println!("State quarter from {:?}!", jimbo);
    } else {
        count += 1;
    }
}

//One more integer option match
fn match_int(i: i8) -> Option<i8> {
    let num: i8 = 23;
    //TODO: This wrong and will be fixed soon
    if let i = num {
        Some(i + 2)
    } else {
        None
    }
}
