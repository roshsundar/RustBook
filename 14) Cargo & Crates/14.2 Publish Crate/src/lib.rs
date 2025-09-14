/*
To add documentation for a crate or module use //!
*/

//! # My Crate
//!
//! `my_crate` is a collection of stuff to demonstrate documentation.

/*
Rust has a special documentation coment, ///, which is meant for other programmers to
know how to use your functions.
Crate authors usually make sections covering
    • when the function panics
    • what errors it returns and when
    • if it is unsafe, why

It can generate html documentation and supports Markdown.
Run the following to generate it. The generated html goes in target/doc directory.
$ cargo doc
To open it as well add --open
$ cargo doc --open

cargo test will run the code in the documentatation itself as a test.
This is to make sure that your examples are up-to-date and work as expected.
$ cargo test
*/

// Make sure the utils module is also included in the documentation
pub mod utils;

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}