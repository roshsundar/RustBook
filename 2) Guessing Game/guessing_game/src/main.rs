use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate random num between 1 & 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read guess from terminal
        io::stdin()
            .read_line(&mut guess) // Result is an enum that can be Ok or Err
            .expect("Failed to read line"); // Causes program to crash and print text if Result is Err

        // Typecast guess as an int
        // parse() knows to convert to int, because guess is specified as u32
        // Also note that guess needn't be mut, as it isn't modified later
            // Compiler will throw warning if you make it mut
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
