/* Summary:
A Box, Box<T>, is a type of smart pointer. It allows you to store data on the heap and point to it in the stack.

Boxes are used most often in the following situations:
    • When you have a type whose size isn't known at compile time, and you want
        to use it in a context that requires an exact size.
    • When you have a large amnt of data and you want to transfer ownership and not copy it.
    • When you want to own a value and have it impl a specific trait instead of being a specific type.
*/

fn main() {
    // Store a value on the heap
    {
        // b points to the location on the heap that containts 5
        let b = Box::new(5);
        println!("{b}");

        // b goes out of scope here and will be dealloc'd.
        // This goes for the Box pointer (on stack), and 5 (on heap).
    }

    // Boxes are often used to store recursive types, since their size isn't knowable at compile time

    /*
    An example of a recursive type is a cons list, which is like a linked list.

    A pseudocode represtation of [1, 2, 3] in cons is (1, (2, (3, Nil))).
    Each item contains a value and the next item.
    The last value Nil doesn't have a next item and terminates the list.
    */
    {
        use crate::List::{Cons, Nil};
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}

/*
The Cons variant stores a pointer (Box) to another List, so the size is known at compile time.
Otherwise, the size of List wouldn't be known, as a List could store a List which stores a List, etc.
*/
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}