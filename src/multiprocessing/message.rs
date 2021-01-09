type Job = Box<dyn FnOnce() + Send + 'static>;

/// Message to a [`threadpool::Worker`]
///
/// [`threadpool::Worker`]: crate::threadpool::Worker
pub enum Message {
    NewJob(Job),
    Terminate,
}
