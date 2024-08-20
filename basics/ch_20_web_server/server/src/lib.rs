use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,

    // we are using a channel to send the Job to do
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Creates a new channel
        let (sender, receiver) = mpsc::channel();

        // Here we are wrapping the receiver in an Arc, so that it's safe
        // to pass through multiple threads, and a Mutex, so that it's safe
        // to access because it locks and unlocks the usage
        let receiver = Arc::new(Mutex::new(receiver));

        // Creates a vector with a fixed capacity
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(
                Worker::new(id, Arc::clone(&receiver))
            );
        }

        ThreadPool { 
            workers: workers, 
            sender: Some(sender),
        }
    }

    /// Executes a function in a thread.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Here we are placing a job in a box
        let job = Box::new(f);

        // The will be sent to 
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Here we implement the Drop trait for ThreadPool so that we 
// can close the program gracefully, instead of going down bad.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // This closes the channel if it's open, so no more requests can
        // be passed if the program it's closed. 
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job. Executing...");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} shutting down.");
                    break;
                }
            }

        });
        Worker { 
            id: id, 
            thread: Some(thread),
        }
    }
}
