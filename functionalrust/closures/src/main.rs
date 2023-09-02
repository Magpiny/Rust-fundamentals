// Closure type inference and annotation
//
use std::{thread, time::Duration};

#[allow(unused)]
fn main() {
    // Closures with parameters and return types
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let hello = |name: &str| -> String {
        //println!("Good morning {name}");
        format!("Good morning {}", name)
    };

    // A closure with more than one parameter
    let add_two = |a: u32, b: u32| -> u32 { a + b };

    let add_two_v1 = |a: u16, b: u16| a + b;

    // A closure without a parameter and return type!!
    let newprog = || println!("Functional rust is fun!!!");

    println!("{}", hello("John"));
    println!("{:?}", newprog());
    println!("Sum is: {}", add_two(21, 79));

    sort_rectangles();
}
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort_rectangles() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.height);
    //sort_by_key is defined to take an FnMut closure in that it calls the closure multiple
    //times: once for each item in the slice. The closure |r| r.width doesn’t capture, mutate,
    //or move out anything from its environment, so it meets the trait bound requirements.
    println!("{:#?}", list);
}
