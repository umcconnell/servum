//! Servum is a simple static server. Not more. Not less.
//!
//! It is fast and comes with no added dependencies. Servum also supports
//! multi-threaded, parallel processing of incoming requests.
//!
//! These are the developer docs for servum, documenting internal methods and
//! functionality. If you are just looking for a quick start and installation
//! instructions, head over to the [`project's website`] or read on.
//!  
//! # Installation
//!
//! Detailed installation instructions and prebuilt binaries for Windows, Mac
//! and Linux can be found on the project's website:
//! [`https://github.com/umcconnell/servum`]
//!
//! ### Using cargo
//!
//! servum can be installed using cargo:
//!
//! ```bash
//! cargo install --git https://github.com/umcconnell/servum.git
//! ```
//!
//! # Usage
//!
//! To serve a directory, run `servum`, or `cargo servum` if you've installed
//! with cargo, inside of the directory.
//!
//! To serve a different directory, just specify it as the first argument.
//!
//! All options and configurations are available in the help menu, accessible
//! via:
//!
//! ```bash
//! servum --help
//! ```
//!
//! # Contributing
//!
//! Servum is open source! Contributions, improvements, bug fixes, etc. are
//! very welcomed.
//!
//! To contribute, make sure you've got rust and cargo installed on your
//! machine. If not, installation via rustup is quick and easy. See the rust
//! website for installation instructions:
//! [`https://www.rust-lang.org/tools/install`]
//!
//! Then, clone the git repo:
//!
//! ```bash
//! git clone https://github.com/umcconnell/servum.git
//! cd servum
//! ```
//!
//! Happy coding!
//!
//! [`project's website`]: https://github.com/umcconnell/servum
//! [`https://github.com/umcconnell/servum`]: https://github.com/umcconnell/servum
//! [`https://www.rust-lang.org/tools/install`]: https://www.rust-lang.org/tools/install
pub mod cli;
pub mod files;
pub mod http;
pub mod multiprocessing;
