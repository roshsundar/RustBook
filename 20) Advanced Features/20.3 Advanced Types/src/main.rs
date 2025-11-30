/* Summary:
Some advanced uses of types.
*/
fn main() {
    // Using the newtype pattern for type safety and abstraction.
    {
        /*
        Newtypes can be used to ensure values are not being confused.
        i.e. struct Millimeters(u32) means that functions that need to work w/ the concept of millimeters
             can use a specific type as opposed to plain u32 values.
        
        Newtypes can abstract implementation details of a type. The newtype can expose a public API different than
        the API of the private inner type.
        i.e. Create a People type that wraps around a HashMap<i32, String> that stores a id-name pair.
             Code using People would have a public API, like a method to add a name to the hashmap.
             Internally, a number is generated for the id, and the user doesn't need to know that to use People.
        */
    }

    // Create type synonyms w/ type aliases.
    {
        // A type alias gives an existing type another name. They will be treated as interchangable.

        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;
        assert_eq!(10, x + y);

        // The primary usage is to make long and complex types into a shorthand.
        // i.e. type ComplexThing = Box<dyn Fn() + Send + 'static>;
        // Now you can use ComplexThing instead of Box<dyn Fn() + Send + 'static>
    }
    
    // The never type that never returns.
    {
        // Rust has a special `!` type, the never type, that is used when a function doesn't return.
        fn bar() -> ! {
            loop {
                // Loops forever. loop returns the ! type.
            }
        }

        // panic! also returns !, since it quits the program.
    }

    // Dynamically sized types and the sized trait.
    {
        // Rust needs to know the size of its types.
        // Some are statically sized and we know the size at compile time, i.e. i32 = 4 bytes.
        // Some are dynamically sized (DS), where size is known only at runtime.

        // An example of this is str which is DS, as they can have variable lengths.
        // i.e. "Hello" or "How's it going partner?".
        // Rust doesn't allow variables of type str to be created or used.
        // You use a pointer to str, &str.

        // The general rule is that DSTs can only be accessed through a pointer of some kind.
        // Recall in Chapter 18.2 that trait collections of various types are also like this, as traits are DS.

        // To work with DSTs, Rust provides the Sized trait to determine whether a type's size is known at compile time.
        // Rust also implicitly adds a traitbound for Sized on every generic function.
        // i.e. fn generic<T>(t: T) {} is equivocated to fn generic<T: Sized>(t: T) {}

        // By default generic functgions only work on types w/ known size at compile time.
        // Using special syntax you can lift this restriction.
        // fn generic<T: ?Sized>(t: &T) {}
        // The ?Sized means T may / may not be Sized. This syntax only works for Sized and not other traits.
        // As the type may not be Sized, it has to be used behind a pointer, hence &T instead of T.
    }
}
