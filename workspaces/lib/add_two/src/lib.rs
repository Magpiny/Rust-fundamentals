//! # Add function
//! Returns the sum of two numbers
use square::sqr;

pub fn add(left: usize, right: usize) -> usize {
    let sum = left + right;
    println!("And the square of the sum is: {}", sqr(sum));
    sum
}
