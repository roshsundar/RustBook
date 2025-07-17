/* Summary: 
This is code for minigrep - a CLI tool to search a file (in cwd) for a string and print the occurences.

Usage is the following for a case sensitive search
$ cargo run -- query file
i.e.
$ cargo run -- to poem.txt

For a case insensitive search use
$ IGNORE_CASE=1 cargo run -- to poem.txt
*/

/*
Organizing binary projects is important. The Rust community has standard guidelines.

If main() gets too large
    • Split the program into main.rs and lib.rs
        • Move logic into lib.rs
    • As long as CLI args parsing logic is simple, keep it in main.rs
        • If it gets complex, move it to lib.rs

What remains in main() after this is
    • Calling CLI parsing, passing in args
    • Setting up any other configuration
    • Calling a run() func in lib.rs
        • Handle errors from run()
*/

use std::{env, process};

use minigrep::Config;

fn main() {
    /* Get the CLI args */
    let args: Vec<String> = env::args().collect();

    /* Attempt to parse the args into a Config struct, print an Error and quit if it fails */
    //  For an Ok, unwrap_or_else() extracts the Ok value.
    //  For an Err, it passes the Err value into the closure function that prints the error string.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}"); // eprintln!() prints to stderr instead of stdout
        process::exit(1); // ! exit program in error state
    });

    /* Run the main logic to search for the word in the file. Print an Error and if anything goes wrong. */
    //  If there is an error from the running of the program, then print it and quit.
    //      • The reason not to use unwrap_or_else() here is because it would unwrap
    //        the () unit value, which we don't care about. We only want the Err.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}