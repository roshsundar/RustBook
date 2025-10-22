/* Summary:
Use the join! macro and the join_all() method to await the completion of multiple futures.

Race futures against each other with the select() method.

Use the yield_now() method to pause and await, allowing other futures to continue execution.

Create custom async abstractions that operate on futures.
*/

use std::{pin::{pin, Pin}, thread, time::Duration};

use trpl::Either;

fn main() {
    // Run a set number of futures w/ the join! macro.
    {
        let fut1 = async {
            trpl::sleep(Duration::from_millis(1000)).await;
            println!("Future 1 returning a bool");
            true
        };

        let fut2 = async {
            trpl::sleep(Duration::from_millis(500)).await;
            println!("Future 2 returning an i32");
            5
        };

        let fut3 = async {
            println!("Future 3 returning an &str");
            "Hello!"
        };

        trpl::run(async {
            let (fut1_res, fut2_res, fut3_res) = trpl::join!(fut1, fut2, fut3); // join! awaits an arbitrary amnt of futures.
            // The join! macro also works with futures of different types. fut1, fut2, and fut3 are all different types.

            println!("{fut1_res} {fut2_res} {fut3_res}");
        });
    }

    println!();

    // Run a dynamic amnt of futures w/ join_all().
    {
        // Each future must be wrapped in a Pin<>.

        let fut1 = pin!(async {
            trpl::sleep(Duration::from_millis(1000)).await;
            println!("Future 1");
        });

        let fut2 = pin!(async {
            trpl::sleep(Duration::from_millis(100)).await;
            println!("Future 2");
        });

        let fut3 = pin!(async {
            trpl::sleep(Duration::from_millis(500)).await;
            println!("Future 3");
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![fut1, fut2, fut3];
        // The type is: A vector of pinned mutable references to futures that return ().
        // Note that all the futures have to return the same type to be in the same list.

        // Run the futures and wait for them to finish.
        trpl::run(async {
            trpl::join_all(futures).await;
        });
    }

    println!();

    // Racing futures.
    {
        let slow = async {
            println!("slow task started");
            trpl::sleep(Duration::from_millis(1000)).await;
            println!("slow task finished")
        };

        let fast = async {
            println!("fast task started");
            trpl::sleep(Duration::from_millis(500)).await;
            println!("fast task finished");
        };

        // Run both futures and wait for one to complete, then move on.
        trpl::run(async {
            trpl::select(slow, fast).await;
        });
    }

    println!();

    // Yielding control to the runtime
    {
        /* 
        If you do a lot of work in an future without an await point, it will block other futures from making progress,
        because .await allows Rust to pause a task and switch to another while it waits.
        
        If you are doing some long-running work, or need a future to work on something indefinitely, then you
        need a way to return control back to the runtime.
        */

        // This fn simulates some long-running process that blocks execution.
        fn slow(name: &str, ms: u64) {
            thread::sleep(Duration::from_millis(ms)); // will block execution for ms
            println!("{name} ran for {ms}ms");
        }

        // Let's say we have 2 slow futures. Both only allow potential for Rust to switch at the end when the future is done.
        // The way each future is, there is no way for Rust to switch between a & b after each slow() call.
        // a will have to finish its work before b can start.
        {
            let a = async {
                println!("a started");

                slow("a", 30);
                slow("a", 10);
                slow("a", 20);

                println!("a finished")
                // Rust could switch to work on b here, but not before. a's work has already been done. 
            };

            let b = async {
                println!("b started.");

                slow("b", 75);
                slow("b", 10);
                slow("b", 15);
                slow("b", 350);

                println!("b finished")
                // Rust could switch to work on a here, but not before. b's work has already been done. 
            };

            trpl::run(async {
                trpl::join!(a, b);
            })
        }

        println!();
        
        // yield_now().await allows Rust to switch between work on a & b.
        // It effectively says 'This future will let something else run, if there's nothing, then continue'
        // a & b will now alternate running slow() calls
        {
            let a = async {
                println!("a started");

                slow("a", 30);
                trpl::yield_now().await;
                slow("a", 10);
                trpl::yield_now().await;
                slow("a", 20);
                trpl::yield_now().await;

                println!("a finished")
            };

            let b = async {
                println!("b started.");

                slow("b", 75);
                trpl::yield_now().await;
                slow("b", 10);
                trpl::yield_now().await;
                slow("b", 15);
                trpl::yield_now().await;
                slow("b", 350);
                trpl::yield_now().await;

                println!("b finished")
            };

            trpl::run(async {
                trpl::join!(a, b);
            })
        }
    }

    println!();

    // Create custom async abstractions.
    {
        // Function that takes in a future and times it to see if it finishes before the limit.
        async fn timeout<F: Future>(
            future_to_try: F,
            max_time: Duration,
        ) -> Result<F::Output, Duration> {
            // Race the future and the time limit to see which finishes first
            match trpl::select(future_to_try, trpl::sleep(max_time)).await {
                Either::Left(output) => Ok(output), // If future finishes before the limit, return its output in an Ok.
                Either::Right(_) => Err(max_time), // If the time limit finishes first, return an Err w/ the time.
            }
        }

        trpl::run(async {
            let slow = async {
                trpl::sleep(Duration::from_secs(4)).await;
                "slow finished!"
            };

            match timeout(slow, Duration::from_secs(2)).await {
                Ok(output) => println!("slow succeeded with output: {output}"),
                Err(duration) => println!("slow failed after {} seconds", duration.as_secs()),
            }

            let fast = async {
                trpl::sleep(Duration::from_secs(1)).await;
                "fast finished!"
            };

            match timeout(fast, Duration::from_secs(2)).await {
                Ok(output) => println!("fast succeeded with output: {output}"),
                Err(duration) => println!("fast failed after {} seconds", duration.as_secs()),
            }
        });
    }
}
