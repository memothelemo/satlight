use std::path::{self, PathBuf};
use thiserror::Error;

/// A file itself, containing contents of it.
#[derive(Debug)]
pub struct SourceFile {
    path: PathBuf,
    contents: String,
}

/// Errors given when it tries to reload the SourceFile object.
#[derive(Debug, Error)]
pub enum SourceFileReloadError {
    /// File does not exists in its current file path
    #[error("File not found")]
    NotFound,

    /// There's something wrong other than file not found.
    #[error("{0}")]
    IO(std::io::Error),
}

impl SourceFile {
    /// It creates a new SourceFile object but it tries to load its contents
    /// upon the creation.
    pub fn new<T: AsRef<path::Path>>(path: T) -> Result<SourceFile, std::io::Error> {
        let contents = std::fs::read_to_string(&path)?;
        Ok(SourceFile {
            path: path.as_ref().to_path_buf(),
            contents,
        })
    }

    /// Gets the current path of the SourceFile object
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets the entire file contents of the SourceFile object
    pub fn contents(&self) -> &String {
        &self.contents
    }

    /// Attempts to reload contents of the SourceFile object
    pub fn reload(&mut self) -> Result<(), SourceFileReloadError> {
        log::debug!("Reloading source file {}", self.path().to_string_lossy());

        use std::io::ErrorKind;
        let new_contents = std::fs::read_to_string(&self.path).map_err(|e| match e.kind() {
            ErrorKind::NotFound => SourceFileReloadError::NotFound,
            _ => SourceFileReloadError::IO(e),
        })?;
        self.contents = new_contents;
        Ok(())
    }
}
