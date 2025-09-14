/* Summary:
- Methods are similar to functions, but are defined in the context of a struct
- The first parameter is always *self*, which is the instance of the struct that called the method
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl implements the following functions for the Rectangle{} struct
impl Rectangle {
    /*
    - self is the instance of the Rectangle{} that called area().
    - Methods must have self as the first param.
    - Methods can borrow or take ownership of self like any other param.
        - Here we immutable borrow it
        - Rare to take ownership of self
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods can also have the name of a struct's field
    fn width(&self) -> bool {
        self.width > 0
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

// Also, it is possible to have multiple impl blocks for a struct
impl Rectangle {
    // Why is other an immutable reference?
    //  - Don't need to modify, so no mutable reference (&mut other)
    //  - Want to use the Rectangle{} passed into other after this fn is done
    //      - So no taking ownership (other)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// All fn's defined in an impl block are called associated funcions (including methods).
// Some associated functions are not methods, meaning that they don't have *self* as the first param.
// Such functions are usually used as constructors that return a new instance of the struct.
impl Rectangle {
    // *Self* is an alias for the impl type, Self = Rectangle in this case
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Methods that only need &self
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        // Method width() vs field width
        if rect1.width() {
            println!("rect1 one has a width of {}", rect1.width)
        }

        println!("The area of the rectangle is {}", rect1.area());

        println!()
    }

    // Method w/ &self and additional parameter
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        println!()
    }

    // Constructor example
    {
        // Square is an associated function that does not require an instance to use.
        // Therefore, to call it, use :: syntax
        let square = Rectangle::square(3);
        println!("Square: {square:#?}");
        println!()
    }

    // Method calls will handle referencing/dereferencing to get the struct automatically
    {
        let mut r = Rectangle {
            width: 1,
            height: 2,
        };

        // The two functions below are equivalent
        let area1 = r.area();
        let area2 = Rectangle::area(&r);
        assert_eq!(area1, area2);

        // The two functions below are equivalent
        r.set_width(2);
        Rectangle::set_width(&mut r, 2);

        // The method calls take care of providing the *self* param
        // Method calls will insert as many references and dereferences to get the *self*

        let r = &mut Box::new(Rectangle {
            width: 1,
            height: 2,
        });

        let area1 = r.area();
        let area2 = Rectangle::area(&**r); // 2 dereferences (cancels &mut and Box) gives the Rectangle, then borrow it to get &Rectangle
        assert_eq!(area1, area2);
    }

    // R and W permissions example
    {
        let mut rect = Rectangle {
            width: 0,
            height: 0,
        };

        rect.set_width(1);

        let rect_ref = &rect;
        // *rect_ref: R

        // rect_ref.set_width(2); //! err: set_width needs mut ref of rect, but rect_ref is immutable, so mutable borrow is not allowed
        // ^ requires W

        println!("Rect width {}", rect_ref.width);
        println!()
    }

    // O permissions example
    {
        let rect1 = Rectangle {
            width: 0,
            height: 0,
        };
        // rect1: RO

        let rect2 = Rectangle::square(1);
        // rect2: RO

        let _max_rect = rect1.max(rect2); // max() moves rect1 and rect to into itself and drops them after completion
                                          // rect1: -
                                          // rect2: -

        // println!("{} {}", rect1.area(), rect2.area()); //! err: borrow after move
        //                   ^ requires R  ^ requires R
    }

    // Another O permission example
    {
        /*
        impl Rectangle {
            fn set_to_max(&mut self, other: Rectangle) {
                *self = self.max(other); //! err: max() attempts to move *self (the Rectangle struct), but can't since &mut self is a mutable reference to that struct.
                                           !      Remember, moving ownership of data while there is a mut reference to it is not allowed by Rust.
            }
        }
        */

        // To solve this, can implement clone trait for Rectangle, which would allow it to be duplicated when self.max(other) is called.
        // The self param of self.max() would recieve a copy of the struct.
    }
}
