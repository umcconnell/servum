use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Instant;

use servum::cli::{self, tui};
use servum::{http, http::HTTPRequest, multiprocessing::ThreadPool};

fn main() {
    tui::print_logo();

    let config = Arc::new(cli::Config::new());

    tui::print_info();

    let listener =
        TcpListener::bind(format!("{}:{}", &config.address, &config.port))
            .unwrap();
    let pool = ThreadPool::new(config.threads);

    tui::print_config(config.clone());

    if config.verbose {
        tui::print_verbose_header();
    }

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let verbose = config.verbose;
        let conf = config.clone();

        pool.execute(move || {
            let mut buffer = [0; 1024];

            #[allow(clippy::unused_io_amount)]
            stream.read(&mut buffer).unwrap();

            let timer = Instant::now();
            let req = HTTPRequest::new(&buffer);

            if let Ok(req) = req {
                let res = http::handle_connection(&req, conf);

                if verbose {
                    tui::print_verbose_stats(&req, &res, timer);
                }

                match req.method {
                    "HEAD" => stream.write(res.header().as_slice()),
                    _ => stream.write(res.into_bytes().as_slice()),
                }
                .unwrap();
            } else if verbose {
                eprintln!("ERR: Invalid HTTP request: {}", req.unwrap_err());
            }

            stream.flush().unwrap();
        });
    }

    println!("Shutting down");
}
