/*
* FUNCTIONS:
*Rust code uses snake case as the conventional style for function and variable names, 
in which all letters are lowercase and underscores separate words. 

We define a function in Rust by entering fn followed by a function name and a set of 
parentheses. The curly brackets tell the compiler where the function body begins and ends.

We can call any function we’ve defined by entering its name followed by a set of parentheses. 

Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope 
that can be seen by the caller.

RUST function parameters MUST be type anotated!

*/
fn main() {
    println!("Hello from main fuction");

    newfunc();

    greetings("Jimmy");

    add_two_numbers(120, 102);

    decimal_to_other_number_systems_conversion(122_234, 8); //b or o or x lowercase only allowed 

    let square = getsquare(7);

    let mssg = hello("Sam");

    println!("Square is: {square}");

    println!("Message: {mssg}");

}

fn newfunc(){
   println!("Hello from another function outside the main function") 
}

fn greetings(name: &str){

    println!("Good morning {name}, Rust is awesome")

}

fn add_two_numbers(num1: i16, num2: i16){
    let sum: i16 = num1 + num2;

    println!("The sum of {num1} and {num2} is: {sum}");
}

fn decimal_to_other_number_systems_conversion(number: i64, base: u8){

    if base == 2{  
        println!("{number} to base {base} is {:b}", number);    
    }else if base == 8{ 
        println!("{number} to base {base} is {:o}", number);    
    }else if base == 16{  
        println!("{number} to base {base} is {:x}", number);    
    }
    else{
        println!("Base {base} is NOT recognized!!");
    }
}

/* 
* It's important to know the difference between statements and expressions in RUST
* STATEMENTS: do not return a value e.g function defination and variable declaration and
* asignments
* EXPRESSIONS: return a value e.g function calls, calling macros, statements wrapped in curly
* braces
*Expressions do not include ending semicolons. If you add a semicolon to the end of 
an expression, you turn it into a statement, and it will then not return a value. 
*
*/
fn increment_y() {
    let y = {
        let x = 3;
        x + 1
    }; // this is an expression

    println!("The value of y is: {y}");
}

// Functions with return values
fn hello(fname: &str) -> String{
    format!("Hello {fname}, Rust is awesome!")
}

fn getsquare(number: i64) -> i64 {
    number*number
}

