fn main() {
    let age: i8 = -21;

    if age >= 18 {
        println!("You're allowed to take beer!!");
    }else if age>0 && age < 18 {
        println!("Go back and Bookworm for your future");
    }
    else{
        println!("Enter a positive integer like {}", iffunc());
    }

    loopit();
    // Rust uses 14:41:63 minutes to count to 1million: Speed test! (build & run)
    whileloop(10);

    forloop();
}

fn iffunc() -> i32{
    // Using if in a let statement
    let condition = true;
    let number = if condition {12} else {23};

    number
}

// REPETITION/LOOPING
// Rust has three kinds of loops: loop, while, and for. Let’s try each one.

// loop keyword
// The loop keyword tells Rust to execute a block of code over and over 
// again forever or until you explicitly tell it to stop.

fn loopit(){
    let mut counter = 0;

    loop{
        counter+=2;
        println!("Coding is freeking fun!");

        if counter == 10 {
            break;
        }
    }
}

// while loop
fn whileloop(mut number: i32){
    let fruits = ["Mango", "banana", "oranges", "guava", "lemon"];
    if number > 0 {
        println!("Since the number is greater than One, i'll count downwards to zero!");

        while number >=0 {
            println!("count {number}");
            number -= 1;
        } 
    }else {
        println!("SInce the number is less than One I'll count upwwards to zero!");
        while number <=0{
            println!("Count: {number}");
            number += 1;
        } 
    }

    // Using while loop to access elements in an array
    let mut index = 0;
    while index < fruits.len(){
        println!("I like {}",fruits[index]);
        index += 1;
    }
}

// Using for loop;
//
fn forloop(){
let fruits = ["Mango", "banana", "oranges", "guava", "lemon"];

for fruit in fruits {
        println!("{fruit} fruit is sweet!");
    }
}

