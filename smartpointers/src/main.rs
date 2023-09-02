// -> SMART POINTERS
/*
* A pointer is a general concept for a variable that contains an address in memory.
* This address refers to, or “points at,” some other data. The most common kind of pointer
* in Rust is a reference, which you learned about in Chapter 4. References are indicated by
* the & symbol and borrow the value they point to. They don’t have any special capabilities
* other than referring to data, and have no overhead.
*
* Smart pointers, on the other hand, are data structures that act like a pointer but also have
* additional metadata and capabilities
*
* Rust, with its concept of ownership and borrowing, has an additional difference between
* references and smart pointers: while references only borrow data, in many cases, smart
* pointers own the data they point to..
*
* Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart
* pointers implement the Deref and Drop traits. The Deref trait allows an instance of the
* smart pointer struct to behave like a reference so you can write your code to work with
* either references or smart pointers. The Drop trait allows you to customize the code that’s
* run when an instance of the smart pointer goes out of scope.
*
* THE MOST COMMON SMART POINTERS IN THE STL INCLUDE
*
    Box<T> for allocating values on the heap
    Rc<T>, a reference counting type that enables multiple ownership
    Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at
    runtime instead of compile time

*
* FINALLY WE'LL COVER THE FOLLOWING
* interior mutability pattern where an immutable type exposes an API for mutating an interior value.
* We’ll also discuss reference cycles: how they can leak memory and how to prevent them.
*
*/

// Using Box<T> to Point to Data on the Heap

// When to use Box<T>
/*
* When you have a type whose size can’t be known at compile time and you want to use a value of that
* type in a context that requires an exact size
*
* When you have a large amount of data and you want to transfer ownership but ensure the data won’t be
* copied when you do so
*
* When you want to own a value and you care only that it’s a type that implements a particular trait
* rather than being of a specific type
*
*/
use crate::List::{Cons, Nil};
use std::ops::Deref;
// Using a Box<T> to Store Data on the heap
fn box2heap() {
    let b = Box::new(21);
    println!("b is {}", b);
}

// Enabling Recursive Types with Boxes
// A value of recursive type can have another value of the same type as part of itself.

// recursive type
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn recursive_type() -> List {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list
}

// Defining our own smart pointer
//
// copy & clone traits
// #[derive(Debug, Copy, Clone)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement copy and clone traits
impl<T: Copy> Copy for MyBox<T> {}
impl<T: Copy> Clone for MyBox<T> {
    fn clone(&self) -> Self {
        *self
    }
}
// Treating a Type Like a Reference by Implementing the Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// Implicit Deref Coercions with Functions and Methods
fn hello(country: &str) {
    println!("Good evening {}", country);
}

// Implicit Deref Coercions with Functions and Methods
// Deref coercion converts a reference to a type that implements the Deref trait into a reference
// to another type. For example, deref coercion can convert &String to &str because String implements
// the Deref trait such that it returns &struct
fn own_smartpointer() {
    let m = 8;
    let n = MyBox::new(m);
    println!("n = {}", *n);

    let x = MyBox::new(23);
    let y = x.clone();
    println!("x clone is {}", *y);

    // Implicit Deref Coercions with Functions and Methods
    let cname = MyBox::new(String::from("Kenya"));
    hello(&cname);

    assert_eq!(8, *n);
}

// Running Code on Cleanup with the Drop Trait
// which lets you customize what happens when a value is about to go out of scope.
// You can provide an implementation for the Drop trait on any type, and that code
// can be used to release resources like files or network connections.
struct CustomSmartPointer {
    data: String,
}
impl Deref for CustomSmartPointer {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{} went out of scope and has been dropped", self.data);
    }
}

fn check_scope() {
    let p = CustomSmartPointer {
        data: String::from("First data"),
    };
    let q = CustomSmartPointer {
        data: String::from("Second data"),
    };
    println!("CustomSmartPointer created: {} & {}", *p, *q);
}

// Dropping a Value Early with std::mem::dropped
//
fn explicit_drop() {
    let h = CustomSmartPointer {
        data: String::from("Third data"),
    };
    println!("CustomSmartPointer created: {}", *h);
    drop(h);
    println!("CustomSmartPointer dropped B4 the end of main()");
}

// Main function
fn main() {
    box2heap();
    recursive_type();

    own_smartpointer();

    let x = 3;
    let y = &x;
    println!("y is {}", y);

    // Usng Box<T> as a reference
    let a = 5;
    let c = Box::new(a);
    println!("c is {}", c);

    assert_eq!(3, *y);
    assert_eq!(*c, 5);

    check_scope();
    explicit_drop();
}
