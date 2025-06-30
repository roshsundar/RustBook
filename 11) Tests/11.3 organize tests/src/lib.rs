/* Summary:
2 categories of tests
    • Unit: small and focused, evaluates 1 isolated module at a time, and can test private interfaces.
    • Integration: external to the library and uses the public interface, can evaluate several modules.

Unit tests exist in the source dir, typically in the file of the tested code.
Integration tests exist in files in a special tests/ dir you have to create.
*/

/*
Unit tests test each unit of code in isolation to quickly find where code isn't working.
They will be in the src dir in each file w/ the code they're testing.
The convention is to create a module called ٭tests٭ in each file for unit tests,
and annotate it w/ #[cfg(test)]
*/

// Note that even if add() wasn't pub, the it_works() test would still work
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*
Integration tests are external to the library, and can only call public functions.
They test if several parts of the library work together properly.

A tests/ dir must be created to house the integration tests. Multiple test
files can be created in the dir.

organize_tests
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
*/