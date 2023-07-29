/*
*GENERICS:
*
*/

#![allow(unused)]
// In struct definition
struct Point<T> {
    x: T,
    y: T,
}

//To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters
struct Point3D<T, U, V> {
    x: T,
    y: U,
    z: V,
}

// In enum definition
enum Option<T> {
    Some(T),
    None,
}

// Enum with multiple types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let integer_point = Point { x: 2, y: 12 };
    let floating_point = Point { x: 4.5, y: 8.4 };

    let three_d_point = Point3D {
        x: 1.2,
        y: 1,
        z: "zero",
    };

    let numbers = vec![32, 45, 67, 78, 12];
    let chars = vec!['a', 'm', 'z', 'q'];

    let biggest_number = largest_number(&numbers);
    let largest_letter = largest_char(&chars);

    println!("Largest number is {}", biggest_number);
    println!("Largest letter is {}", largest_letter);

    let gen_largest_num = largest(&numbers);
    println!("GENERICS: Largest number: {}", gen_largest_num);
}

// largest number
fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

// largest char
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Combining largest number and largest char using generics
// NOTE: funcs 1 & 2 have the same body logic
// Using generics to combine the two functions above
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
