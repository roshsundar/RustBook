/* This is a test module to house some integration tests.
Use ٭cargo test٭ to run.
*/

use organize_tests::add;

mod common;

#[test]
fn it_adds() {
    common::setup();

    let result = add(1, 2);
    assert_eq!(result, 3);
}

// No test module is necessary since Rust treats the ٭tests٭ directory specially
// and compiles this code when ٭cargo test٭ is run.
// To run just the tests in this file, use ٭cargo test --test integration_test٭

/*
Let's say you want to have some common functionality to use in multiple test files.

Group the code in a submodule by creating a sub-directory for it.

organize_tests
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
*/

/*
$ If the project is a binary crate that only has a src/main.rs file and doesn't also have
$ a src/lib.rs file, functions inside src/ can't be exposed to the tests/ dir.

AKA, if you have a binary crate, make sure you have a src/lib.rs to enable the creation
of integration tests.
*/