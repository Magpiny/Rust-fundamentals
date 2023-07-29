//enumerations, also referred to as enums. Enums allow you to define a
//type by enumerating its possible variants
//enum example
#![allow(unused)] // stops the compiler & text editor from screeming at me on unused variables,
                  // funtions etc etc

enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Adding a method to an enum just like in struct
impl Message {
    fn call(&self) {}
}

fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    route(ipv4);
    route(ipv6);

    let m = Message::Write(String::from("Hello"));
    m.call();

    println!("localhost: {:#?}", home);
}

fn route(ipadr: IpAddrKind) {}

// Option
// std::option::Option
// The Option type encodes the very common scenario in which a value could 
// be something or it could be nothing.
// Was introduced to solve the 'null' type mess;
pub enum Option<T> {
    None,
    Some(T),
}

let x: Option<i32> = Some(203);

let y: Option<i32> = None;

let text: Option<String> = Some("Hello, world!".to_string());



