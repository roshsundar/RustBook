use std::{sync::mpsc, thread, time::Duration};

/* Summary:
Message passing is when threads communicate by sending messages containing data,
rather than sharing memory.

Rust uses channels to send data from one thread to another.
*/
fn main() {
    // Channel example.
    {
        // Create a new channel. mspc stands for 'multiple producer, single consumer'.
        // Channels can have multiple sending ends, but only one recieving end.
        // tx = transmitter and rx = reciever.
        let (tx, rx) = mpsc::channel();

        // Create a new thread and move ownership of tx into it.
        thread::spawn(move || {
            let val = String::from("hi");
            println!("Sending: '{val}' from spawned thread");

            // send "hi" from the transmitter to receiver.
            tx.send(val).unwrap();
        });

        // Receive "hi" from the tx into the rx.
        let received = rx.recv().unwrap(); // recv() blocks the main thread until it gets a value from tx.
                                                   // use try_recv() if you want to check for data in the channel immediately. 
        println!("Got: '{received}' on the main thread");
    }

    println!();

    // Channels transfer ownership of data.
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap(); // send() passes ownership of the data to the receiver.
            // println!("val is {val}"); // !❌ Err: val is used after it is moved.
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }

    println!();

    // Send multiple messages.
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            // Send each String to the main thread w/ a 1 second delay.
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }

            // tx is dropped, the channel closes.
        });

        // Treat rx as an iterator, print each value as it comes in.
        // The iterator ends when the channel closes.
        for received in rx {
            println!("Got: {received}");
        }
    }

    println!();

    // Multiple transmitters.
    {
        let (tx, rx) = mpsc::channel();


        // clone() tx to create a 2nd transmitter, and send some Strings from a thread
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Send some Strings from a thread with the original transmitter.
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }
}
