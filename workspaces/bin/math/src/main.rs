use add_two::add;
use square::sqr;

fn main() {
    // let add = |x: i32, y: i32| -> i32 { x + y };
    let number = 123456789;

    println!("Sum is {}", add(45, 45));
    println!("The square of {number} is {}", sqr(number));
}
