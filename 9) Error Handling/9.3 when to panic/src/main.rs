/* Summary:
Most times its best to use Result to handle errors, since you have control on how to handle them.

For certain situations it's better to panic! - i.e. examples, prototyping, tests.

Code should panic! when it's possible the code could be in a bad state - when something unexpected happens such that continuing the code
could be harmful.
    - When some assumption/guarantee has been broken (i.e. invalid, contradicting, or missing values) that the program needs to function
panic! is a good option for calling external code that might fail and you can't fix.
*/

use std::net::IpAddr;

fn main() {
    // If you can garuntee that a Result will always be Ok variant, then substituting w/ unwrap() or expect() - both of which panic! - is fine.
    {
        let _home: IpAddr = "127.0.0.1" // We know this IP addr is always valid
        .parse() // Still, parse() returns a Result
        .expect("Hardcoded IP address should be valid"); // We know the Result will always be Ok, so expect() is fine
        
        // If the IP addr was instead input by a user, a more robust way of handling the Result would be needed
    }

}

/*
panic! and result are also useful to make sure callers/users of your code are obeying your restrictions.

Any user of your Guess struct below must make sure the instance they create has a value between 1 & 100.
*/

pub struct Guess {
    value: i32,
}

impl Guess {
    // Guess::new1() should be used if it is absolutely critical that value must be between 1 and 100
    pub fn new1(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // Guess::new2() should be used if value must be between 1 and 100, but is not critical and the caller can handle the mistake
    pub fn new2(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!("Guess value must be between 1 and 100, got {value}."))
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}