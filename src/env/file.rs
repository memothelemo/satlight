use super::FilePath;
use std::path;
use thiserror::Error;

/// A file itself, containing contents of it.
#[derive(Debug)]
pub struct SourceFile {
    declaration: bool,
    path: FilePath,
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
    /// Creates an object with a prepared configurations
    pub fn new(
        declaration: bool,
        contents: String,
        path: FilePath,
    ) -> Result<SourceFile, std::io::Error> {
        Ok(SourceFile {
            declaration,
            path,
            contents,
        })
    }

    /// It creates a new SourceFile object with an unknown FilePath variant.
    pub fn from_unknown_variant(
        declaration: bool,
        path: FilePath,
        contents: Option<String>,
    ) -> Result<SourceFile, std::io::Error> {
        let contents = match &path {
            FilePath::Stdin => contents.unwrap_or(String::new()),
            FilePath::Filesystem(path) => std::fs::read_to_string(&path)?,
        };
        Ok(SourceFile {
            declaration,
            path,
            contents,
        })
    }

    /// It creates a new SourceFile object but it tries to load its contents
    /// upon the creation.
    pub fn from_filesystem<T: AsRef<path::Path>>(
        declaration: bool,
        path: T,
    ) -> Result<SourceFile, std::io::Error> {
        let contents = std::fs::read_to_string(&path)?;
        Ok(SourceFile {
            declaration,
            path: FilePath::Filesystem(path.as_ref().to_path_buf()),
            contents,
        })
    }

    /// Gets the boolean value if this file is for declaration
    pub fn declaration(&self) -> bool {
        self.declaration
    }

    /// Gets the current path of the SourceFile object
    pub fn path(&self) -> &FilePath {
        &self.path
    }

    /// Gets the entire file contents of the SourceFile object
    pub fn contents(&self) -> &String {
        &self.contents
    }

    /// Attempts to reload contents of the SourceFile object
    pub fn reload(&mut self) -> Result<(), SourceFileReloadError> {
        if let FilePath::Filesystem(path) = &self.path {
            log::debug!("Reloading source file {}", path.to_string_lossy());

            use std::io::ErrorKind;
            let new_contents = std::fs::read_to_string(&path).map_err(|e| match e.kind() {
                ErrorKind::NotFound => SourceFileReloadError::NotFound,
                _ => SourceFileReloadError::IO(e),
            })?;
            self.contents = new_contents;
        }
        Ok(())
    }
}
