/* Summary:
Threads and async are the 2 frameworks Rust provides for concurrency.
Which is the right one to use is situation dependent.

Each thread uses some memory and has overhead when starting and finishing.
Aslo, threads are only an option when your OS and hardware supports them. 
i.e. some embedded systems don't have OS's and can't support threads.

Async works on tasks and switches between them by the Rust runtime.
This also comes with some overhead.
*/

use std::{thread, time::Duration};

use trpl::{ReceiverStream, Stream};

fn main() {
    // Many things that can be done by async can be done by threads and vice-versa.
    // i.e. the get_intervals() function from Chapter 17.4.
    //      Compare how similar they are.
    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        // This is *not* `trpl::spawn` but `std::thread::spawn`!
        thread::spawn(move || {
            let mut count = 0;
            loop {
                // Likewise, this is *not* `trpl::sleep` but `std::thread::sleep`!
                thread::sleep(Duration::from_millis(1));
                count += 1;

                if let Err(send_error) = tx.send(count) {
                    eprintln!("Could not send interval {count}: {send_error}");
                    break;
                };
            }
        });

        ReceiverStream::new(rx)
    }

    // Threads act as a boundary for sets of synchrounous operations.
    //      • Concurrency is possible between threads.
    //      • Threads run to completion with no interruptions (except by OS).
    // Tasks act as a boundary for serts of asynchronous operations.
    //      • Concurrency is possible between and within tasks.
    //      • A task can switch between futures in its body.

    // Threads are harder to compose than futures.
    // i.e. Much harder to have timeout and throttle functionality w/ threads.

    // Consider these suggestions to determine which method to use:
    //      • If the work is very parallizable, threads are better.
    //          • i.e. evaluating data where each part can be processed separately.
    //      • If the work is very concurrent, async is a better choice.
    //          • i.e. handling messages from many souces that arrive at different intervals.

    // Tasks and threads often work well together. Tasks are often moved between threads.

    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
