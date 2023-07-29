pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greetings(name: &str) -> String {
    format!("Good evening {}", name)
}

//2:
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn canhold(self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

// Testing that a condition will cause a panic!
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    // A test for can_hold that checks whether a larger rectangle can
    // indeed hold a smaller rectangle
    // Testing with assert
    #[test]
    fn largercanholdsmaller() {
        let larger = Rectangle {
            width: 7,
            height: 5,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(larger.canhold(&smaller));
    }

    //Testing Equality with the assert_eq! and assert_ne! Macros
    #[test]
    fn addtwo() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Adding custom failure messages
    #[test]
    #[ignore]
    fn greetingcontainsname() {
        // this test fails
        let greeting = greetings("Wanjare");
        assert!(
            greeting.contains("Sam"),
            "greeting did not contain name... values was {}",
            greeting
        );
    }

    // Checking for panics with *should_panic*
    // The test passes if the code inside the function panics; the test
    // fails if the code inside the function doesn’t panic.
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> in tests
    // You can’t use the #[should_panic] annotation on tests that use
    // Result<T, E>
    #[test]
    fn it_works2() -> Result<(), String> {
        let result = add(2, 3);
        if result == 5 {
            Ok(())
        } else {
            Err(String::from("2+3 is not equal to 5"))
        }
    }
}
