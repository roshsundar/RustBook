/* Summary:
Understand how to use Rust async features to make a cli program that takes 2 URLs, fetches their
content concurrently, and returns the result of whichever finishes first.
*/

use std::env::args;

use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = args().collect();

    // Create a runtime that takes in a future and runs it.
    trpl::run(async { // The async block runs all the async code inside and returns a future.
        // Create the futures for each URL.
        // futures don't do anything until they're awaited, kinda like iterators with collect().
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = 
            // Run both futures to fetch the titles. Select which one finishes first.
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left, // URL 1 finished first
                Either::Right(right) => right, // URL 2 finished first.
            };
        
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is {title}"),
            None => println!("Its title couldn't be parsed"),
        }
    })
}


async fn page_title(url: &str) -> (&str, Option<String>) { // Internally, this fn desugars into a regular fn, that runs async code in an async block and returns a future.
    // Fetch all the data from the URL, await the response which may take a while.
    let response = trpl::get(url).await;
    
    // The entirety of the response needs to arrive first, so need to await that.
    let text = response.text().await;

    // Parse the HTML text to get the title
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}