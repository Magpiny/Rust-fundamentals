/*
*Slice:

Slices let you reference a contiguous sequence of elements in a collection rather than the whole
collection. A slice is a kind of reference, so it does not have ownership.
*
*
*/

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let greet = &s[..];

    println!("{hello} {world}");
    println!("{greet}");
}
