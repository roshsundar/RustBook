/* Summary:
Generics allow definitions for functions, structs, enums, & methods that can be 
used with many concrete data types.
*/

// The <> syntax allows a generic type, T in this case, to be used in the function.
// _thing can by any type - i32, String, structs, and more
fn _generic_function<T>(_thing: T) {
    /*
    Not showing an example in this function because the ability to do things is limited.
    Since T is generic, Rust can't make assumptions about what T can do.
    I.e. T could be an i32, meaning _thing += 1 is valid.
    However, T could be String, which meants that _thing += 1 isn't valid.
    The use of traits can overcome this, and will be discussed in the next section.
    */
}

fn main() {
    // Generics in structs
    {
        // The <> syntax allows a generic type, T in this case, to be used in the struct
        struct Something<T> {
            s: T // Can store any kind of value here
        }

        struct SeveralThings<T, U> {
            // thing1 and thing2 can be any type
            thing1: T,
            thing2: U,
        }

        let i = Something { s: 1} ;
        let s = Something { s: "Hi!" };
        println!("{} {}", i.s, s.s);

        let t = SeveralThings { thing1: "another", thing2: 2.5 };
        println!("{} {}", t.thing1, t.thing2);

        println!()
    }

    // Generics in enums
    {
        /*
        Rust has standard enums that make use of generics.

        enum Option<T> {
            Some(T),
            None,
        }

        It has a generic T. The Some variant holds a value of type T. The None variant holds no value.
        This allows the concept of an optional value for any kind of value.

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        It has generic T and E. The Ok variant holds a type T value, the Err variant holds a type E value.
        This allows the ability to represent an operation that succeeds or fails,
        with a custom result type and error type.
        */
    }
    
    // Generics on methods
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        // The impl block needs to know T as a generic type rather than a concrete type (i.e. i32)
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        let p1 = Point { x: "2.0", y: "4.0"};
        println!("{} {}", p1.x(), p1.y);

        // Constraints on generic types can be specified when defining methods.
        // In this case the distance_from_origin() method is only defined on instances of Point<f64>.
        impl Point<f64> {
            fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let p2 = Point { x: 2.0, y: 4.0 };
        println!("{}", p2.distance_from_origin());
        // println!("{}", p1.distance_from_origin()) //! err: the function is only defined on instances of Point<f64>, p1 is a Point<&str>

        /*
        $ A specific and generic method of the same name can't be implemented.

        I.e. distance_from_origin() method can't also be implemented for Point<T>. Otherwise, Rust would not know which
        method to use then you call the method for a Point<f64>, the specific version or generic version?
        */

        println!()
    }

    // Different generics in the impl and method
    {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        // The impl block can use the same generics as the Point struct
        impl<X1, Y1> Point<X1, Y1> {
            /*
            mixup() declares 2 additional generics to allow the other instance to have whatever types it wants.
            
            If a Point<i32, f64> had mixup() called on it, and *other: Point<X1, Y1>* instead,
            then other would also have to be a point of i32 and f64.
            
            In other words, X1 & Y1 are the generics for the Point instance that calls this method.
            Y1 & Y2 are the generics for the other Point instance.
            */
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point { x: self.x, y: other.y }
            }
        }

        let p1 = Point { x: 5, y: 10.4};
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);
        println!("{} {}", p3.x, p3.y);

        println!()
    }
}
