/* Summary:
Some advanced uses of traits.
*/

fn main() {
    // Associated types
    {
        // Associated types connect a type placeholder w/ a trait, allowing its method definitions to use it.
        // The implementor of the trait specifies the concrete type to be used for the placeholder in that specific impl.
        // Per type, an associated-type trait can only have one impl.

        trait Container {
            type Item; // Associated type

            fn get(&self) -> &Self::Item;
        }

        struct NumberBox {
            value: i32,
        }

        impl Container for NumberBox {
            type Item = i32;
            fn get(&self) -> &Self::Item {
                &self.value
            }
        }

        let n = NumberBox { value: 5 };
        assert_eq!(5, *n.get());
    }

    // Default generic type params and operator overloading
    {
        // When we use generic params, we can specify default concrete type for the generic type.
        // They are used mainly in 2 ways:
        //  • To extend a type w/o breaking existing code
        //  • To allow customization in specific cases most users won't need

        // Operator overloading is a useful case for this, where you can customize
        // operator behaviors (i.e. +) in certain situations.

        use std::ops::Add;

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Self) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 },
        );

        /* Add trait definition:
        trait Add<Rhs=Self> { //<- Rhs is the default type param, which defaults to the type impl'ing Add
            type Output; //<- associated type for the output of add()

            fn add(self, rhs: Rhs) -> Self::Output;
        }
        */
        // For the Point impl above, we used the default for Rhs b/c we wanted to add 2 Points.

        // Example of customizing the Rhs type.

        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }

        let millis = Millimeters(1000);
        let meter = Meters(1);
        let total = millis + meter;
        assert_eq!(2000, total.0);
    }

    // Disambiguating between methods of the same name
    {
        // Rust allows traits to have methods w/ the same name as each other, and impl them on the same type.

        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Wingardium Leviosa!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);

        println!();
        
        // For associated functions, you have to use fully qualified syntax.

        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", Dog::baby_name());
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // Fully qualified syntax
        // It specifies that we treat Dog as an Animal for this func call.
    }

    println!();

    // Supertraits
    {
        // Sometimes you may want to define a trait that depends on another trait. The dependency is called a supertrait.
        // The trait can use the associated items of the supertrait.

        use std::fmt;

        trait OutlinePrint: fmt::Display { // OutlinePrint will only work for types that also impl Display.
            fn outline_print(&self) {
                let output = self.to_string(); // to_string() is defined by Display, so we can use it here.
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {output} *");
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }    
        }

        struct Point {
            x: i32,
            y: i32,
        }

        impl OutlinePrint for Point {}

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        Point { x: 1, y: 3 }.outline_print();
    }

    println!();

    // Using the newtype pattern to impl external traits on external types
    {
        // Normally we can only impl a trait on a type if the trait, type, or both are local to the crate.
        // The newtype pattern can circumvent this by creating a new type in a tuple struct.

        // Let's say we want to impl Display on Vec<T>, both of which are defined outside of this crate.
        
        use std::fmt;

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("{w}");
    }
}