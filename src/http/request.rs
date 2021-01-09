use crate::http::HTTPRequestError;
use std::path::Path;
use std::{fmt, str};

/// A struct used to represent an HTTP request.
///
/// This struct is used to pass HTTP requests to the according request handling
/// functions (mainly [`handle_connection`]). It extracts information from a
/// [`std::net::TcpStream`] buffer. Basic validation is performed, with possible
/// errors being returned as [`HTTPRequestError`] variant.
///
/// # Example
///
/// ```rust
/// # use servum::http::{HTTPRequest};
/// let buffer = b"GET / HTTP/1.1";
/// let req = HTTPRequest::new(buffer).unwrap();
///
/// assert_eq!(req.method, "GET");
/// assert_eq!(req.filepath.to_str().unwrap(), "/");
/// ```
///
/// [`handle_connection`]: crate::http::handle_connection
#[derive(Debug)]
pub struct HTTPRequest<'a> {
    pub method: &'a str,
    pub filepath: &'a Path,
}

impl<'a> HTTPRequest<'a> {
    /// Create a new HTTPRequest from a [`std::net::TcpStream`] buffer.
    ///
    /// If basic validation fails (method or filepath not present, request not
    /// Utf8), a [`HTTPRequestError`] will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use servum::http::{HTTPRequest};
    /// let buffer = b"HEAD /static/logo.svg HTTP/1.1";
    /// let req = HTTPRequest::new(buffer).unwrap();
    ///
    /// assert_eq!(req.method, "HEAD");
    /// assert_eq!(req.filepath.to_str().unwrap(), "/static/logo.svg");
    /// ```
    ///
    /// Another example, in which the request is empty and thus cannot be
    /// processed:
    ///
    /// ```rust
    /// # use servum::http::{HTTPRequest, HTTPRequestError};
    /// let buffer = b"";
    /// let req = HTTPRequest::new(buffer);
    ///
    /// assert!(req.is_err());
    /// assert!(matches!(req.unwrap_err(), HTTPRequestError::NoMethod));
    /// ```
    pub fn new(buffer: &'a [u8]) -> Result<Self, HTTPRequestError> {
        let mut first_line = str::from_utf8(buffer)?.split_ascii_whitespace();

        let method = first_line.next().ok_or(HTTPRequestError::NoMethod)?;
        let filepath =
            Path::new(first_line.next().ok_or(HTTPRequestError::NoPath)?);

        if first_line.next().is_none() {
            return Err(HTTPRequestError::NoPath);
        }

        Ok(Self { method, filepath })
    }
}

impl fmt::Display for HTTPRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} HTTP/1.1", self.method, self.filepath.display())
    }
}

#[cfg(test)]
mod test {
    use super::{HTTPRequest, HTTPRequestError};

    #[test]
    fn from_buf() {
        let req = HTTPRequest::new(b"HEAD /index.html HTTP/1.1").unwrap();

        assert_eq!(req.method, "HEAD");
        assert_eq!(req.filepath.to_str().unwrap(), "/index.html");
    }

    #[test]
    fn no_method() {
        let req = HTTPRequest::new(b"");

        assert!(req.is_err());
        assert!(matches!(req.unwrap_err(), HTTPRequestError::NoMethod));
    }

    #[test]
    fn no_path() {
        let req = HTTPRequest::new(b"HEAD HTTP/1.1");

        assert!(req.is_err());
        assert!(matches!(req.unwrap_err(), HTTPRequestError::NoPath));
    }

    #[test]
    fn utf8_error() {
        // Invalid 💖, should be [240, 159, 146, 150]
        let buf = &[0, 159, 146, 150];
        let req = HTTPRequest::new(buf);

        assert!(req.is_err());
        assert!(matches!(req.unwrap_err(), HTTPRequestError::Utf8Error));
    }
}
