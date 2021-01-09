//! ThreadPool implementation from the Rust Book
//!
//! See: [`https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html`]
//!
//! [`https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html`]:
//! https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html
mod message;
mod threadpool;
mod worker;

pub use message::Message;
pub use threadpool::ThreadPool;
pub use worker::Worker;
