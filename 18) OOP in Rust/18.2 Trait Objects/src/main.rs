/* Summary:
Polymorphism is a concept in OOP where code works with data of several types.
Rust acheives this through trait objects.

To demonstrate this, we will use the example of a skeleton GUI library.
A list of different UI components will be iterated over and calls a draw() method,
which in a real GUI library would render it.
*/

/*
A trait object points to an instance of a type impl'ing a trait, and a table
used to look up trait methods on that type at runtime.

Create a trait object by specificing a pointer (i.e & or Box<T>), then 'dyn'
keyword, and then a trait.

Use trait objects in place of a generic or concrete type. Where it's used, Rust
will ensure at compile time that any val used in that context will impl the trait
of the trait object.

$ Use a trait object when you want a collection that contains different types
$ that all share the same trait. 
A generic type + trait bound only allows for a collection of one type
that shares the same trait.

Generics perform static dispatch. At compile time, Rust knows what version of a trait method you're calling.
Trait objects perform dynamic dispatch. It looks up which version of the trait method to call at runtime.
*/

use gui::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        // The components list contains 2 different UI types, but both impl the Draw trait.
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Option 1"),
                    String::from("Option 2"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),

            // If you were to add a type that doesn't impl Draw here, then it would give a compiler error.
        ],
    };

    screen.run();    
}
