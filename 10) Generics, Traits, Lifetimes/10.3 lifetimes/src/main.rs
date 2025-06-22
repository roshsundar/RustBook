/* Summary:
Lifetimes ensure references are valid as long as they need to be.
Every reference has a lifetime. In most cases it is implict.

In some cases, the lifetime of a reference must be explicitly set by annotating.
Annotating the lifetime of a rerference allows rust to track it and make sure it
isn't trying to use data that was already dropped (dangling reference).
*/

use std::fmt::Display;

fn main() {
    // Lifetimes aim to prevent dangling references - when a reference points to something that it was not intended to
    {
        let r;

        {
            let x = 5;
            r = &x; // !❌err: x doesn't live long enough
        } // x is dropped here, but r still references it

        println!("r: {}", r);
    }
    
    // Rust keeps track of lifetimes implicitly
    {
        // 'a denotes the lifetime of r, b' denotes the lifetime of x
        // The lifetime of the referenced data ('b) is shorter than the lifetime of the reference ('a)
        // The compiler throws an error

        let r;                // ---------+-- 'a
                                    //          |
        {                           //          |
            let x = 5;         // -+-- 'b  |
            r = &x;                 //  |       | // !❌err: x doesn't live long enough
        }                           // -+       |
                                    //          |
        println!("r: {r}");         //          |
    }                               // ---------+

    // To fix it, make the lifetime of the data live longer than the reference
    {
        // 'b lives longer than 'a
        // The reference to x will be valid since x is valid

        let x = 5;             // ----------+-- 'b
                                    //           |
        let r = &x;           // --+-- 'a  |
                                    //   |       |
        println!("r: {r}");         //   |       |
                                    // --+       |  
    }                               // ----------+

    // Why lifetimes can be needed in functions
    {
        // Let's say we have a function that compares 2 strings and returns the larger one
        fn longest(x: &str, y: &str) -> &str { // !❌err: the return value does not have a specified lifetime
            if x.len() > y.len() { x } else { y }
        }
        
        // Rust can't tell whhen the reference being returned is x or y. It could be either based on the if-else statement.
        // So the lifetime of the returned &str is not known. The borrow checker can't determine the lifetimes of x and y either.

        // Specifying lifetimes for the references would fix this.
    }

    //$ Lifetime annotation
    {
        // Lifetime annotations do not change how long references live.
        // Rather, they specify the relations of the lifetimes of multiple references.
    
        // The syntax is an ' followed by a lowercase letter. Some examples:
        // &i32         <- A reference
        // &'a i32      <- A reference w/ an explicit lifetime
        // &'a mut i32  <- A mutable reference w/ an explicit lifetime
    }

    // Lifetime annotation in function signatures
    {
        // Generic lifetime params are declared like generic type params

        // The lifetime of the returned reference must = the smaller lifetime between x and y.
        //$ In other words, both x and y must live at least as long as the returned &str.
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        // Specifying the lifetimes does not change the lifetimes of any params.
        // It tells the borrow checker to reject any values that violate these constraints.

        // Here is an example of the generic lifetimes imposing constraints on how longest() can be used
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str()); // !❌err: string2 doesn't live long enough
        } // ! string2 dropped here while borrowed
        println!("The longest string is {result}"); // ! borrow used here

        // From visual inspection we can see result will reference string1 and be valid at the println!
        // but the compiler can't see that. The generic lifetime in longest() enforces that string1 and string2
        // should live at least as long as result, but string2 is dropped early.

        // To fix this, make the string1 and string2 live at least as long as result
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {result}");
        }
        // string1 outlives result ✅, and string2 drops at the same time as result ✅. The constraints of the
        // generic lifetime are satisfied
    }

    // Lifetime annotation in struct definitions
    {
        // Structs can hold references, but a lifetime annotation must be on every reference.

        // The lifetime annotation means that an instance of ImportantExcerpt cant outlive the reference held in part
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let _ = ImportantExcerpt {
            part: first_sentence // reference to novel
        };

        //✅ The data in novel exists before the ImportantExcerpt instance is created
        //✅ novel goes out of scope after the ImportantExcerpt instance does
        // The reference in part is valid
    }

    // Inferred Lifetimes
    {
        // Rust can infer the lifetimes of references in functions in some cases, so no annotation is needed

        fn foo(s: &str) -> &str {
            &s
        }

        // This is because there are common deterministic patterns in the way lifetimes are annotated, so the compiler will
        // implicitly annotate and check them. If the compiler still can't figure out what the returned value's lifetime is,
        // then you have to annotate
    }

    // Lifetime annotation in methods
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        // Struct lifetimes must be declared in the impl regardless if they're used or not
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }
    }

    // Static lifetime
    {
        // 'static is a special lifetime that allows a reference to live for the entire program duration

        // All string literals have a static lifetime
        let _s: &'static str = "I have a static lifetime";
        // The text is stored directly in the program binary, which is always available

        // Rust will give an error message when there is a dangling reference or mismatched lifetimes.
        // Rust may suggest the use of 'static, but its often not necessary to fix the error.
    }

    // Example of generic types, trait bounds, and lifetimes together
    {
        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {ann}");
            if x.len() > y.len() { x } else { y }
        }
    }
}
