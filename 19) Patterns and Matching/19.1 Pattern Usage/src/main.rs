/* Summary:
Patterns are used in several contexts in Rust and are quite common. 
*/

fn main() {
    // `match` arms
    {
        /*
        Patterns are used in the arms of match expressions.
        
        match VALUE {
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
        }
        */

        let x = Some(3);

        // The patterns in this match are None and Some(val)
        match x {
            Some(val) => println!("{val}"),
            None => (),
        }
    }

    println!();

    // `if let` expressions
    {
        let favorite_color: Option<String> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        // Patterns are used on the left side of the =

        if let Some(color) = favorite_color {
            println!("Using {color} as the background color");
        } else if is_tuesday {
            println!("Tuesday is green day");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    println!();

    // `while let` conditional loops
    {
        use std::sync::mpsc::channel;
        use std::thread;

        let (tx, rx) = channel();
        thread::spawn(move || {
            for val in [1, 2, 3] {
                tx.send(val).unwrap();
            }
        });

        while let Ok(value) = rx.recv() {  // Patterns are used on the left side of the =
            println!("{value}");
        }
    }

    println!();

    // `for` loops
    {
        let v = vec!['a', 'b', 'c'];

        // The expression directly after the `for` is a pattern.

        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
    }

    println!();

    // `let` statements
    {
        // let statements also use patterns.
        // let PATTERN = EXPRESSION;

        let (x, y, z) = (1, 2, 3);
        println!("x={x} y={y} z={z}");

        // If the num of elem in the pattern doesn't match the tuple, there'll be a compiler error.
        // let (x, y) = (1, 2, 3); //! err: expected a tuple with 3 elements, found one with 2 elements.
    }

    println!();
    
    // Function params
    {
        fn print_coords(&(x, y): &(i32, i32)) {
            println!("Current location: ({x}, {y})");
        }

        let point = (3, 5);
        print_coords(&point);
    }
}
