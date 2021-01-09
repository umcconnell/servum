use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::multiprocessing::message::Message;

/// ThreadPool Worker
pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new Worker
    ///
    /// The worker receives a (unique) id and a receiver end of
    /// [`std::sync::mpsc`]. New [`Message`]s are read from receiver end and
    /// executed in a seperate thread.
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} got a job; executing", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was terminated", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
