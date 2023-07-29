// Lifetimes are another kind of generic that we’ve already been
// using. Rather than ensuring that a type has the behavior we want,
// lifetimes ensure that references are valid as long as we need
// them to be.

// Lifetime anotation
// &i32;        // a reference
// &'a i32;     // a reference with an explicit lifetime
// let c:&'a mut i32; // a mutable reference with an explicit lifetime
//
// Lifetime anotation in function signature
//

// When annotating lifetimes in functions, the annotations go in the function
// signature, not in the function body.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    println!("a = {}", addtwo(&2, &3));
}

fn addtwo<'n>(a: &'n i32, b: &'n i32) -> &'n i32 {
    let _results = a + b;
    let prod: i32 = {
        let product = a * b;
        product
    };
    println!("a * b = {}", prod);
    a
}

#[allow(unused)]
// Lifetime anotaions in method definition
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
#[allow(unused)]
// Lifetime anotation in struct defintion
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// TODO lifetime elisions
//

// The Static Lifetimes
// denotes that the affected reference can live for the entire duration
// of the program.
// All string literals have the 'static lifetime
const _S: &'static str = "I havei a static lifetime.";

//  THE END!!
