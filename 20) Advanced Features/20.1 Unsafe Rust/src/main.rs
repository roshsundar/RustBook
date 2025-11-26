/*Summary:
Unsafe Rust is Rust code that is allowed to violate its conservative safety rules.
You are trusted to manage memory safety.

There are 5 actions in unsafe Rust that you can't do in safe Rust
    1. Dereference a raw pointer
    2. Call an unsafe function or method
    3. Access or modify a mutable static variable
    4. Implement an unsafe trait
    5. Access fields of a union
Note that unsafe doesn't turn off the borrow checker or other Rust safety checks.
*/

fn main() {
    // Dereferencing a raw pointer
    {
        /*
        Raw pointers:
            • Are allowed to ignore the borrowing rules by having both immutable and mutable pointers, or multiple mutable pointers, to the same location.
            • Aren't guaranteed to point to valid memory.
            • Are allowed to be null.
            • Don't have any automatic cleanup.

        Rust has an immultable and mutable raw pointer, `*const T` and `*mut T` respectively. 
        */

        let mut num = 5;
        let r1 = &raw const num;
        let r2 = &raw mut num;

        unsafe { // Deref raw pointers in an unsafe block
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        }

        // You can create a raw pointer to an arbitrary memory address.
        // There is no reason to do it, there may or may not be valid data there.
        let address = 0x012345usize;
        let _r = address as *const i32;
    }

    println!();

    // Calling an unsafe function or method
    {
        unsafe fn dangerous() {
            println!("unsafe stuff")
        }

        unsafe {
            dangerous();
        }

        println!();

        // You can also wrap unsafe code inside a safe function.
        fn mostly_safe() {
            println!("some safe stuff");

            unsafe {
                println!("some unsafe stuff");
            }
        }

        mostly_safe();
    }

    // Using `extern` functions to call external code
    {
        // Interface for calling C's abs() function.
        unsafe extern "C" { // Since C doesn't follow Rust's safety features, need to wrap it in unsafe.
            fn abs(input: i32) -> i32;
        }

        unsafe {
            assert_eq!(3, abs(-3)); // Call C's abs() function
        }
    }

    println!();

    // Accessing or modifying a mutable static variable
    {
        // In Rust, global variables are static variables.
        static HELLO_WORLD: &str = "Hello, world!";
        println!("name is: {HELLO_WORLD}");

        // Static variables can be mutable, but modification is unsafe.

        static mut COUNTER: u32 = 0;

        unsafe fn add_to_count(inc: u32) { // SAFETY: Not thread safe. If threading, only call from one thread at a time.
            unsafe {
                COUNTER += inc;
            }
        }

        unsafe {
            add_to_count(3);
            assert_eq!(3, *(&raw const COUNTER)) // Can't reference a mutable static value directly, can only access w/ raw pointer.
        }
    }

    // Impl'ing an unsafe trait
    {
        // A trait must be unsafe when at least one method has an invariant the compiler can't verify

        unsafe trait Foo {
            // methods go here
        }

        unsafe impl Foo for i32 {
            // method implementations go here
        }
    }

    // Accessing fields of a `union`
    {
        // union is similar to struct, but only one field is used in a particular instance at a time.
        // Accessing a union field is unsafe.
    }

    // Using Miri to check unsafe code
    {
        // Miri is a Rust tool for detecting undefined behavior.
        // It works at runtime, and looks for rule violation.

        // Install it with `rustup +nightly component add miri`

        // Run it with `cargo +nightly miri run` or `cargo +nightly miri test`
    }
}
