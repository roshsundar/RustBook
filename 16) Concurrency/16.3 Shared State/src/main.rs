/* Summary:
Another way to handle concurrency is for multiple threads to share access to the same data.

Mutexes allow data to be accessed by only one thread at a time.
*/

use std::{sync::{Arc, Mutex}, thread};

fn main() {
    /*
    Mutexes allow data to be accessed by only one thread at a time.

    A thread must aquire the mutex's lock in order to use its data. The lock is what allows for exclusive access.
    When the thread is done, it must unlock the mutex so other threads can aquire the lock.
    */

    // Create a Mutex.
    {
        // Create a mutex around 5.
        let m = Mutex::new(5);

        {
            // lock() blocks this thread until it aquires the lock, held by a MutexGuard.
            let mut num = m.lock().unwrap();

            // MutexGuard can be treated as a mutable reference to the data inside the mutex.
            *num = 6;

            // The MutexGuard goes out of scope here, releasing the lock.
        }

        println!("m = {m:?}");
    }

    println!();

    // Use a mutex in multiple threads w/ Arc<T>.
    {
        // Arc<T> is a thread-safe version of Rc<T> which is used the same way.

        // Wrap the mutex in an Arc<T> so that each thread can share it.
        let counter =  Arc::new(Mutex::new(0));

        let mut handles = vec![];

        // Create 10 threads which each increment the counter val once.
        for _ in 0..10 {
            // Each thread needs to have shared ownership of the mutex.
            // Cloning the Arc each time allows each thread access to the mutex to sieze the lock.
            let counter = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap(); // Wait until the mutex lock is available.

                *num += 1;
            });

            handles.push(handle);
        }

        // Wait for all 10 threads to finish before moving on.
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(10, *counter.lock().unwrap());
    }

    /*
    Like RefCell<T>, Mutex<T> provides interior mutability. Just like an Rc<RefCell<T>> allows an Rc to mutate its wrapped data,
    Arc<Mutex<T>> allows Arc to mutate its wrapped data.
    */

    /*
    Mutexes come with the risk of deadlocks.
    These occur when some task requires 2 locks, each having been aquired by 2 separate threads, causing them to wait on each other forever.
    */
}
