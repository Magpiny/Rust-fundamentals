// Ownership & References:
// Scope, Ownership and References are vital parts of Rustlan
//
fn main() {
    let mut s = String::from("hello"); // mutable string s

    let fname = String::from("John");

    println!("Hello {}", greetings(&fname));

    change(&mut s);
}

fn greetings(name: &String) -> &String {
    return name;
}

// Mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
