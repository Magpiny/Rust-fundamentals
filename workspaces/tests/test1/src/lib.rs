// This folder is all about tests
//! # MY TESTS FOLDER
#![allow(unused)]
use add_two::add;
use square::sqr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_square() {
        assert_eq!(16, sqr(4));
    }
}
