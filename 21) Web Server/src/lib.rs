use std::{sync::{Arc, Mutex, mpsc}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        // Each worker will share ownership of the receiver. So it needs to be wrapped in a mutex and arc
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { 
            workers,
            sender: Some(sender),
        }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Dropping the sender closes the channel.
        // All the .recv() calls in the workers will return an error.
        drop(self.sender.take());

        for worker in self.workers.drain(..) { // drain(..) removes all the workers from the vec and returns an iterator. 
            println!("Shutting down worker {}", worker.id);

            // Each worker needs to finish its current job before closing.
            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                // Wait for the mutex to be available. Then wait for the receiver to get a message.
                let message = receiver.lock().unwrap().recv();

                // The mutex lock is restored here.

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job(); // Each worker will execute job() simultaneously.
                    }

                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>; // Job is a trait object for the closure that goes into ThreadPool.execute() 