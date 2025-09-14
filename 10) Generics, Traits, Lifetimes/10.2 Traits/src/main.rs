/* Summary:
A trait defines functionality a type has and can share w/ other types. It defines standard behavior in abstract ways.
A type's behavior is defined by its methods. Different types share the same behavior if the same methods can be called.

Trait bounds specify that a generic type can be any type that also has a certain behavior.
*/

#![allow(warnings)]

mod summarizer;

use std::fmt::Display;

// Summary trait has to be brought into scope for summarize() to be used
use summarizer::*;

fn main() {
    let tweet = Tweet {
        username: String::from("NaopleonBonaparte"),
        content: String::from(
            "Russia kinda cold fr fr. Borodino not bussin on god no cap!"
        ),
        reply: false,
        retweet: false,
    };

    println!("New tweet from {}", tweet.summarize());
    println!("{}", tweet.get_author_summary());

    println!();

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article: {}", article.summarize());
    println!("{}", article.get_author_summary());

    // Traits as params
    {
        // item can be a reference to any type which implements the Summary trait, i.e. NewsArticle or Tweet
        fn notify1(item: &impl Summary) {
            // Since item impl's Summary, any of those methods can be called on it
            println!("Breaking news! {}", item.summarize())
        }

        // This is the longform of the above func
        // Summary is a trait bound for the generic T, meaning that T has to impl Summary
        fn notify2<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize())
        }

        // item1 and item2 can be different types, so long as both impl Summary
        fn lookat1(item1: &impl Summary, item2: &impl Summary) {
            notify1(item1);
            notify2(item2);
        }

        // item1 and item2 must be the same type that impl Summary
        // The T generic is the same for item1 and item2, and T is specified to impl Summary
        fn lookat2<T: Summary>(item1: &T, item2: &T) {
            notify1(item1);
            notify2(item2);
        }
    }

    // Multiple trait bounds
    {
        // item can be a reference to any type that implements the Summary and Display trait
        fn tell_and_show1(item: &(impl Summary + Display)) {}

        // Trait bound version of above func
        fn tell_and_show2<T: Summary + Display>(item: &T) {}

        // Too many trait bounds in function signature can look confusing
        // the *where* syntax allows trait bounds to be defined after functon signature for visual clarity
        fn some_func <T, U>(t: &T, u: &U) -> i32
        where 
            T: Display + Clone,
            U: Clone + Summary,
        {
            return 1;
        }
    }

    // Return a type that implements a trait
    {
        // The func can return any type that implements Summary
        fn returns_summarizable() -> impl Summary {
            // In this case, return a Tweet
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }

            // However, impl Summary can only return a single type
            // ! trying to return either a NewsArticle OR a Tweet would error
        }
    }

    // Use trait bounds to conditionally implement methods
    {
        struct Pair<T> {
            x: T,
            y: T,
        }
        
        // new() will work with a Pair of any type
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        // summarize_the_pair() will only be available for Pairs where the type implements Summary and Display traits
        impl <T: Summary + Display> Pair<T> {
            fn summarize_the_pair(&self) {
                println!("{}", self.x.summarize());
                println!("{}", self.y.summarize());
            }
        }

        // In other words, this code is saying that you can create a Pair of any type
        // but only a Pair of Tweets or a Pair of NewsArticles can call summarize_the_pair()
    }
    
    // Can also conditionally impl a trait for any type that impl's another trait
    // AKA a blanket implementation
    {
        /*
        i.e. Rust standard library implements ToString trait for any type that also impl's Display trait

        impl<T: Display> ToString for T {
            // --snip--
        }

        B/c the standard library has this blanket implementation, we can call to_string() - defined by the ToString trait - 
        on any type that implements the Display trait. 
        
        i.e. convert ints into String values b/c integers implement Display:
        let s = 3.to_string();
        */
    }
}