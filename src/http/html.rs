use std::fmt::Display;

/// Generate an HTML document containing title, lead and content.
///
/// This function is mainly used to generate HTML documents from [`HTTPStatus`]
/// structs and send them to the consumer. This is especially useful for quickly
/// generating 404, 500, etc... error pages.
///
/// # Example
///
/// ```rust
/// # use servum::http::html_doc;
/// let doc = html_doc("Not Found", "404", "Page not found!");
///
/// assert!(doc.starts_with("<!DOCTYPE html>"));
/// assert!(doc.ends_with("</html>\n"));
/// assert!(doc.find("<h1>404</h1>").is_some());
/// ```
///
/// [`HTTPStatus`]: crate::http::HTTPStatus
pub fn html_doc<T, U, V>(title: T, lead: U, content: V) -> String
where
    T: Display,
    U: Display,
    V: Display,
{
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>{title}</title></head><body><h1>{lead}</h1><p>{content}</p></body></html>\n",
        title=title,
        lead=lead,
        content=content,
    )
}

#[cfg(test)]
mod test {
    use super::html_doc;

    #[test]
    fn htmldoc() {
        let doc = html_doc("Server Error", 500, "Internal server error");

        assert!(doc.starts_with("<!DOCTYPE html>"));
        assert!(doc.ends_with("</html>\n"));
        assert!(doc.find("<title>Server Error</title>").is_some());
        assert!(doc.find("<h1>500</h1>").is_some());
        assert!(doc.find("<p>Internal server error</p>").is_some());
    }
}
