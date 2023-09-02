fn main() {
    println!("Hello, world!");

    pointers();

    unsafe {
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    type_alias();
}

// unsafe rust
// RAW POINTERS -> *const T and *mut T
//
// creating raw pointers from references

fn pointers() {
    let mut num = 9;

    let p1 = &num as *const i32;
    let p2 = &mut num as *mut i32;

    println!("p1 is {:?}", p1); // -> p1 is 0x7ffedcbebd44

    unsafe {
        println!("p2 is {}", *p2);
    }
}

// Unsafe functions
unsafe fn dangerous() {}

// Using extern functions to call external code;
// Foreign Function Interface (FFI)
// An FFI is a way for a programming language to define functions and enable a
// different (foreign) programming language to call those functions.
extern "C" {
    fn abs(input: i32) -> i32;
}

// Creating type aliases
fn type_alias() {
    type Int = i32;

    let distance: i32 = 100;
    let length: Int = 23;

    println!("Length: {distance} + {length} = {}", distance + length);
}

// Advanced traits
// SPecifying placeholder types in trait definitions
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

#[allow(unused)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// THe never type that never returns !
fn bar() -> ! {
    panic!();
}
