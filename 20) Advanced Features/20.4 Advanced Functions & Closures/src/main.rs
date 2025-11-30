/*Summary:
Some advanced features related to functions and closures.
*/

fn main() {
    // Function pointers.
    {
        // Functions can be passed as args into other functions.
        // The fn type is called a function pointer (FP).

        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        assert_eq!(10, do_twice(add_one, 4));

        // FPs impl all 3 closure traits (Fn, FnMut, FnOnce). So, you can always pass a FP as an arg for a function that expects a closure.
        // Generally, you should write functions w/ a generic type bound to a closure trait so it can take both.

        // One example where you would only want fn and not closures is interfacing w/ external code.
        // i.e. C doesn't have closures.
    }

    // Returning Closures.
    {
        // Closures/functions don't have a concrete type that is returnable.
        // Instead, you return an impl Trait w/ one of the closure traits.

        fn returns_closure() -> impl Fn(i32) -> i32 {
            |x| x + 1
        }

        let closure = returns_closure();
        assert_eq!(4, closure(3));

        // However, each closure is its own type.
        // If you need to work w/ multiple functions that return closures, you need trait objects.

        fn returns_a_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

        fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |x| x + init)
        }

        let handlers = vec![returns_a_closure(), returns_initialized_closure(4)];
        for handler in handlers {
            println!("{}", handler(5));
        }
    }
}
