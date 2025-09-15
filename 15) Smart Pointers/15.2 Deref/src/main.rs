/* Summary:
Implementing the Deref trait allows you to customize behavior of the * (dereference operator).
By implementing Deref such that a smart pointer is treated like a regular reference, you can write
code that operates on both references and smart pointers.
*/

fn main() {
    // Dereference a reference to get a value
    {
        // A regular reference is a type of pointer, which stores/points to the memory location of another value.

        let x = 5;
        let y = &x; // y stores the location of x
        assert_eq!(x, *y); // *y gets the value stored at the location, 5 in this case 
    }

    // Dereference a Box to get a value
    {
        let x  = 5;
        let y = Box::new(x); // Box points to copy of x on the heap
        assert_eq!(x, *y); // The dereference works the same for a Box as it does for a reference
    }

    /* -------- Create a custom smart pointer w/ Deref trait -------- */

    // MyBox holds an item of any type
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        // Takes in an item and return a MyBox that contains it
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use::std::ops::Deref;
    // Impl the Deref trait for MyBox.
    impl<T> Deref for MyBox<T> {
        // Degines an associated type for Deref to use. It's a slightly diff way to declare a generic param
        type Target = T;

        // Deref trait requires the deref method, which returns a reference to the value in self.
        fn deref(&self) -> &Self::Target {
            // MyBox stores a tuple w/ one item in it, so return the reference to it
            &self.0
        }
    }

    /* -------------------------------------------------------------- */
    
    // Demo MyBox
    {
        

        let x = 5;
        let y = MyBox::new(x);
        
        assert_eq!(x, *y); // * works on MyBox b/c we impl'd the Deref trait for it.
        
        // Internally, *y is being run as *(y.deref())
        // The reason deref() doesn't return the value directly is to not transfer its ownership out.
    }

    // Deref coercion
    {
        /* 
        Deref coercion converts a reference of one type to another, if the first impl's Deref trait.
        i.e. convert &String to &str, since String impl's Deref so that it returns &str.
        
        Deref coercion happens automatically when a reference is passed into a param that expects a different type.
        */

        fn hello(name: &str) {
            println!("Hello, {name}")
        }

        let m = MyBox::new(
            String::from("Rust")
        );

        // Because MyBox impl's Deref, just provide a reference to the MyBox instead of the String inside.
        //      Rust internally converts the &MyBox<String> into an &str. It calls deref() on MyBox to get &String.
        //      Then it calls deref() again on &String to get &str.
        hello(&m);
    }

    // Deref coercion w/ mutability
    {
        /*
        Similar to how Deref overrides * on immutable references, DerefMut overrides * on mutable references.

        Rust does deref coercion when it finds types and trait implementations in 3 cases:
            1. From &T to &U when T: Deref<Target=U>
            2. From &mut T to &mut U when T: DerefMut<Target=U>
            3. From &mut T to &U when T: Deref<Target=U>

        The first converts an immutable reference of one type to another.
        The second converts a mutable reference of one type to another.
        The third converts a mutable reference of one type to an immutable reference of another.
        */
    }
}
