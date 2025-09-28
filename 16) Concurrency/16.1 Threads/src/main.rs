/* Summary:
Threads are independent parts of a program that run simultaneously, generally used to improve performance.

However, they add complexity. Since they can run simultaneously, there's no guarantee of an order for
which parts of the code run.

This can lead to issues, such as:
    • Race conditions - When threads access data in an inconsistent order
    • Deadlocks - When 2 threads wait on each other, blocking both from executing
    • Bugs that happen only in certain situations and are hard to reproduce and fix reliably
*/

use std::{thread, time::Duration};

fn main() {
    // Creating a new thread with spawn()
    {
        // To create a new thread, use thread::spawn() and pass it a closure

        thread::spawn(|| {
            for i in 1..10 {
                println!("{i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1)); // thread::sleep() delays execution for a duration
            }
        });

        for i in 1..5 {
            println!("{i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        //$ Note that all spawned threads are killed when the main thread is finished.
        //$ Expect to see different output every time, since the operating system scheduling is unpredictable
    }

    thread::sleep(Duration::from_millis(5));
    println!();
    println!("--------");
    println!();

    // Wait for threads to finish using join() handles
    {
        // Save the thread into a handle
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("{i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1)); // thread::sleep() delays execution for a duration
            }
        });

        for i in 1..5 {
            println!("{i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
        
        // join() waits for the handle thread to finish.
        // It blocks the thread currently running until the handle thread finishes.
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(5));
    println!();
    println!("--------");
    println!();

    // Where join() is called matters
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("{i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1)); // thread::sleep() delays execution for a duration
            }
        });

        // The main thread will wait for this to finish...
        handle.join().unwrap();

        //... then this will run
        for i in 1..5 {
            println!("{i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }

    thread::sleep(Duration::from_millis(5));
    println!();
    println!("--------");
    println!();

    // `move` and threads
    {
        // `move` is used with threads to take ownership of values from the environment into the thread.
        // This is done so that references are valid. Rust can't know thread execution order ahead of time.

        let v = vec![1, 2, 3];

        // i.e. It's possible the main thread drops v before the thread tries to reference it.
        // So, v needs to be moved into the thread to guarantee its availability.
        
        let handle = thread::spawn(move || {
            println!("Here's a vector: {v:#?}");
        });

        handle.join().unwrap();
    }
}
