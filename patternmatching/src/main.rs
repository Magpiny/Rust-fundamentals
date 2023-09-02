// Pattern matching in RUST
// A pattern consists of the following
//
// Literals
// Destructured arrays, enums, structs, or tuples
// Variables
// Wildcards
// Placeholders

// Number is even
fn iseven() {
    let number = Some(9);

    match number {
        // Using match guards
        Some(x) if x % 2 == 0 => println!("Number {x} is even"),
        Some(x) => println!("Number {} is odd", x),
        None => (),
    }
}

#[allow(unused)]
fn main() {
    let (a, b, c) = (12, 24, 36);
    get_grade(76);
    if let Some(x) = Some(16) {
        println!("X is {}", x);
    };

    // Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 3, y: 4 };
    let Point { x: a, y: b } = p;
    assert_eq!(a, 3);

    // v2: struct Destructuring
    let Point { x, y } = p;
    assert_eq!(4, y);

    whilelet();

    newmatch();

    somet();
    iseven();
}

fn newmatch() {
    let x = Some(5);
    let y = 50;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

// testing match and Some
fn somet() {
    let nums = (1, 2, 3, 4);
    match nums {
        (first, _, third, _) => println!("First and third: {} & {}", first, third),
    }
    if let Some(x) = Some(45) {
        println!("X is {}", x);
    }
}

// using range in a pattern
fn get_grade(grade: i32) {
    match grade {
        81..=100 => println!("Grade is A"),
        74..=80 => println!("Grade is A-"),
        67..=73 => println!("Grade is B+"),
        60..=66 => println!("Grade is B"),
        55..=59 => println!("Grade is B-"),
        48..=54 => println!("Grade is C+"),
        41..=47 => println!("Grade is C"),
        35..=40 => println!("Grade is C-"),
        30..=34 => println!("Grade is D+"),
        25..=29 => println!("Grade is D"),
        20..=24 => println!("Grade is D-"),
        1..=19 => println!("Grade is E"),
        _ => println!("NUMBER OUTSIDE RANGE: Enter a valid number between 1 and 100!!"),
    }
}

fn whilelet() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Removed: {}", top);
    }
}
