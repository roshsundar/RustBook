/* Summary
cargo test runs all tests in the program. This may not always be desired, and there are various ways
to control how tests are run.
*/

/*
cargo test will run tests in parallel (using threads) by default.
$ Because the tests are running at the same time, they can't depend on each other or any shared state or environment
    • i.e. current working directory, environment vairables, etc.

For example, say that several tests write & read from a .txt file. 
B/c the tests run at the same time, one test may overrite the file while another is reading.
This would cause that other test to fail, not because the code is wrong, but because the tests interfered with each other.

One solution is to run the tests one at a time.
    cargo test -- --test-threads=1
--test-threads flag controls how many threads are used to run tests, just 1 in this case to avoid shared state.
*/

/*
$ By default, any print macro in a test won't actually print if it passes, only if it fails.
Running the following example with cargo test will only print output for the failing case.
To show output in the passing case as well, use the flag --show-output.
cargo test -- --show-output

$ To run a single test, say this_test_will_pass(), use cargo test this_test_will_pass

$ Can specify part of a test name, and any test with that in it will run.
i.e. cargo test this_test, will run this_test_will_pass & this_test_will_fail.
*/

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests_10 {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }

    #[test]
    #[ignore] //$ When cargo test is run, this test will be excluded. To run excluded tests, use cargo test -- --ignored
    fn expensive_test() {
        // testing code that takes an hour to run
    }
}
