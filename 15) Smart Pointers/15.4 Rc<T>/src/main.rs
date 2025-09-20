/* Summary:
Rc<T>, or the reference counter, is a type of smart pointer that enables multiple owners for a single value.
$ Note that Rc allows read only access.

An Rc keeps track of the # of references to a value to determine if it's in use.
If there are 0 references, the value is dropped.

Rc<T> is used when we want to allocate some data for multiple parts of the program to read,
and we can't know at compile time which will finish last.
*/

use std::rc::Rc;

fn main() {
    // Rc and cloning
    {
        // Wrap "Hello world" into an Rc, starts with a reference count of 1
        let s = Rc::new(String::from("Hello word"));
        
        // Clone the Rc, reference count of s is 2
        let s1 = Rc::clone(&s);

        // Clone the Rc again, reference count of s is 3
        let s2 = Rc::clone(&s);

        // Note that cloning the Rc does not copy the underlying data

        // Ownership of "Hello world" is shared by s, s1, and s2
        println!("Original: '{s}', Rc 1: '{s1}', Rc 2: '{s2}'")
    }

    println!();

    // Reference count
    {
        let s = Rc::new(String::from("Hello word"));

        // The reference count of "Hello word" is 1
        assert_eq!(1, Rc::strong_count(&s));

        let s1 = Rc::clone(&s);

        // The reference count of "Hello word" is 2
        assert_eq!(2, Rc::strong_count(&s));
        assert_eq!(2, Rc::strong_count(&s1));

        {
            let s2 = Rc::clone(&s);

            // The reference count of "Hello word" is 3
            assert_eq!(3, Rc::strong_count(&s));
            assert_eq!(3, Rc::strong_count(&s1));
            assert_eq!(3, Rc::strong_count(&s2));

            // s2 is dropped here, the reference count of "Hello word" decreases by 1
        }

        // The reference count of "Hello word" is 2
        assert_eq!(2, Rc::strong_count(&s));
        assert_eq!(2, Rc::strong_count(&s1));
    }

    println!();

    // Once the reference count of a value = 0, it is dropped
    {
        struct Example;
        impl Drop for Example {
            fn drop(&mut self) {
                println!("Example dropped!");
            }
        }

        let x = Rc::new(Example);
        let y = Rc::clone(&x);

        println!("Reference count for Example is {}", Rc::strong_count(&x));

        println!("Drop the Rc's to Example");
        drop(x);
        drop(y);
        // There are no more Rc's to Example, so it will be dropped here.

        println!("All Rc's to Example have been dropped");
    }
}
