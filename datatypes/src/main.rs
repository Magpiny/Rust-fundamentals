// RUST: Data Types
//
/*
*   SCALAR TYPES
*       INT: Number without a fractional component (Whole numbers) 
*           i8/u8 to i128/u128
*           i32(default)
*
*       FLOATING POINT: (decimal numbers)
*           we have f32(single precision float) and f64(default)(double precision float)
*           All floats are signed!
*
*       BOOL
*       NUMBERS
*       CHARACTERS
*
*   VECTOR TYPES
*
* */

fn main() {
    let x = 4.0;    //f64

    let y: f32 = 4.5; // f32

    // Numeric Operations
    // addition
    let sum = x + y;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // chars
    let symbol: char = 'N'; //type annotated
    let sym = 'S';          // without type annotation
   
    // Strings
    let hello: String = "Hello there, I am hardcore Rustacean!";
    let fname: &str = "Samuel";

    //Boolean values
    let loggedin: bool = true;
    let isuser = false;

    
    println!("x:f64 -> {x}");
    println!("x:f32 -> {y}");
    println!("sum: {sum}");
    println!("difference: {difference}");
    println!("product: {product}");
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");
    println!("remainder: {remainder}");
    println!("{hello}".replace("there", fname));

    // Compund types
    // Under these we have: tuples and arrays

    
    // A: TUPLES
    // A tuple is a collection of values of different types. Tuples are constructed using 
    // parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), 
    // where T1, T2 are the types of its members.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
     
    // declaring tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // annotated tuple
    let tup1 = ("Sam", true, 60);             // The type will be inferred
    //
    // Accessing items in a tuple
    // 1: Using DESTRUCTURING
    let (x, y, z) = tup1;

    // 2: Using .(dot) notation. _-> tuplename.itemIndex
    let k = tup.0;

    println!("The first element in tup is: {k}");

    println!("My name is: {x}");

    //B: ARRAYS
    //An array is a collection of objects of the same type T, stored in contiguous memory. 
    //Arrays are created using brackets [], and their length, which is known at compile time, 
    //is part of their type signature [T; length].
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    
    // Declaring arrays:
    let week_days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let squares: [i32; 5] = [1, 4, 9, 16, 25];
    let lucky_number = [4; 5];      // same as let lucky_number = [4,4,4,4,4];

    // Accessing array elements:
    let resting_day = week_days[0];

    println!("We rest on {resting_day}");

}

// THE END!

