//! HTTP utilities
mod handler;
mod html;
mod request;
mod request_err;
mod response;
mod status;

pub use handler::handle_connection;
pub use html::html_doc;
pub use request::HTTPRequest;
pub use request_err::HTTPRequestError;
pub use response::HTTPResponse;
pub use status::HTTPStatus;
