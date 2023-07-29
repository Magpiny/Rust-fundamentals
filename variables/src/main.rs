/*
*   VARIABLES:
*   Rust variables are immutable by default but can be made to be mutable
*   using the 'mut' keyword
*   Immutable = cannot reassing a value to a variable once declared



*/


fn main() {
    //immutable variable -> REASSIGNING A VALUE NOT ALLOWED
    let x = 56;

    let y: i64 = -23490; 

    //mutable variable -> Reassigning a value to this variable is alllowed
    let mut zizu  = 45;

    // CONSTANTS
    // Constants are ALWAYS immutable -> mut cannot be used on them
    // Type value MUST always be annotated
    // 'const' keyword is used instead of let in declaration
    // Use all Upper Case and Lower Case in naming CONSTANTS -> RUST Convention
    const HOURS_IN_A_DAY: i32 = 24;

    // shadowing
    let x = x + 2;
    println!("{}",x);


    println!("There are {}hrs in a day", { HOURS_IN_A_DAY });
}
