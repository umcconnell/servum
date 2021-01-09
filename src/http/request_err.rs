/// HTTPRequestError representing possible HTTP request validation errors.
///
/// Used by [`HTTPRequest`] to indicate failed parsing attempts.
///
/// [`HTTPRequest`]: crate::http::HTTPRequest
#[derive(Debug)]
pub enum HTTPRequestError {
    NoMethod,
    NoPath,
    Utf8Error,
}

impl std::error::Error for HTTPRequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            HTTPRequestError::NoMethod => None,
            HTTPRequestError::NoPath => None,
            HTTPRequestError::Utf8Error => None,
        }
    }
}

impl std::fmt::Display for HTTPRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            HTTPRequestError::NoMethod => {
                write!(f, "Request does not have an associated HTTP method")
            }
            HTTPRequestError::NoPath => {
                write!(f, "Request does not have an associated request path")
            }
            HTTPRequestError::Utf8Error => {
                write!(f, "Request contains invalid Utf8 characters")
            }
        }
    }
}

impl From<std::str::Utf8Error> for HTTPRequestError {
    fn from(_err: std::str::Utf8Error) -> HTTPRequestError {
        HTTPRequestError::Utf8Error
    }
}
