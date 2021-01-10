![Test](https://github.com/umcconnell/servum/workflows/test/badge.svg)
![Lint](https://github.com/umcconnell/servum/workflows/lint/badge.svg)
![Docs](https://github.com/umcconnell/servum/workflows/docs/badge.svg)

# servum

Servum is a simple static server. Not more. Not less.

It is fast[\*](#speed) and comes with no external dependencies. Servum also
supports multi-threaded, parallel processing of incoming requests.

## Quickstart

Install servum from the
[Releases Tab](https://github.com/umcconnell/servum/releases).

Then run `servum` inside the directory you want to serve :rocket:.

## Table of Contents

-   [Installation](#installation)
    -   [Linux](#linux)
    -   [Cargo](#cargo)
-   [Usage](#usage)
-   [Speed](#speed)
-   [Developing](#developing)
    -   [Prerequisites](#prerequisites)
    -   [Initial setup](#initial-setup)
    -   [Testing](#testing)
    -   [Docs](#docs)
-   [Contributing](#contributing)
-   [Versioning](#versioning)
-   [Authors](#authors)
-   [License](#license)

## Installation

Pre-built binaries for Windows, Mac and Linux can be found under the
[Releases Tab](https://github.com/umcconnell/servum/releases).

### Linux

To install the binary systemwide on linux, make sure it is executable before
moving it to `usr/bin`:

```bash
# In your Downloads folder:
chmod a+x ./servum
sudo mv ./servum /usr/bin/
```

### Cargo

If you're a **Rust programmer**, servum can be installed _via_ cargo.

```bash
cargo install --git https://github.com/umcconnell/servum.git --branch main --bin cargo-servum
```

## Usage

To get started, just run `servum`, or `cargo servum` if you've installed using
cargo, in the directory you want to serve.

```
servum [optional: BASE_DIR] [optional: OPTIONS]
```

servum takes an optional path to the base directory, i.e. the root directory
you want to serve. This defaults to the current directory.

Additional configuration, such as the port, address, number of threads to use,
etc., is possible using further CLI arguments. Run `servum --help` to get help.

## Speed

The speed test consisted of serving the [example folder](example/) on a laptop
(Intel i7-9750H) using a single thread. Performance was measured using
[wrk](https://github.com/wg/wrk) with 6 threads and 400 connections for 60s:

```bash
wrk -t6 -c400 -d60s http://localhost:8080/index.html
```

| Server                                                           | Notes                 | Requests/sec | Transfer/sec | Errors      |
| ---------------------------------------------------------------- | --------------------- | ------------ | ------------ | ----------- |
| 1. **servum**                                                    | 1 thread              | 41176.04     | 42.21MB      | /           |
| 2. [nginx](http://nginx.org/)                                    | from docker container | 27036.48     | 31.69MB      | /           |
| 3. [serve](https://github.com/vercel/serve)                      | node                  | 16683.90     | 2.69MB       | /           |
| 4. [python3](https://docs.python.org/3/library/http.server.html) | module `http.server`  | 1135.15      | 1.27MB       | 19 timeouts |

## Developing

Servum is open source! Contributions, improvements, bug fixes, etc. are
very welcomed.

### Prerequisites

Before you get started, make sure you've got rust and cargo installed on your
machine. If not, installation via rustup is quick and easy. See the rust website
for installation instructions: https://www.rust-lang.org/tools/install

### Initial setup

First, clone the repo with git:

```bash
git clone https://github.com/umcconnell/servum.git
cd servum
```

Then, you can run servum using cargo:

```bash
cargo run [BASE_DIR] [OPTIONS]
```

... or build a binary:

```bash
cargo build --release
```

Check out the [examples](examples/) folder for an example static website.

Happy coding!

### Docs

The developer docs are available online:
https://umcconnell.github.io/servum/servum/index.html

Docs can also be built and viewed locally using:

```bash
cargo doc --document-private-items --open
```

### Testing

Tests can be run using:

```bash
cargo test
```

When adding new features, make sure to add tests to your code.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) and
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details on our code of conduct, and
the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available,
see the [tags on this repository](https://github.com/umcconnell/servum/tags).

## Authors

Ulysse McConnell - [umcconnell](https://github.com/umcconnell/)

See also the list of
[contributors](https://github.com/umcconnell/servum/contributors) who
participated in this project.

## License

servum is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE.md) and [LICENSE-MIT](LICENSE-MIT.md) for
details.

## Acknowledgments

A huge thanks goes to the open and welcoming rust community and their great
documentation effort. This project is based off Chapter 12 of the Rust Book
"Building a Multithreaded Web Server"

-   Chapter 12: https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html
-   https://doc.rust-lang.org/book/
-   https://blog.burntsushi.net/rust-error-handling/
-   https://www.reddit.com/r/rust/comments/gj8inf/rust_structuring_and_handling_errors_in_2020/
-   https://alican.codes/rust-github-actions/
