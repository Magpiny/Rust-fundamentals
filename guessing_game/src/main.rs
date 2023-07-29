// Number guessing game
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secrete_number = rand::thread_rng().gen_range(1..=10);

    println!("The secrete number is: {secrete_number}");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secrete_number){

            Ordering::Less => println!("Number Too Small"),
            Ordering::Equal => {
                println!("CONGRATULATIONS!! You win!");
                break;
            },
            Ordering::Greater => println!("Number bigger than expected number"),
        }
    }
}

