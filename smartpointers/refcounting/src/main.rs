/*
* Reference counting Rc<T>
* We use the Rc<T> type when we want to allocate some data on the heap for multiple parts
* of our program to read and we can’t determine at compile time which part will finish using
* the data last.
*
* Note that Rc<T> is only for use in single-threaded scenarios.
*
*/
use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug, Clone)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn referece_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(4, Rc::clone(&a));
    let c = Cons(3, Rc::clone(&a));
    println!("Rc<T> a & b! {:?} & {:?}", b, c);
}

// # RefCell<T> and the Interior Mutability Pattern
/*
* Interior mutability is a design pattern in Rust that allows you to mutate data even when
* there are immutable references to that data; normally, this action is disallowed by the
* borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to
* bend Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to the
* compiler that we’re checking the rules manually instead of relying on the compiler to
* check them for us;
*
*
*   Enforcing Borrowing Rules at Runtime with RefCell<T>
*   Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios
*/

/*
*   Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
*
*   Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
*
*   Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only
*   immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows
*   checked at runtime.
*
*   Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
*   inside the RefCell<T> even when the RefCell<T> is immutable.

*/
use crate::List2::{Const, Nil as Nill};

#[derive(Debug)]
enum List2 {
    Const(Rc<RefCell<i32>>, Rc<List2>),
    Nil,
}

use std::cell::RefCell;

fn mut_rc() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Const(Rc::clone(&value), Rc::new(Nill)));

    let b = Const(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Const(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?} \n", c);
}

// The main function
fn main() {
    mut_rc();
    referece_count();
    let a = 10;
    let b = &a * 5;
    println!("b is {}", b * 5);
}

// TODO RESEARCH ON THE HALTING PROBLEM!
