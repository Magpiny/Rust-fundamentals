/*
*Method Syntax

Methods are similar to functions:
we declare them with the fn keyword and a name,
they can have parameters and a return value, and they contain some code that’s run
when the method is called from somewhere else. Unlike functions, methods are defined
within the context of a struct (or an enum or a trait object and their first parameter
is always self, which represents the instance of the struct the method is being called on.
*
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        let sum = self.width + self.height;
        sum * 2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };

    println!(
        "Area is: {} and perimeter is: {}",
        rect1.area(),
        rect1.perimeter()
    );

    // Calling an associated function
    let sqr = Rectangle::square(4);
    println!("Square is: {:#?}", sqr);
}

/*
*Associated Functions

All functions defined within an impl block are called associated functions because
they’re associated with the type named after the impl. We can define associated
functions that don’t have self as their first parameter (and thus are not methods)
because they don’t need an instance of the type to work with. We’ve already used
one function like this: the String::from function that’s defined on the String type.
*
*/
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// -------------------------------> THE END <-------------------------------
