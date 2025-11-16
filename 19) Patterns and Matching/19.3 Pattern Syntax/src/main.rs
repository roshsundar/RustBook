/* Summary:
Show valid pattern syntax which can be used in various situations.
*/

use core::hash;

fn main() {
    // Matching literals
    {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("some number"),
        }
    }

    println!();

    // Matching named variables
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"), // The y var inside this match is new, not the y var outside.
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    println!();

    // Multiple patterns w/ |
    {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"), // match 1 and 2
            3 => println!("three"),
            _ => println!("some number"),
        }
    }

    println!();

    // Matching ranges of values w/ ..=
    {
        let x = 5;
        match x {
            1..=5 => println!("one thru five"),
            _ => println!("something else"),
        }
    }

    println!();

    // Destructuring structs
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point {x: 0, y: 7 };

        let Point { x, y } = p; // Destructure p into x and y fields.
        assert_eq!(x, 0);
        assert_eq!(y, 7);

        // Destructure p in each arm of the match, while using literal values in the first 2.
        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x:0, y } => println!("On the y axis at {y}"),
            Point { x, y } => println!("On neither axis: ({x}, {y})"),
        }
    }

    println!();

    // Destructuring enums
    {
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            },
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            },
            Message::Write(text) => {
                println!("Text message: {text}");
            },
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
        }
    }

    println!();

    // Destructuring nested structs and enums
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Operation {
            ChangeColor(Color),
        }

        let op = Operation::ChangeColor(Color::Hsv(0, 160, 255));

        match op {
            Operation::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Operation::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}");
            }
        }
    }

    println!();
    
    // Destructuring structs and tuples
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
        println!("{feet}' {inches}\" x: {x} y: {y}");
    }

    println!();

    // Ignore a value w/ _
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {y}");
        }

        foo(3, 4);
    }

    println!();

    // Ignore specific parts of a val w/ a nested _
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"), // Don't need the values in either Some(i32).
            _ => setting_value = new_setting_value,
        }

        println!("setting is {setting_value:?}");
        
        println!();
        // _ can be used multiple times in a patterns to ignore particular values.

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}");
            }
        }
    }
    println!();

    // Create an unused variable by starting it's name with _
    {
        let _x = 5; // Unused variable.

        
        let s = Some(String::from("Hello!"));

        if let Some(_) = s { // if _ were _s instead, "Hello!" would move into _s, meaning that s couldn't be used again...
            println!("found a string");
        }

        println!("{s:?}"); // ... here!
    }

    println!();

    // Ignore parts of a value w/ ..
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 1, z: -1};

        match origin {
            Point { x, .. } => println!("x is {x}"), // Ignore y and z.
        }

        println!();

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => println!("Some numbers: {first}, {last}"),
        }
    }

    println!();

    // Extra conditionals w/ match guards.
    {
        // A match guard is an additional if condition after a pattern in a match arm, that
        // must match for that arm to be chosen.

        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"), // The match guard tests if x is even.
            Some(x) => println!(" The number {x} is odd"),
            None => (),
        }

        println!();
        // A match guard can be applied in match arms w/ multiple patterns.
        
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"), // The pattern matches 4, but since y is false this arm won't run.
            _ => println!("no"),
        }
    }

    // @ bindings
    {
        // @ lets us create a var that holds a val when we test that value for a pattern match.

        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id_variable @ 3..=7 } => println!("Found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => println!("Found an id in range 10 thru 12"),
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }
}
