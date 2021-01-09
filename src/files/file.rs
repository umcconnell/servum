use std::fmt;
use std::fs::DirEntry;

/// Wrapper struct around [`std::fs::DirEntry`]
///
/// This wrapper is used to display files when listing directories. The display
/// trait [`fmt::Display`] will represent the file as a HTML link to the file
/// with the corresponding file or folder name.
pub struct File(pub DirEntry);

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_dir = &self
            .0
            .file_type()
            .map(|entry| match entry.is_dir() {
                true => "/",
                false => "",
            })
            .unwrap_or("");

        write!(
            f,
            "<a href=\"./{file_name}{is_dir}\">{file_name}{is_dir}</a>",
            file_name = &self.0.file_name().to_str().unwrap(),
            is_dir = is_dir
        )
    }
}
