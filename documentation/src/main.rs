//! # `add_two` gives the result of adding two numbers
//! Adds two i32 numbers and return an i32 sum

pub use self::area::circle;
pub use self::area::rectangle;
pub use self::fig_prop::Circle;
pub use self::fig_prop::Rectangle;

/// The defacto rust entry point
/// ## Basic usage
/// ```
///   use add_two;
///
///   let a = 23;
///   let b = 47;
///   // Adding two numbers
///   let sum = add_two(a, b);
///   println!("Sum is: {}", sum);
/// ```
///
/// ### OUTPUT: Sum is 70
///
/// Documentation by Wanjare S.
fn main() {
    let radius = Circle { radius: 21.0 };
    let rectangle = Rectangle {
        length: 8.0,
        width: 5.0,
    };

    // areas
    println!("area of circle is {}", area::circle(radius));
    println!("area of Rectangle is {}", area::rectangle(rectangle));

    println!("a + b = {}", add_two(23, 37));
}

/// `add_two()` returns the sum of two numbers
fn add_two(a: i32, b: i32) -> i32 {
    return a + b;
}

/// Plane figures properties
pub mod fig_prop {
    pub struct Circle {
        pub radius: f32,
    }

    pub struct Rectangle {
        pub length: f32,
        pub width: f32,
    }
}

/// # A library for calculating the area of plane figures
/// Easily calculate the area of a cirlce and a rectangle
/// Calculate areas of plane figures with ease!
pub mod area {
    pub use crate::fig_prop::*;

    pub fn circle(circ: Circle) -> f32 {
        3.142 * circ.radius * circ.radius
    }

    pub fn rectangle(rec: Rectangle) -> f32 {
        rec.length * rec.width
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_additon() {
        assert_eq!(add_two(6, 4), 10);
    }
}
