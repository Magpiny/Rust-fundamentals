/*
A struct, or structure, is a custom data type that lets you package
together and name multiple related values that make up a meaningful group.

*/
struct User {
    username: String,
    email: String,
    password: i32,
    status: bool,
}
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    // Defining a structure
    struct User {
        username: String,
        email: String,
        password: i32,
        status: bool,
    }

    //Instantiate a struct
    //Use the new struct
    let user1 = User {
        username: String::from("Jonte"),
        email: String::from("jonte12@ymail.com"),
        password: 123456,
        status: true,
    };

    //Accessing properties of an object
    let email = user1.email;
    println!("Email: {email}");

    build_user(
        String::from("jkl@ymail.com"),
        String::from("Jane"),
        122345,
        false,
    );

    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("Area of rectangle is: {}", rectangle_area(&rect1));
}

fn build_user(email: String, username: String, password: i32, status: bool) -> User {
    User {
        username,
        email,
        password,
        status,
    }
}

fn rectangle_area(rectangle: Rectangle) -> i32 {
    return rectangle.width * rectangle.height;
}
