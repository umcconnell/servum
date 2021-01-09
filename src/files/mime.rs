use std::path::Path;

/// Guess a file's MIME type based on it's extension
///
/// This function will try to gues the appropriate MIME type for a given file
/// by matching on it's file type. If no match is found, [`None`] is returned
/// instead.
///
/// The list of supported MIME types are adapted from
/// [`https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types`]
///
/// # Example
///
/// ```rust
/// # use servum::files::mime::guess_mime_type;
/// # use std::path::Path;
/// let path = Path::new("/path/to/script.js");
///
/// assert_eq!(guess_mime_type(path).unwrap(), "text/javascript");
/// ```
///
/// [`https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types`]:
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
pub fn guess_mime_type(path: &Path) -> Option<&'static str> {
    path.extension()
        .and_then(|ext| match ext.to_str().unwrap() {
            "aac" => Some("audio/aac"),
            "abw" => Some("application/x-abiword"),
            "arc" => Some("application/x-freearc"),
            "avi" => Some("video/x-msvideo"),
            "azw" => Some("application/vnd.amazon.ebook"),
            "bin" => Some("application/octet-stream"),
            "bmp" => Some("image/bmp"),
            "bz" => Some("application/x-bzip"),
            "bz2" => Some("application/x-bzip2"),
            "csh" => Some("application/x-csh"),
            "css" => Some("text/css"),
            "csv" => Some("text/csv"),
            "doc" => Some("application/msword"),
            "docx" => {
                Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document")
            }
            "eot" => Some("application/vnd.ms-fontobject"),
            "epub" => Some("application/epub+zip"),
            "gz" => Some("application/gzip"),
            "gif" => Some("image/gif"),
            "htm" => Some("text/html"),
            "html" => Some("text/html"),
            "ico" => Some("image/vnd.microsoft.icon"),
            "ics" => Some("text/calendar"),
            "jar" => Some("application/java-archive"),
            "jpeg" => Some("image/jpeg"),
            "jpg" => Some("image/jpeg"),
            "js" => Some("text/javascript"),
            "json" => Some("application/json"),
            "jsonld" => Some("application/ld+json"),
            "mid" => Some("audio/midi"),
            "midi" => Some("audio/midi"),
            "mjs" => Some("text/javascript"),
            "mp3" => Some("audio/mpeg"),
            "mpeg" => Some("video/mpeg"),
            "mpkg" => Some("application/vnd.apple.installer+xml"),
            "odp" => Some("application/vnd.oasis.opendocument.presentation"),
            "ods" => Some("application/vnd.oasis.opendocument.spreadsheet"),
            "odt" => Some("application/vnd.oasis.opendocument.text"),
            "oga" => Some("audio/ogg"),
            "ogv" => Some("video/ogg"),
            "ogx" => Some("application/ogg"),
            "opus" => Some("audio/opus"),
            "otf" => Some("font/otf"),
            "png" => Some("image/png"),
            "pdf" => Some("application/pdf"),
            "php" => Some("application/x-httpd-php"),
            "ppt" => Some("application/vnd.ms-powerpoint"),
            "pptx" => {
                Some("application/vnd.openxmlformats-officedocument.presentationml.presentation")
            }
            "rar" => Some("application/vnd.rar"),
            "rtf" => Some("application/rtf"),
            "sh" => Some("application/x-sh"),
            "svg" => Some("image/svg+xml"),
            "swf" => Some("application/x-shockwave-flash"),
            "tar" => Some("application/x-tar"),
            "tif" => Some("image/tiff"),
            "tiff" => Some("image/tiff"),
            "ts" => Some("video/mp2t"),
            "ttf" => Some("font/ttf"),
            "txt" => Some("text/plain"),
            "vsd" => Some("application/vnd.visio"),
            "wav" => Some("audio/wav"),
            "weba" => Some("audio/webm"),
            "webm" => Some("video/webm"),
            "webp" => Some("image/webp"),
            "woff" => Some("font/woff"),
            "woff2" => Some("font/woff2"),
            "xhtml" => Some("application/xhtml+xml"),
            "xls" => Some("application/vnd.ms-excel"),
            "xlsx" => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
            "xml" => Some("application/xml"),
            "xul" => Some("application/vnd.mozilla.xul+xml"),
            "zip" => Some("application/zip"),
            "3gp" => Some("video/3gpp"),
            "3g2" => Some("video/3gpp2"),
            "7z" => Some("application/x-7z-compressed"),
            "rs" => Some("text/x-rust"),
            _ => None,
        })
}
