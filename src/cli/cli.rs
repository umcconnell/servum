use super::err::CliError;
use std::{env, path::PathBuf};

/// Rudimentary argument parsing and user configuration.
///
/// This approach is not very general or particularly robust, but it has the
/// benefit of being lighter than crates such as *clap*.
///
/// Programs should make use of Config's default implementation, available
/// through the [`Default`] trait by calling `Config::default()`.
///
/// Possible configurable options include:
/// - `address`: [`String`] (default: `"127.0.0.1"`)  
///   Server address to bind to. Default is the loopback address 127.0.0.1, i.e
///   localhost.
/// - `base_dir`: [`PathBuf`] (default: current directory)  
///   Base directory to serve files from. Defaults to the current directory
///   ([`env::current_dir`])
/// - `list_dir`: [`bool`] (default: `true`)  
///   Whether or not to list directories. Defaults to yes.
/// - `port`: [`usize`] (default: `8080`)  
///   What port to listen on. Defaults to 8080. Ports, such as port `80` (HTTP)
///   need elevated privileges to bind to.
/// - `threads`: [`usize`] (default: `4`)  
///   Number of threads to use to respond to incoming request in parallel.
///   Value must be greater than 0. Default is 4.
/// - `verbose`: [`bool`] (default: `true`)  
///   Whether or not to be verbose and log stats about incoming requests.
///   Default is true.
///
/// # Example
///
/// ```rust
/// # use servum::cli::Config;
/// let config = Config::default();
///
/// assert_eq!(config.address, "127.0.0.1");
/// assert_eq!(config.port, 8080);
/// // Do some more fancy stuff...
/// ```
pub struct Config {
    pub address: String,
    pub base_dir: PathBuf,
    pub list_dir: bool,
    pub port: usize,
    pub threads: usize,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: String::from("127.0.0.1"),
            port: 8080,
            base_dir: env::current_dir().unwrap(),
            threads: 4,
            verbose: true,
            list_dir: true,
        }
    }
}

impl Config {
    /// Create a new user configuration from environment arguments.
    ///
    /// Read and parse environment arguments from the user and collect them
    /// into a [`Config`] struct.
    ///
    /// If errors are encountered or the help menu is requested, the current
    /// process will be exit with code `1` (error) or `0` (help) accordingly.
    pub fn new() -> Config {
        let mut args: Vec<String> = env::args().skip(1).collect();
        let mut conf = Config::default();

        // Insert base-dir if first argument is <BASE_DIR>
        if !args.is_empty() && !args[0].starts_with('-') {
            args.insert(0, String::from("--base-dir"));
        }

        let showed_help = Config::parse_args(&args, &mut conf).unwrap_or_else(|e| {
            eprintln!(
                "Error while parsing arguments: {}\nUse --help for more information on available arguments",
                e
            );
            std::process::exit(1);
        });

        if showed_help {
            std::process::exit(0);
        }

        conf
    }

    /// Parse environment arguments and update a user [`Config`] instance
    ///
    /// This function may return an error when it encounters an unknown
    /// argument or an invalid value.
    ///
    /// If parsing succeeds, the function will return a boolean indicating
    /// whether the help menu has been shown (`true`), indicating that the
    /// program may exit normally (exit code `0`), or not, meaning the program
    /// can continue.
    fn parse_args<'a>(
        args: &'a [String],
        conf: &mut Config,
    ) -> Result<bool, CliError<'a>> {
        let mut it = args.iter();

        while let Some(el) = it.next() {
            let mut el = el.as_str();

            // Parse flags first
            match el {
                "-q" | "--quiet" => {
                    conf.verbose = false;
                    continue;
                }
                "--no-list-dir" => {
                    conf.list_dir = false;
                    continue;
                }
                "-h" => {
                    println!("{}", Config::help_short());
                    return Ok(true);
                }
                "--help" => {
                    println!("{}", Config::help_long());
                    return Ok(true);
                }
                _ => (),
            }

            let val = match el.contains('=') {
                false => it.next().map(|v| v.as_str()),
                true => {
                    let mut temp = el.split('=');
                    el = temp.next().unwrap();
                    temp.next()
                }
            };
            let val = val.ok_or(CliError::MissingVal(el))?;

            match el {
                "--base-dir" => {
                    conf.base_dir = PathBuf::from(val).canonicalize()?
                }
                "-a" | "--address" => conf.address = val.to_string(),
                "-p" | "--port" => {
                    conf.port = val
                        .parse::<usize>()
                        .map_err(|_| CliError::InvalidVal("--port", val))?
                }
                "-t" | "--threads" => {
                    conf.threads = val
                        .parse::<usize>()
                        .map_err(|_| CliError::InvalidVal("--threads", val))?
                }
                arg => return Err(CliError::InvalidArg(arg)),
            }
        }

        Ok(false)
    }

    /// Return the help menu header common to all help menus.
    fn help_header() -> &'static str {
        "servum 1.0.0
Ulysse McConnell <ulysse.mcconnell+dev@protonmail.com>

servum is a simple, bare-bones and fast static web server for local
(web-)development. HTTP only.

Project home page: https://github.com/umcconnell/servum


USAGE:
    servum 
    servum [BASE_DIR]
    servum [BASE_DIR] [OPTIONS]"
    }

    /// Return the help menu in its verbose form.
    pub fn help_long() -> String {
        [
            Config::help_header(),
            "

ARGS:
    <BASE_DIR>
            Base directory to serve content from. All sub-directories and files
            will be served. Default is the current directory.

OPTIONS:
    -a, --address <STRING>:
            Address to listen on. Default is the loopback address 127.0.0.1,
            i.e localhost.
    -p, --port <NUM>:
            Port to listen on. Note that some ports, such as port 80 (HTTP)
            require elevated privileges to bind to and may already be in use.
            Default is 8080.
    -t, --threads <NUM>:
            Number of parallel threads to handle request with. Must be at least
            1. Default is 4.
    -q, --quiet:
            Don't be verbose and stop printing information about incoming
            requests.
        --no-list-dir:
            Don't list directories and prevent directory traversals by returning
            \"403 Permission Denied\" responses when attempting to access a
            directory.
    -h, --help:
            Show this help. Use -h for a quick summary of available commands and
            --help for a more detailed view.
",
        ]
        .concat()
    }

    /// Return the help menu in its short form.
    pub fn help_short() -> String {
        [
            Config::help_header(),
            "

ARGS:
    <BASE_DIR>    Optional directory to serve content from.

OPTIONS:
    -a, --address <STRING>:     Address to listen on. Default is 127.0.0.1
    -p, --port <NUM>:           Port to listen on. Default is 8080
    -t, --threads <NUM>:        Number of threads. Default is 4.
    -q, --quiet:                Don't be verbose.
        --no-list-dir:          Don't list directories.
    -h, --help:                 Show this help. Use --help for more details.
",
        ]
        .concat()
    }
}
