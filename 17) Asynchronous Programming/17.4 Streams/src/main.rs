/* Summary:
A stream in async Rust represents a sequence of values that can be processed over time.
It's basically like an async version of an iterator.
*/

use std::{pin::pin, time::Duration};

use rand::Rng;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    // Create a stream from an iterator. Apply processing methods to it, like with iterators.
    {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n* 2); // Multiply each element by 2 and create an iterator.
        let stream = trpl::stream_from_iter(iter); // Turn the iterator into a stream.
        let mut filtered = stream.filter(|&value| value <= 10); // Filter the stream for any element <= 10.

        trpl::run(async {
            while let Some(value) = filtered.next().await { // iterate over the filtered stream with next()
                println!("The value was: {value}");
            }
        });
    }

    println!();

    // Composing streams from channels.
    {
        // Example of a function to send a sequence of letters through a channel and be received by a stream.
        // Each letter has a delay arriving to the stream. The stream puts a time limit on the arrival of each item.

        fn get_messages() -> impl Stream<Item = String> {
            let (tx, rx) = trpl::channel();

            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

            // Send the letters to receiver's queue, w/ a random delay.
            trpl::spawn_task(async move {
                for message in messages {
                    let delay = {
                        let mut rng = rand::rng();
                        rng.random_range(100..=300)
                    };

                    trpl::sleep(Duration::from_millis(delay)).await;
                    tx.send(format!("Message: '{message}'")).unwrap();
                }
            });

            // Convert the receiver into a stream.
            ReceiverStream::new(rx)
        }

        trpl::run(async {
            let messages = get_messages(); // This stream contains the letters.
            let mut messages_under_limit = pin!(messages.timeout(Duration::from_millis(200))); // Add a timeout to each item in the stream.

            while let Some(result) = messages_under_limit.next().await { // Iterate over the stream of letters.
                match result {
                    Ok(message) => println!("{message}"), // The letter was received before the timeout.
                    Err(reason) => eprintln!("Problem: {reason:?}"), // The letter didn't arrive before the timeout.
                }
            }
        });   
    }

    println!();

    // Merging streams.
    {
        fn get_intervals() -> impl Stream<Item = u32> {
            let (tx, rx) = trpl::channel();

            trpl::spawn_task(async move {
                let mut count = 0;
                loop {
                    trpl::sleep(Duration::from_millis(1)).await;
                    count += 1;
                    tx.send(count).unwrap();
                }
            });

            ReceiverStream::new(rx)
        }

        fn get_messages() -> impl Stream<Item = String> {
            let (tx, rx) = trpl::channel();

            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

            trpl::spawn_task(async move {
                for message in messages {
                    let delay = {
                        let mut rng = rand::rng();
                        rng.random_range(100..=300)
                    };

                    trpl::sleep(Duration::from_millis(delay)).await;
                    tx.send(format!("Message: '{message}'")).unwrap();
                }
            });

            ReceiverStream::new(rx)
        }

        trpl::run(async {
            let messages = get_messages().timeout(Duration::from_millis(200));

            // Make the intervals stream match the messages stream, so they can be merged.
            let intervals = get_intervals()
                .map(|count| format!("Interval: {count}")) // Make the output into a String.
                .throttle(Duration::from_millis(100)) // Slow down the arrival of each item from the stream by 100ms.
                .timeout(Duration::from_secs(10)); // Add a timeout. Since we don't need a timeout for intervals, make it long to not interfere.

            let merged = messages.merge(intervals).take(40); // Stop after pulling 40 items from the stream.
            let mut stream = pin!(merged);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(message) => println!("{message}"),
                    Err(reason) => println!("Problem: {reason}"),
                }
            }
        });
    }
}
