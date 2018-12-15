// Rust has support for Documentation tests, which ensure that code and documentation are in sync.
//
// Arguments to the test binary are specified after the arguments to the `cargo test` command, and
// a '--' sepatartor. Example:
// `cargo test -- --nocapture --test-threads=1` ensures that the test runner does not capture the
// stdout/stderr of tests and that the tests are executed sequentially.
//
// Test filtering:
// `cargo test <pattern>` can be used to run tests matching the provided pattern
// `cargo test -- --ignored` can be used to run tests with the ignore attribute. This is helpful
// when in the normal case we want to skip long-running tests.
//
// Rust allows us to test private functions in addition to public ones.
//
// Following is automatically generated when creating a library crate.
#[cfg(test)]  // This is to specify to the compiler that the module tests should not be included when building this module.
mod tests {

    use super::Rectangle;
    use super::CmpRectangle;  // Rectangle which can be checked for equality.
    use super::add_two;
    use super::greet;
    use super::Guess;

    #[test]  // This indicates to the test runner that this is a test.
    fn assert() {
        // assert! macro takes an expression that evaluates to a boolean. It does nothing if the
        // value is true, and panics if the value is false.
        // assert!(false);  // this would cause this test to panic.
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 12, width: 9};
        let smaller = Rectangle { length: 6, width: 6};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn rotated_large_can_hold_smaller() {
        let larger = Rectangle { length: 5, width: 7 };
        let smaller = Rectangle { length: 6, width: 4 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        // The asssert_eq! macro verifies that the 2 values (Q: What about references?) that are
        // passed to it are equal (Q: Do we have to define what equality means?).
        // assert_ne! is the opposite of the assert_eq! macro.
        assert_eq!(4, add_two(2));
        // The order in which the expected and actual values are passed to the assert macro do not
        // matter.
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn assert_eq_ne() {
        // Values that are passed to assert_eq! or assert_ne! should implement the Debug and
        // PartialEq traits. Debug, so that the values can be printed in case of an error, and
        // PartialEq so that we can compare the 2 values.
        // Following would not compile becuase the Rectangle struct does not implement the
        // PartialEq trait.
        // let larger = Rectangle { length: 12, width: 9};
        // let smaller = Rectangle { length: 6, width: 6};
        // assert_eq!(larger, smaller);
        let larger = CmpRectangle { length: 12, width: 9};
        let smaller = CmpRectangle { length: 6, width: 6};
        assert_ne!(larger, smaller);  // why are values not moved into the assert_ne macro?
        let smaller_dup = CmpRectangle { length: 6, width: 6};
        assert_eq!(smaller_dup, smaller);
    }

    #[test]
    fn custom_message() {
        // All arguments to assert* macros after the required number of arguments are passed to
        // format! and the output of format is printed if the test fails.
        let _result = greet("Abhay");
        // assert!(result.contains("Abhay"), "Expected Abhay, found {}", result);
    }

    #[test]
    #[should_panic]  // This attribute ensures that the test panics due to an expected error.
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]  // This attribute ensures that the test panics due to an expected error.
    // Adding should_panic attribute ensures that the panic that we expect is at the right place in
    // the test, not in an expected place.
    fn greater_than_100_with_expectation() {
        Guess::new_diff(200);
    }

    #[test]
    #[ignore]  // Tests with the attribute 'ignore' set are not run by default. They can be run by passing the --ignored flag to the test runner.
    fn expensive_test() {
        // code that takes an hour to run
    }


}

// Tests can go into any module, and can even be part of a module with non-test code. This allows
// the test to be present alongside the code it is testing.
mod foo_test {
    #[test]
    fn is_a_test() {
    }
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.length > other.length && self.width > other.width) ||
            (self.width > other.length && self.length > other.width)
    }
}

#[derive(PartialEq, Debug)]
struct CmpRectangle {
    length: u32,
    width: u32,
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greet(_name: &str) -> String {
    format!("Hello, Bothra")
}



struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
    pub fn new_diff(value: u32) -> Guess {
        // prints different messages for <1 and >100 values.
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}
