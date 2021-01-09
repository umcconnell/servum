use crate::http::HTTPStatus;
use std::{fmt, io, str};

/// A struct representing an HTTP response.
///
/// This struct is used to create HTTP responses and easily convert them into
/// bytes to send over the TCP connection. It used by the [`handle_connection`]
/// function as the universal return type for any response.
///
/// HTTPResponse supports conversion from [`io::Error`] and [`HTTPStatus`].
///
/// # Example
///
/// ```rust
/// # use servum::http::{HTTPResponse, HTTPStatus};
/// use std::io;
///
/// let resp = HTTPResponse::new(
///     HTTPStatus::from(200),
///     Some("text/plain"),
///     io::Result::Ok(b"Hello World".to_vec()),
/// );
///
/// assert_eq!(resp.body, b"Hello World");
/// assert_eq!(resp.status.to_string(), "HTTP/1.1 200 OK");
/// assert_eq!(resp.mime.unwrap(), "text/plain");
/// ```
///
/// A more advanced example, including conversion from [`std::io::Error`]. Note
/// that this can be used in conjuntion with the `?` operator to conveniently
/// return `HTTPResponse` from IO errors.
///
/// When converting from [`std::io::Error`], the error will be converted to a
/// [`HTTPStatus`]. This status' HTML representation will be used as the
/// response body.
///
/// ```rust
/// # use servum::http::{HTTPResponse};
/// use std::io;
///
/// let err = io::Error::new(io::ErrorKind::NotFound, "");
/// let resp = HTTPResponse::from(err);
///
/// assert_eq!(resp.status.to_string(), "HTTP/1.1 404 Not Found");
/// assert_eq!(resp.mime.unwrap(), "text/html");
/// ```
///
/// [`handle_connection`]: crate::http::handle_connection
#[derive(Debug)]
pub struct HTTPResponse<'a> {
    pub status: HTTPStatus<'a>,
    pub mime: Option<&'a str>,
    pub body: Vec<u8>,
}

impl<'a> HTTPResponse<'a> {
    /// Create a new HTTPResponse from an [`HTTPStatus`], an optional MIME type
    /// and an [`io::Result`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use servum::http::{HTTPResponse, HTTPStatus};
    /// use std::io;
    ///
    /// let err = io::Error::new(io::ErrorKind::NotFound, "");
    /// let status = HTTPResponse::new(HTTPStatus::from(&err), None, Err(err));
    ///
    /// assert_eq!(status.mime.unwrap(), "text/html");
    /// assert_eq!(status.status.to_string(), "HTTP/1.1 404 Not Found");
    /// ```
    pub fn new(
        status: HTTPStatus<'a>,
        mime: Option<&'a str>,
        body: io::Result<Vec<u8>>,
    ) -> Self {
        HTTPResponse {
            mime: match body {
                Ok(_) => mime,
                Err(_) => Some("text/html"),
            },
            body: body.unwrap_or_else(|_| status.to_html().into_bytes()),
            status,
        }
    }

    /// Generate a HTTP header by from the response and return it as bytes.
    ///
    /// This function uses the MIME type, the [`HTTPStatus`] and the length of
    /// the body to generate a HTTP header with the following fields:
    ///
    /// - HTTP status
    /// - Content-Length
    /// - Content-Type (optional)
    /// - Connection: close
    ///
    /// # Example
    ///
    /// ```rust
    /// # use servum::http::{HTTPResponse, HTTPStatus};
    /// let header = HTTPResponse::from(HTTPStatus::from(200)).header();
    /// let header_str = std::str::from_utf8(&header).unwrap();
    ///
    /// assert!(header_str.starts_with("HTTP/1.1 200 OK"));
    /// assert!(header_str.ends_with("Connection: close\r\n\r\n"));
    /// ```
    pub fn header(&self) -> Vec<u8> {
        // Sure about "Connection: close"?
        format!(
            "{status}\r\nContent-Length: {len}\r\n{mime}Connection: close\r\n\r\n",
            status = self.status,
            len = self.body.len(),
            mime = match self.mime {
                Some(t) => String::from("Content-Type: ") + t + "\r\n",
                None => String::from(""),
            },
        )
        .into_bytes()
    }

    /// Turn the HTTPResponse into a vector of bytes by consuming the response.
    ///
    /// This function internally calls the [`HTTPResponse::header`] method and
    /// chains it to the response body before returning it.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use servum::http::{HTTPResponse, HTTPStatus};
    /// let resp = HTTPResponse::from(HTTPStatus::from(404)).into_bytes();
    /// let resp_str = std::str::from_utf8(&resp).unwrap();
    ///
    /// assert!(resp_str.starts_with("HTTP/1.1 404 Not Found"));
    /// assert!(resp_str.ends_with("<h1>404</h1><p>Not Found</p></body></html>\n"));
    /// ```
    pub fn into_bytes(self) -> Vec<u8> {
        self.header()
            .into_iter()
            .chain(self.body.into_iter())
            .collect()
    }
}

// For debugging purposes
impl fmt::Display for HTTPResponse<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            std::str::from_utf8(&self.header()).unwrap(),
            str::from_utf8(&self.body).unwrap()
        )
    }
}

impl From<io::Error> for HTTPResponse<'_> {
    fn from(error: io::Error) -> Self {
        Self::from(HTTPStatus::from(&error))
    }
}

impl<'a> From<HTTPStatus<'a>> for HTTPResponse<'a> {
    fn from(status: HTTPStatus<'a>) -> Self {
        Self {
            body: status.to_html().into_bytes(),
            status,
            mime: Some("text/html"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{io, HTTPResponse, HTTPStatus};

    #[test]
    fn httpresponse() {
        let res = HTTPResponse::new(
            HTTPStatus::from(404),
            None,
            Err(io::Error::new(io::ErrorKind::NotFound, "")),
        );
        let body_str = std::str::from_utf8(&res.body).unwrap();

        // Status
        assert_eq!(res.status.to_string(), "HTTP/1.1 404 Not Found");
        assert_eq!(res.status.comment, None);

        assert_eq!(res.mime.unwrap(), "text/html");

        assert!(body_str.find("<h1>404</h1>").is_some());
        assert!(body_str.find("<p>Not Found</p>").is_some());
    }

    #[test]
    fn from_io_err() {
        let err =
            io::Error::new(io::ErrorKind::Other, "Some unknown error occurred");
        let res = HTTPResponse::from(err);
        let body_str = std::str::from_utf8(&res.body).unwrap();

        assert_eq!(
            res.status.to_string(),
            "HTTP/1.1 500 Internal Server Error"
        );
        assert_eq!(res.status.comment.unwrap(), "Some unknown error occurred");

        assert_eq!(res.mime.unwrap(), "text/html");

        assert!(body_str.find("<h1>500</h1>").is_some());
        assert!(body_str.find("<p>Internal Server Error</p>").is_some());
        assert!(body_str
            .find("<p>Some unknown error occurred</p>")
            .is_some());
    }

    #[test]
    fn from_httpstatus() {
        let res = HTTPResponse::from(HTTPStatus::from(501));
        let body_str = std::str::from_utf8(&res.body).unwrap();

        assert_eq!(res.status.to_string(), "HTTP/1.1 501 Not Implemented");
        assert_eq!(res.status.comment, None);

        assert_eq!(res.mime.unwrap(), "text/html");

        assert!(body_str.find("<h1>501</h1>").is_some());
        assert!(body_str.find("<p>Not Implemented</p>").is_some());
    }
}
