use crate::{
    cli::Config,
    http::{HTTPRequest, HTTPResponse},
};
use std::{sync::Arc, time::Instant};

/// Print ASCII Art of `servum` to the console.
pub fn print_logo() {
    println!(
        r"______________________   _____  ________ ___ 
__  ___/  _ \_  ___/_ | / /  / / /_  __ `__ \
_(__  )/  __/  /   __ |/ // /_/ /_  / / / / /
/____/ \___//_/    _____/ \__,_/ /_/ /_/ /_/         
"
    );
}

/// Print general information about help and quitting to the console.
pub fn print_info() {
    println!("To exit, just press Ctrl/Cmd+C at any time");
    println!("To get help, pass --help");
    println!("");
}

/// Print information about the current user [`Config`] to the console.
pub fn print_config(config: Arc<Config>) {
    println!("Serving {}", config.base_dir.display());
    println!(
        "Server listening at http://{}:{}",
        config.address, config.port
    );
    println!("");
}

/// Print table header of verbose output to the console.
pub fn print_verbose_header() {
    println!(
        "[{req_method: <6} {req_path: <32}] -> \t{res_code: <6} {res_msg: <24} {time}μs",
        req_method = "Method",
        req_path = "/path/to/file/...",
        res_code = "Code",
        res_msg = "Message",
        time = "Time in "
    );
    println!("{}", "-".repeat(90));
}

/// Print verbose stats about a request to the console.
pub fn print_verbose_stats(
    req: &HTTPRequest,
    res: &HTTPResponse,
    timer: Instant,
) {
    println!(
        "[{req_method: <6} {req_path: <33}] -> \t{res_code: <6} {res_msg: <24} {time: <4}μs",
        req_method = req.method,
        req_path = {
            let mut path = req.filepath.display().to_string();
            path.truncate(32);
            path
        },
        res_code = res.status.code,
        res_msg = res.status.msg,
        time = timer.elapsed().as_micros(),
    );
}
