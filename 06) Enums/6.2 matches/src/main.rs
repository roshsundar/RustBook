/* Summary:
match allows you to compare a value against a series of patterns, and then execute code based on which pattern matches.
The compiler will confirm that all possible expressions of the value are considered.
*/

#[derive(Debug)]
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match takes an expression (coin), and evaluates it in an arm. This match has 4 arms.
    //      - matches must be exhaustive, covering every case.
    // In this case, the match will return a number
    match coin {
        Coin::Penny => 1, // arm 1 of the match, code is run after =>
        Coin::Nickel => 5,
        Coin::Dime => {
            // mutiple lines of code can be run with {}
            println!("Time for a dime!");
            10
        }
        Coin::Quarter(state) => {
            // if an enum variant has values stored, have to capture it in a var
            println!("Quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // Basic match example
    {
        let coins = [
            Coin::Penny,
            Coin::Nickel,
            Coin::Dime,
            Coin::Quarter(UsState::Alabama),
        ];

        let mut coins_value = 0;
        for coin in coins {
            coins_value += value_in_cents(coin)
        }
        println!("Value of the coin is: {}", coins_value);
        println!()
    }

    // Match result can be captured in a variable
    {
        let state = UsState::Alabama;
        let char_num = match state {
            UsState::Alabama => 7,
        };
        println!("Alabama is {char_num} letters long.");
        println!()
    }

    // match w/ Option<T>
    {
        let five = Some(5);
        let _six = plus_one(five); // = Some(6)
        let _none = plus_one(None); // = None
    }

    // Catch-all patterns and _ placeholder
    {
        let dice_roll = 7;
        // A var can be created at the end of a match statement to catch all remaining cases.
        // Fulfills the requirement that matches have to be exhaustive.
        match dice_roll {
            9 => println!("Lucky 9!"),
            other => println!("Something else: {other}"), // doesn't have to be named other
        }

        let dice_roll = 7;
        // The _ matches all remaining cases (like other), but doesn't capture the value
        match dice_roll {
            9 => println!("Lucky 9!"),
            _ => println!("Unlucky :("),
        }

        let dice_roll = 7;
        // () can be used to not run code for a particular arm
        match dice_roll {
            9 => println!("Lucky 9!"),
            _ => (),
        }

        println!()
    }

    // Unreachable arms
    {
        let dice_roll = 7;
        // Based on the patterns for each arm, and the order of the arms, some arms may never execute
        match dice_roll {
            9 => println!("Lucky 9!"),
            //9 => (), // Will never execute - all 9's caught by previous arm
            8 => (),
            _ => (),
            //6 => (), // Will never execute - previous arm catches all remaining values
        }
    }

    // matches and ownership
    {
        // In general, have to be careful when an enum contains non-copy data - i.e. String

        // If a match does not take ownership of a string, it can be used after
        let opt = Some(String::from("Hello world"));
        match opt {
            Some(_) => println!("Some!"),
            None => (),
        }
        println!("{opt:?}");

        // If a match does take ownership of a string, it can't be used after
        let opt = Some(String::from("Hello world"));
        match opt {
            Some(s) => println!("Some: {s}"), // s takes ownership of "Hello world"
            None => (),
        }
        // println!("{opt:?}"); //! Err: borrow moved value

        // The solution is to match on a reference
        let opt = Some(String::from("Hello world"));
        match &opt {
            Some(s) => println!("Some: {s}"), // s takes just a reference to "Hello world"
            None => (),
        }
        println!("{opt:?}");
    }
}
