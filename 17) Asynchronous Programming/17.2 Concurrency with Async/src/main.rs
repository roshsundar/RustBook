/* Summary:
Examples of using async to solve concurrency problems that were tackled w/ threads in Chapter 16.
*/

use std::time::Duration;

fn main() {
    // Counting up on a task.
    {
        trpl::run(async {
            let handle = trpl::spawn_task(async{
                for i in 1..5 {
                    println!("{i} from the task.");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            // Run the task and wait for it to finish.
            handle.await.unwrap();
        });
    }

    println!();

    // Counting up on separate tasks.
    {
        trpl::run(async {
            // First task.
            let fut1 = async {
                for i in 1..10 {
                    println!("{i} from the first task.");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            // Second task.
            let fut2 = async {
                for i in 1..5 {
                    println!("{i} from the second task.");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            // Run both futures and wait for them to finish.
            trpl::join(fut1, fut2).await;
        });
    }

    println!();
    
    // Sending messages between 2 tasks.
    {
        // Create the async channel.
        let (tx, mut rx) = trpl::channel();

        // Create the transmit task.
        let tx_fut = async move { // move ownership of the tx in here.
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            // Send the messages to the receive task w/ a slight delay.
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }

            // tx is dropped here. That'll let the rx.recv() - and thus the receive task - finish.
        };

        // Create the receive task.
        let rx_fut = async {
            // Wait for messages to be sent from the receive task, and print them as they come in.
            while let Some(message) = rx.recv().await { // The loop will end when tx is dropped in the transmit task.
                println!("received '{message}'");
            }
        };

        trpl::run(async {
            // Start the transmit and receive tasks, and wait for both to finish.
            trpl::join(tx_fut, rx_fut).await;
        });
    }

    println!();

    // Sending messages between multiple tasks.
    {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone(); // Make a copy of the tx so another task can send to the rx.

        // First transmit task.
        let tx_fut = async move { // move tx in so it gets dropped when the task is done.
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Second transmit task.
        let tx1_fut = async move { // move tx1 in so it gets dropped when the task is done.
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // Receive task.
        let rx_fut = async {
            while let Some(message) = rx.recv().await { // Stop receiving when both tx and tx1 are dropped.
                println!("received '{message}'");
            }
        };

        // Run both transmit tasks and receive task, wait for them to finish.
        trpl::run(async {
            trpl::join3(tx_fut, tx1_fut, rx_fut).await;
        });
    }
}
