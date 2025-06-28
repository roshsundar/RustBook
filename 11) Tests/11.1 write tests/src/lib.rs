/* Summary:
Tests are special Rust functions that verify that other code produces expected results.

Bodies of test functions typically:
    • Set up needed data or state
    • Run the code to evaluate
    • Assert that the results are expected

The ٭ test ٭ attribute declares a function as a test function to be run with ٭ cargo test ٭

A test fails if it panics or returns an Err variant (of Result<T, E>), and passes otherwise.
*/

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

//? ------------------------------------------------------------------------------------------------------------------------

#[cfg(test)] //$ Configure the module as a test module that will be run with ٭ cargo test ٭
mod add_tests {
    use super::*; // add_tests in an inner module, so code outside the module must be brought in scope

    #[test] // Designate a function as a test function
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // checks that result and 4 are equal, will panic and fail otherwise
    }

    #[test]
    fn will_fail() {
        //$ A test fails when something in the function panics
        panic!("Make this test fail")
    }
}

//? ------------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)] 
mod rectangle_tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        //$ The assert! macro tests T/F. If it evals to T, then nothing happens. If it evals to F, then it panics and the test fails.

        assert!(larger.can_hold(&smaller)); // Will eval to T, and the test passes
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger)) // smaller can't hold larger -> F. The ! makes it T. Therefore the assert passes
    }
}

//? ------------------------------------------------------------------------------------------------------------------------

pub fn multiply(a: usize, b:usize) -> usize {
    a * b
}

#[cfg(test)]
mod multiply_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiply(2, 6);
        assert_eq!(result, 12); //$ assert_eq! fails if the values are !=
    }

    #[test]
    fn it_not_not_works() {
        let result = multiply(2, 6);
        assert_ne!(result, 0); //$ assert_ne! fails if the values are ==
    }
}

//? ------------------------------------------------------------------------------------------------------------------------

pub fn greeting_buggy(name: &str) -> String {
    // format!("Hello {name}!") is the proper implementation
    format!("Hello person!")
}

#[cfg(test)]
mod greeting_tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting_buggy("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting didn't contain the name: {result}" //$ This will output since the assert fails, also works for the other assert macros
        )
    }
}

//? ------------------------------------------------------------------------------------------------------------------------

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess value must be >= 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be <= 100, got {value}.");
        }

        Self { value }
    }
}

#[cfg(test)]
mod guess_tests {
    use super::*;

    #[test]
    #[should_panic] //$ should_panic makes the test pass if code inside the test panics. The test fails if it doesn't panic.
    fn greater_than_100() {
        // Check the Guess value > 100 error condition by creating a new Guess
        Guess::new(200);
    }

    #[test]
    //$ The expected param will check that the panic message includes the specified substring, "the disco" in this case.
    // The test will fail if the code inside panics without the expected substring in the panic message
    #[should_panic(expected = "the disco")]
    fn will_fail() {
        // This test will fail since the code panics and the message doesn't contain "the disco"
        panic!("Panic at the venue!");
    }
}

//? ------------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod add_tests_2 {
    use super::*;

    #[test]
    //$ Tests can also be written to return a Result<T, E>.
    // If the Result is Ok, then the test will pass and won't print anything.
    // If the Result is Err, then it will fail and print E
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(()) // The Result will be Ok so test will pass and nothing prints
        } else {
            Err(String::from("2 + 2 != 4 😉"))
        }
    }

    #[test]
    fn will_fail() -> Result<(), i32> {
        // The test will fail and print -1
        Err(-1)
    }

    //$ #[should_panic] can't be used on tests that return Result<T, E>

    //$ To assert that a Result<T, E> is an Err variant, use assert!(result.is_err())
}