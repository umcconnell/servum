use std::path::{Component, Path, PathBuf};

/// Try decoding hex encoding after `%` in URIs.
///
/// If the hex encoding cannot be decoded to decimal, e.g. because of `%` in
/// filenames, bad requests, etc..., [`None`] will be returned.
///
/// Code from:
/// [`https://github.com/servo/rust-url/blob/master/percent_encoding/src/lib.rs#L360-L366`]
///
/// [`https://github.com/servo/rust-url/blob/master/percent_encoding/src/lib.rs#L360-L366`]:
/// https://github.com/servo/rust-url/blob/master/percent_encoding/src/lib.rs#L360-L366
fn from_hex(it: &mut std::slice::Iter<u8>) -> Option<u8> {
    let mut clone = it.clone();

    let a = char::from(*clone.next()?).to_digit(16)?;
    let b = char::from(*clone.next()?).to_digit(16)?;

    *it = clone;

    Some(a as u8 * 16_u8 + b as u8)
}

/// Decode percent-encoded URIs
///
/// Create a new [`PathBuf`] with all `%` decoded to UTF-8. Internally iterates
/// the input and calls [`from_hex`] on all `%`.
///
/// # Example
///
/// ```rust
/// # use servum::files::path::decode_percents;
/// # use std::path::PathBuf;
/// let path = "/%F0%9F%A6%80.html";
///
/// assert_eq!(decode_percents(path), PathBuf::from("/🦀.html"));
/// ```
pub fn decode_percents(path: &str) -> PathBuf {
    if !path.contains('%') {
        return PathBuf::from(path);
    }

    let mut acc: Vec<u8> = Vec::with_capacity(path.len() + 1);
    let mut it = path.as_bytes().iter();

    while let Some(el) = it.next() {
        acc.push(match el {
            b'%' => from_hex(&mut it).unwrap_or(b'%'),
            byte => *byte,
        })
    }

    PathBuf::from(std::str::from_utf8(&acc).unwrap())
}

/// Normalize a file path
///
/// Removes unecessary path components, such as `..`, `.`, without checking the
/// file system, in contrast to using [`std::path::Path::canonicalize`]. Creates
/// a new [`PathBuf`].
///
/// See [`IETF RFC 3986 Section 5.2.4`] for more information on the
/// normalization algorithm.
///
/// Code from:
/// [`https://github.com/rust-lang/cargo/blob/2e4cfc2b7d43328b207879228a2ca7d427d188bb/src/cargo/util/paths.rs#L65-L90`]
///
/// # Example
///
/// ```rust
/// # use servum::files::path::normalize_path;
/// # use std::path::{Path, PathBuf};
/// let path = Path::new("./subdir/.././index.html");
///
/// assert_eq!(normalize_path(path), PathBuf::from("index.html"));
/// ```
///
/// [`https://github.com/rust-lang/cargo/blob/2e4cfc2b7d43328b207879228a2ca7d427d188bb/src/cargo/util/paths.rs#L65-L90`]:
/// https://github.com/rust-lang/cargo/blob/2e4cfc2b7d43328b207879228a2ca7d427d188bb/src/cargo/util/paths.rs#L65-L90
/// [`IETF RFC 3986 Section 5.2.4`]: https://tools.ietf.org/html/rfc3986#section-5.2.4
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret =
        if let Some(c @ Component::Prefix(..)) = components.peek().copied() {
            components.next();
            PathBuf::from(c.as_os_str())
        } else {
            PathBuf::with_capacity(path.as_os_str().len() + 1)
        };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

/// Process a file path
///
/// Combines [`decode_percents`] and [`normalize_path`] in one function. An
/// additional base path is joined onto the input path to be processed.
///
/// Example
///
/// ```rust
/// # use servum::files::path::process_path;
/// # use std::path::{Path, PathBuf};
/// let path = Path::new("./subdir/subsubdir/.././.././%F0%9F%A6%80.html");
/// let base_dir = Path::new("/");
///
/// assert_eq!(process_path(path, base_dir), PathBuf::from("/🦀.html"));
/// ```
pub fn process_path(path: &Path, base_dir: &Path) -> PathBuf {
    let filename = decode_percents(path.to_str().unwrap());
    normalize_path(&base_dir.join(filename))
}

#[cfg(test)]
mod test {
    use super::{decode_percents, normalize_path, Path, PathBuf};

    #[test]
    fn decode_no_encoding() {
        let path = "./dir/./../dir/subdirfile-name.txt";

        assert_eq!(decode_percents(&path), PathBuf::from(path));
    }

    #[test]
    fn decode_encoding() {
        let path = ".%2Fpath%20with%20spaces%2Fand%20%E2%9C%8B%F0%9F%98%81%2Fmore%20%F0%9F%9A%80%2Feven%20more%20%F0%9F%9A%A9%2Fstop%20%E2%9B%94.txt";

        assert_eq!(
            decode_percents(&path),
            PathBuf::from(
                "./path with spaces/and ✋😁/more 🚀/even more 🚩/stop ⛔.txt"
            )
        );
    }

    #[test]
    fn decode_edge_case() {
        let path = ".%2fpath%20with%20spaces%2fedge%20case%.txt";
        //  ^-- lowercase                      ^-- percent at the end
        assert_eq!(
            decode_percents(&path),
            PathBuf::from("./path with spaces/edge case%.txt")
        );
    }

    #[test]
    fn normalize() {
        let path = "/a/b/c/./../../g";

        assert_eq!(normalize_path(Path::new(path)), PathBuf::from("/a/g"));
    }

    #[test]
    fn normalize_2() {
        let path = "mid/content=5/../6";

        assert_eq!(normalize_path(Path::new(path)), PathBuf::from("mid/6"));
    }
}
