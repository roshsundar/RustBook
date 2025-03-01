/* Summary:
Example of how to create and print structs.
*/

#[derive(Debug)] // *Debug* trait allows the struct to be printed. The *derive* attribute adds the functionality
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the struct w/ formatted println!
    {
        println!("rect1 = {rect1:#?}"); // Using the :#? instead of :? allows for nicer formatting of the struct
        println!()
    }

    // Print the struct w/ dbg! macro
    {
        // dbg! takes ownership of an expression, as opposed to println! taking a reference.
        // Prints the line # where the dbg! occurs, along with the resulting value of the expression.
        // Returns ownership back, so it can be recaptured in a var.

        dbg!(&rect1); // Don't want dbg! to take ownership of rect1, so give a reference
        println!()
    }

    println!("Area of rect1 is {}", area(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
    dbg!(rect.width * rect.height) // dbg! prints this expression and returns the ownership of area value to this fn
                                   // The area then gets returned from this fn
}
