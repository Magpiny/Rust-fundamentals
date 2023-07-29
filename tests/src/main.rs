/*
* :WRITTING AUTOMATED TESTS
* Tests are Rust functions that verify that the non-test code is
* functioning in the expected manner.
*
TYPES OF TESTS
*Unit Tests
Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
You’ll put unit tests in the src directory in each file with the code that
they’re testing. The convention is to create a module named tests in each
file to contain the test functions and to annotate the module with cfg(test)

*Integration Tests
Integration tests are entirely external to your library and use your code
in the same way any other external code would, using only the public
interface and potentially exercising multiple modules per test.

Their purpose is to test whether many parts of your library work together
correctly. Units of code that work correctly on their own could have
problems when integrated, so test coverage of the integrated code is
important as well. To create integration tests, you first need a tests
directory.
---------THE TESTS DIRECTORY----------------
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

*
The bodies of test functions typically perform these three actions:
* *Set up any needed data or state.
* *Run the code you want to test.
* *Assert that the results are what you expect.
*
*At its simplest, a test in Rust is a function that’s annotated with
the test attribute.
*Attributes are metadata about pieces of Rust code;
* e.g #[test], #[allow(unused)] , #[derive(Debug)]
*
* Runt tests with ->$ cargo test
*/

fn main() {
    println!("----'WRITTING AUTOMATED TESTS'-----");
    addthree(12, 23, 34);
}

fn addthree(first: i32, sec: i32, third: i32) -> i32 {
    let sum = first + sec + third;
    println!("{} + {} + {} = {sum}", first, sec, third);
    sum
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add3() {
        let result = addthree(2, 3, 4);
        assert_eq!(result, 9);
    }
}

// CONTROLLING HOW TESTS ARE RUN
// By controlling which tests run, you can make sure your cargo test
// results will be fast.

//RUN ALL TESTS
//cargo test -- --include-ignored
//
//RUN ALL TESTS leaving out ignored tests
//cargo test
//
//RUN ONLY IGNORED TESTS
//cargo test -- --ignored
//
//RUNNING A SPECIFIC TEST
//cargo test testname
//
//SHOWING A FUNCTION OUTPUT
//cargo test -- --show-output
//
//RUNNING TEST CONSECUTIVELY
//cargo test -- --test-threads=1
//
//
//CHECK adder folder FOR MORE TESTS AND CONTEXTS
//
//THE END!!
