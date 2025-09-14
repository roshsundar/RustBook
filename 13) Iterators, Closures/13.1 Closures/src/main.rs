/* Summary:
Closures are anonymous funcs that you can save in a variable or pass as args to a func.
You can create the closure in one place and then call it elsewhere to run it.
Unlike funcs, closures can capture values from the scope they're defined in, w/out them being passed in as args.
*/

use std::{thread, time::Duration};


fn main() {
    // unwrap_or_else()
    {
        // unwrap_or_else() method, defined on an Option<T>, has a closure as a param.
        //      • In this case it's (|| k * 2)
        //      • If a closure has params, they go between the ||. This closure doesn't have params
        //      • Notice how the closure can use k without it being passed as a param
        // If the Option<T> is Some(T) variant, then it returns the T value.
        // If the Option<T> is None variant, then it calls the closure and returns its value.
        let k = 10;
        assert_eq!(Some(4).unwrap_or_else(|| k * 2), 4);
        assert_eq!(None.unwrap_or_else(|| k * 2), 20);

        println!()
    }

    // Closure annotation & type inference
    {
        // This is a fully annotated closure
        let explicit_closure = |n: u32| -> u32 {
            println!("Intense calculations going on");
            thread::sleep(Duration::from_secs(2));
            n
        };
        println!("{}", explicit_closure(10)); // Run the closure and print the value

        // A function that adds 1 and a series of less explicit closures
        fn  add_one_v1   (x: u32) -> u32 { x + 1 }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| { x + 1 }; // note: this will compile err if it isn't used, since x isn't explicitly type annotated
        let add_one_v4 = |x|  x + 1  ; // note: this will compile err if it isn't used, since x isn't explicitly type annotated
        println!("{} {} {} {}", add_one_v1(1), add_one_v2(1), add_one_v3(1), add_one_v4(1)); // add_one_v3() and add_one_v4() are used here w/ an i32, so Rust can infer x's type
        
        // If the type of a closure isn't explicitly annotated, it is inferred by its first use.
        let example_closure = |x| x;
        let _s = example_closure(String::from("hello")); // Infers x to be a String
        //let _n = example_closure(5); //!✕ err: expects String, not i32

        println!()
    }

    // Capturing values into a closure
    {
        // Closures captures values from their environment the same way a function does:
        // immutable borrow, mutable borrow, take ownership. The closure will infer
        // which of these to use based on what it does w/ the values.

        //$ • Immutable borrowing in a closure

        let list = vec![1, 2, 3];
        println!("Before defining immutable closure: {list:?}");

        // This closure immutably borrows list.
        // Also notice that list doesn't need to passed in as an arg, like you would have to in a func.
        let immutably_borrows = || println!("From immutable closure: {list:?}");

        println!("Before calling immutable closure: {list:?}");
        immutably_borrows();
        println!("After calling immutable closure: {list:?}");

        println!();

        //$ • Mutably borrowing in a closure

        let mut list = vec![1, 2, 3];
        println!("Before defining mutable closure: {list:?}");

        // This closure mutably borrows list. list can't be mutably or immutably referenced until the closure is executed.
        let mut mutably_borrows = |x| list.push(x);

        //println!("Before calling mutable closure: {list:?}"); // !✕ err: can't make an immutable borrow of list when it is already mutably referenced.
        mutably_borrows(7);
        // The closure has executed and has ended its mutable reference. list can now be referenced again.
        println!("After calling mutable closure: {list:?}");

        println!();

        //$ • Moving ownership through a closure

        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        // This closure moves ownership of list through ٭move٭ into the new thread.
        thread::spawn(move || println!("From thread: {list:?}"))
            .join()
            .unwrap();
        // In fact Rust enforces the move here. The main thread may finish before the new thread.
        // So just trying to immutable reference list may not work, since list could be dropped already if the main thread is done.
        // So to be sure Rust mandates moving list since it can't be sure when the threads could complete.

        println!()
    }

    // Closure traits
    {
        // The way a closure captures and handles values from the environment affects which traits it implements.
        // A closure will auto implement one or more of the following Fn traits:
        //      • FnOnce - applies to closures that can be called once. All closures implement this trait.
        //      • Fnmut - applies to closures that don't move captured values out, but might mutate them.
        //      • Fn - applies to closures that don't move captured values out and don't mutate them, also
        //             closures that capture nothing from the environment.
    }

    // unwrap_or_else() and FnOnce
    {
        /* Below is the implementation of the unwrap_or_else() method on Option<T>

        impl<T> Option<T> {
            pub fn unwrap_or_else<F>(self, f: F) -> T
            where
                F: FnOnce() -> T
            {
                match self {
                    Some(x) => x,
                    None => f(),
                }
            }
        }

        The T is type of the value in the Some(T) variant. If the Option is a Some variant, then the method returns the value inside.

        The closure is passed in param f of type F, which is called if the Option is a None variant.
        The trait bound of F is FnOnce() -> T, which means F can be called at least once, takes no args, and returns a T.
        All closures impl FnOnce, so it can accept all 3 types of closures.
        */
    }

    // sort_by_key() and FnMut
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
        
        // sort_by_key() takes an FnMut closure as it's called multiple times, for each item in the slice
        list.sort_by_key(|r| r.width);
        println!("{list:#?}");

        println!();

        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
        
        /*
        Try to count the number of operations in the sort.

        let mut sort_operations = vec![];
        let value = String::from("closure called");

        list.sort_by_key(|r| {
            sort_operations.push(value); // !✕ err: can't do this beacuse FnMut won't allow value's String to be moved out into sort_operations
            r.height
        });
        println!("{list:#?}");
        */

        // Solution: change the closure body so that it doesn't move values out of the enviro
        
        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.height
        });
        println!("{list:#?}, sorted in {num_sort_operations} operations");
    }
}