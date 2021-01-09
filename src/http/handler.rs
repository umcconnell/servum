use crate::http::{self, html_doc, HTTPRequest, HTTPResponse, HTTPStatus};
use crate::{cli::Config, files};
use std::{fs, io, path::Path, sync::Arc};

/// List a directory for a given [`Path`].
///
/// Turn a directory of subdirectories and files into an HTML list. This list is
/// turned into an HTML document using [`html_doc`] for representing directories
/// to the front-end user. Possible errors while read the directory are returned
/// as [`std::io::Error`].
///
/// [`Path`]: std::path::Path
/// [`html_doc`]: crate::http::html_doc
fn list_dir(path: &Path) -> io::Result<Vec<u8>> {
    let mut result = fs::read_dir(path)?
        .filter_map(|f| match f {
            Ok(entry) => Some(files::file::File(entry).to_string()),
            Err(_) => None,
        })
        .collect::<Vec<String>>();

    result.sort_unstable();

    let result = [
        "<a href=\"./../\">&uarr; Parent Directory</a><ul><li>",
        &result.join("</li><li>"),
        "</li></ul>",
    ]
    .concat();

    Ok(html_doc(
        "Directory Listing",
        format!("Listing for {}", path.display()),
        result,
    )
    .into_bytes())
}

/// Handle incoming HTTP requests.
///
/// This function validates and executes incoming HTTP requests by normalizing
/// and sanitizing request paths, respecting user config and returning a
/// [`HTTPResponse`] with the processed file, directory listing or error.
///
/// This function solely handles HTTP `GET` and `HEAD` requests, as these are
/// the only requests types that must be supported by simple static server.
///
/// # Example
///
/// A rather extensive example involving a fair amount of boilerplate code:
///
/// ```rust
/// # use servum::http::{handle_connection, HTTPRequest};
/// use servum::cli::Config;
/// use std::{str, sync::Arc};
///
/// // Standard config with sane defaults
/// let config = Arc::new(Config::default());
///
/// // Body left out for brevity...
/// let buf = b"POST /contact HTTP/1.1";
/// let req = HTTPRequest::new(buf).unwrap();
///
/// let res = handle_connection(&req, config.clone());
/// let res_body = str::from_utf8(&res.body).unwrap();
///
/// // Response status
/// assert_eq!(res.status.to_string(), "HTTP/1.1 501 Not Implemented");
///
/// // MIME
/// assert_eq!(res.mime.unwrap(), "text/html");
///
/// // Response body
/// assert!(res_body.starts_with("<!DOCTYPE html>"));
/// assert!(res_body.find("<title>501</title>").is_some());
/// assert!(res_body.find("<h1>501</h1>").is_some());
/// assert!(res_body
///     .find("<p>Server only supports GET and HEAD requests</p>")
///     .is_some());
/// ```
pub fn handle_connection<'a>(
    req: &HTTPRequest,
    config: Arc<Config>,
) -> HTTPResponse<'a> {
    if !(req.method == "GET" || req.method == "HEAD") {
        return HTTPResponse::from(HTTPStatus::new(
            501,
            "Not Implemented",
            Some(String::from("Server only supports GET and HEAD requests")),
        ));
    }

    let path = req.filepath;
    let req_filename = match path.to_str().unwrap() {
        "/" => Path::new("index.html"),
        _ => match path.starts_with("/") {
            true => path.strip_prefix("/").unwrap(),
            false => path,
        },
    };

    let filename = files::path::process_path(req_filename, &config.base_dir);

    let is_sub = &filename
        .ancestors()
        .find(|a| a == &config.base_dir)
        .is_some();

    if !is_sub {
        return HTTPResponse::from(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Directory traversal is not allowed!",
        ));
    }

    let mut filetype = files::mime::guess_mime_type(&filename);

    let mut contents = match filename.is_dir() {
        false => fs::read(&filename),
        true => {
            filetype = Some("text/html"); // Directory listings or errs are HTML
            match config.list_dir {
                true => http::handler::list_dir(&filename),
                false => {
                    return HTTPResponse::from(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "Directory traversal is not allowed!",
                    ))
                }
            }
        }
    };

    // List directory if /index.html is not found
    if config.list_dir
        && contents.is_err()
        && req_filename.to_str().unwrap() == "index.html"
    {
        contents = http::handler::list_dir(&config.base_dir);
    }

    let status = HTTPStatus::from(&contents);
    HTTPResponse::new(status, filetype, contents)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn listdir_success() {
        let dir_listing = list_dir(Path::new("example/")).unwrap();
        let dir_str = std::str::from_utf8(&dir_listing).unwrap();

        assert!(dir_str.starts_with("<!DOCTYPE html>"));
        assert!(dir_str.find("<h1>Listing for example/</h1>").is_some());
        assert!(dir_str.find("<ul><li>").is_some());
        assert!(dir_str
            .find("<a href=\"./index.html\">index.html</a>")
            .is_some());
        assert!(dir_str.ends_with("</html>\n"));
    }

    #[test]
    fn listdir_err() {
        let dir_listing = list_dir(Path::new("example/i_dont_exist/"));

        assert!(dir_listing.is_err());
        assert!(matches!(
            dir_listing.unwrap_err().kind(),
            io::ErrorKind::NotFound
        ));
    }

    // Helper
    fn simulate_request(buffer: &[u8], config: Option<Config>) -> HTTPResponse {
        let config = config.unwrap_or_else(|| {
            let mut conf = Config::default();

            // Run the examples from the example/ directory
            conf.base_dir = Path::new("example/").canonicalize().unwrap();
            conf
        });

        let config = Arc::new(config);
        let req = HTTPRequest::new(buffer).unwrap();

        handle_connection(&req, config)
    }

    #[test]
    fn request_ok() {
        let res = simulate_request(b"GET /index.html HTTP/1.1", None);

        assert_eq!(res.status.to_string(), "HTTP/1.1 200 OK");
    }

    #[test]
    fn request_invalid_method() {
        let res = simulate_request(b"DELETE /user/post HTTP/1.1", None);

        assert_eq!(res.status.to_string(), "HTTP/1.1 501 Not Implemented");
        assert_eq!(res.mime.unwrap(), "text/html");
        assert!(res.body.len() > 0);
    }

    #[test]
    fn request_implied_index() {
        let mut conf = Config::default();
        conf.base_dir = Path::new("example/").canonicalize().unwrap();

        let res = simulate_request(b"HEAD / HTTP/1.1", None);
        let body = std::str::from_utf8(&res.body).unwrap();

        assert_eq!(res.status.to_string(), "HTTP/1.1 200 OK");
        assert!(body.starts_with("<!DOCTYPE html>"));
        assert!(body.ends_with("</html>\n"));
    }

    #[test]
    fn file_not_found() {
        let res = simulate_request(b"GET /i-dont-exist HTTP/1.1", None);
        let body = std::str::from_utf8(&res.body).unwrap();

        assert_eq!(res.status.to_string(), "HTTP/1.1 404 Not Found");
        assert!(body.find("<h1>404</h1>").is_some());
    }

    #[test]
    fn respect_no_listdir() {
        let mut conf = Config::default();
        conf.list_dir = false;
        conf.base_dir = Path::new("example/").canonicalize().unwrap();

        let res = simulate_request(b"GET /pages HTTP/1.1", Some(conf));
        let body = std::str::from_utf8(&res.body).unwrap();

        assert_eq!(res.status.to_string(), "HTTP/1.1 403 Forbidden");
        assert!(body.find("<h1>403</h1>").is_some());
    }

    #[test]
    fn dir_traversal_forbidden() {
        let res = simulate_request(b"GET /../src/lib.rs HTTP/1.1", None);
        let body = std::str::from_utf8(&res.body).unwrap();

        assert_eq!(res.status.to_string(), "HTTP/1.1 403 Forbidden");
        assert!(body.find("<h1>403</h1>").is_some());
        assert!(body
            .find("<p>Directory traversal is not allowed!</p>")
            .is_some());
    }

    #[test]
    fn listdir_when_no_index_html() {
        let mut conf = Config::default();
        conf.base_dir = Path::new("example/pages").canonicalize().unwrap();

        let res = simulate_request(b"GET / HTTP/1.1", Some(conf));
        let body = std::str::from_utf8(&res.body).unwrap();

        assert_eq!(res.status.to_string(), "HTTP/1.1 200 OK");
        assert!(body.find("Listing for").is_some());
        assert!(body.find("<h1>Listing for").is_some());
    }
}
