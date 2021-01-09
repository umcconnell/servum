use std::sync::{mpsc, Arc, Mutex};

use crate::multiprocessing::message::Message;
use crate::multiprocessing::worker::Worker;

/// ThreadPool for multi-thread computations.
///
/// Code from the Rust Book Chapter 20:
/// [`https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html`]
///
/// [`https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html`]: https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Execute a job closure
    ///
    /// Notify the threadpool a new job is pending using the [`Message::NewJob`]
    /// enum. The job will be picked up and processed by the next waiting
    /// worker.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use servum::threadpool::ThreadPool;
    /// let pool = ThreadPool::new(2);
    ///
    /// for i in (0..=3).rev() {
    ///     pool.execute(move || {
    ///         println!("{}", i);
    ///     })
    /// }
    ///
    /// println!("Liftoff!");
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
