/*
Rust scope rules prevent implementing external (outside of crate) traits on external types.
    i.e. implement Display trait on Vec<T> type, both outside of this crate

Can implement an external trait on an internal type.
    i.e. implement Display trait (external) on Tweet type (internal)

Can implement an internal trait on an external type.
    i.e. implement Summary trait (internal) on Vec<T> type (external)
*/

// To call summarize() in other files, Summary must be public
pub trait Summary {
    // Each type implementing Summary can either use the default behavior of summarize() or
    // override it to have its own behavior for summarize().
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    // Default implementations can call other methods in the trait, even if those don't have default implementations
    fn get_author_summary(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement Summary behavior for NewsArticle
impl Summary for NewsArticle {
    // Use the default behavior of summarize()

    // Implement summarize_author()
    fn summarize_author(&self) -> String {
        format!("{} at {}", self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement Summary behavior for Tweet
impl Summary for Tweet {
    // Override the default behavior of summarize()
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}