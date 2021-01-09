use crate::http::html::html_doc;
use std::{fmt, io};

/// A simple struct representing HTTP status responses.
///
/// This struct is used to create HTTP response messages and easily convert them
/// to HTML documents to display them to the end-user. It also supports
/// conversion from [`std::io::Result`], [`std::io::Error`] and [`usize`].
///
/// # Example
///
/// ```rust
/// # use servum::http::HTTPStatus;
/// let status = HTTPStatus::new(200, "OK", None);
///
/// assert_eq!(status.to_string(), "HTTP/1.1 200 OK");
/// ```
///
/// Another example, using conversion from [`usize`]:
///
/// ```rust
/// # use servum::http::HTTPStatus;
/// let status = HTTPStatus::from(500);
///
/// assert_eq!(status.to_string(), "HTTP/1.1 500 Internal Server Error");
/// ```
///
/// A more advanced example , including conversion from [`std::io::Result`].
/// Note that this can be used in conjuntion with the `?` operator to
/// conveniently return `HTTPStatus` errors.
///
/// ```rust
/// # use servum::http::HTTPStatus;
/// use std::io;
///
/// let res = io::Error::new(io::ErrorKind::PermissionDenied, "Cannot read file!");
/// let status = HTTPStatus::from(&res);
///
/// assert_eq!(status.to_string(), "HTTP/1.1 403 Forbidden");
/// assert_eq!(status.comment, Some(String::from("Cannot read file!")));
/// ```
#[derive(Debug)]
pub struct HTTPStatus<'a> {
    pub code: usize,
    pub msg: &'a str,
    pub comment: Option<String>,
}

impl<'a> HTTPStatus<'a> {
    /// Create a new instance of HTTPStatus with the given code, message and
    /// optional comment
    ///
    /// # Example
    ///
    /// ```rust
    /// # use servum::http::HTTPStatus;
    /// let status = HTTPStatus::new(404, "Not Found", None);
    ///
    /// assert_eq!(status.to_string(), "HTTP/1.1 404 Not Found");
    /// ```
    pub fn new(code: usize, msg: &'a str, comment: Option<String>) -> Self {
        HTTPStatus { code, msg, comment }
    }

    /// Create a HTML document from the status instance by calling [`html_doc`]
    ///
    /// The status code is used as page title and lead. The optional comment is
    /// turned into a paragraph below the lead.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use servum::http::HTTPStatus;
    /// let status = HTTPStatus::new(
    ///     403,
    ///     "Forbidden",
    ///     Some(String::from("Directory traversal is forbidden!")),
    /// );
    /// let doc = status.to_html();
    ///
    /// assert!(
    ///     doc.contains("<h1>403</h1><p>Forbidden</p><p>Directory traversal is forbidden!</p>")
    /// );
    /// ```
    pub fn to_html(&self) -> String {
        html_doc(
            self.code,
            self.code,
            format!(
                "{msg}{comment}",
                msg = self.msg,
                comment = match &self.comment {
                    Some(c) => String::from("</p><p>") + c,
                    None => String::from(""),
                }
            ),
        )
    }
}

impl fmt::Display for HTTPStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP/1.1 {} {}", self.code, self.msg)
    }
}

impl From<usize> for HTTPStatus<'_> {
    fn from(code: usize) -> Self {
        let msg = match code {
            200 => "OK",
            403 => "Forbidden",
            404 => "Not Found",
            501 => "Not Implemented",
            _ => "Internal Server Error",
        };

        Self::new(code, msg, None)
    }
}

impl<T> From<&io::Result<T>> for HTTPStatus<'_> {
    fn from(result: &io::Result<T>) -> Self {
        match result {
            Ok(_) => Self::new(200, "OK", None),
            Err(e) => Self::from(e),
        }
    }
}

impl From<&io::Error> for HTTPStatus<'_> {
    fn from(error: &io::Error) -> Self {
        let comment = error.get_ref().map(|e| e.to_string());

        match error.kind() {
            io::ErrorKind::NotFound => Self::new(404, "Not Found", comment),
            io::ErrorKind::PermissionDenied => {
                Self::new(403, "Forbidden", comment)
            }
            err => {
                dbg!(err);
                Self::new(500, "Internal Server Error", comment)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{io, HTTPStatus};

    #[test]
    fn httpstatus() {
        let status = HTTPStatus::new(
            403,
            "Permission denied",
            Some(String::from("Directory traversal is not allowed!")),
        );

        assert_eq!(status.code, 403);
        assert_eq!(status.msg, "Permission denied");
        assert_eq!(
            status.comment.unwrap(),
            "Directory traversal is not allowed!"
        );
    }

    #[test]
    fn from_io_ok() {
        let res = io::Result::Ok("");
        let status = HTTPStatus::from(&res);

        assert_eq!(status.code, 200);
        assert_eq!(status.msg, "OK");
        assert!(status.comment.is_none());
    }

    #[test]
    fn from_io_err() {
        let res = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let status = HTTPStatus::from(&res);

        assert_eq!(status.code, 404);
        assert_eq!(status.msg, "Not Found");
        assert_eq!(status.comment.unwrap(), "File not found");
    }

    #[test]
    fn from_usize() {
        let status = HTTPStatus::from(200);

        assert_eq!(status.code, 200);
        assert_eq!(status.msg, "OK");
        assert_eq!(status.comment, None);
    }

    #[test]
    fn display() {
        let status = HTTPStatus::from(&io::Result::Ok(""));

        assert_eq!(status.to_string(), "HTTP/1.1 200 OK");
    }
}
