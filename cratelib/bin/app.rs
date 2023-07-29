//import modules and functions from lib
//
//use cratelib::area_math;
//use cratelib::mathlib;
use cratelib::{area_math, financial::simple_accounting, mathlib, pnames};

fn main() {
    println!("Hello world");

    let area = area_math::triangle(12.0, 13.56);
    println!("Area of the triangle is: {area}cm2");

    let rem = mathlib::modulo(23, 23);
    println!("Remainder is {rem}");

    let si = simple_accounting::simple_interest(120000.00, 10.5, 4.5);
    println!("Your Simple Interest is: {}", si);

    let mssg: String = String::from("Hello Wanjare, Rust is fun fun!");
    pnames::print_name(mssg)
}

// -> Learnt code reuse thro' modules
// use of keywords: pub, use and mod
// defining path to binary crate in cargo.toml file
// Made a small math library(library crate) and consumed in the binary crate
