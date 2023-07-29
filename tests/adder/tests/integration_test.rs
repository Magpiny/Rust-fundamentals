use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add(2, 2));
}

// To run this test only without the unit tests in the lib
// $ cargo test --test integration_test
