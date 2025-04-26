/* Summary:
The panic! macro allows for unrecoverable errors that stop the execution of the program.
*/

fn main() {
    // Will crash the program and print the message, filename & line #, and the backtrace
    panic!("crash and burn");

    // Accessing element beyond vector length will panic at runtime
    let v = vec![1, 2, 3];
    v[99];

    // A backtrace is a list of all the functions that have benn called to get to this point.
    // The key is to start from the top and go down until you find files/functions you wrote.
    // The problem likely originates from there.

    /* The backtrace for the v[99] panic! above is here:
    stack backtrace:
    0: rust_begin_unwind
                at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/std/src/panicking.rs:645:5
    1: core::panicking::panic_fmt
                at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:72:14
    2: core::panicking::panic_bounds_check
                at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:208:5
    3: <usize as core::slice::index::SliceIndex<[T]>>::index
                at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:255:10
    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
                at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:18:9
    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
                at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/alloc/src/vec/mod.rs:2770:9
    6: panic::main
                at ./src/main.rs:4:6
    7: core::ops::function::FnOnce::call_once
                at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5

    $ Line 6 of the backtrace points to the line in this code that is causing the problem.
    */
}
