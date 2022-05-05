mod file;

pub use file::*;

/// Project bundle module
pub mod project;

use chashmap::CHashMap;
use salitescript::common::errors::{SaliteError, TextSpanOutOfBounds};
use std::path::PathBuf;
use thiserror::Error;

/// Errors are from `parse_project` function
#[derive(Debug, Error)]
pub enum ParseProjectError {
    /// This error caused by a parsing error
    #[error("{path}:{span}: {message}")]
    ParseError {
        /// The file path from where the error come from
        path: PathBuf,

        /// The exact reason why it got an error
        message: String,

        /// The source of an error in a file
        span: salitescript::ast::Span,
    },

    /// This error caused by a text span out of bounds while
    /// trying to render out `ParseProjectError::ParseError` variant
    #[error("{0}")]
    TextSpanOutOfBounds(TextSpanOutOfBounds),
}

/// Attempts to compile all of the files from the Project object
pub fn parse_project(
    project: &project::Project,
) -> Result<CHashMap<PathBuf, salitescript::ast::File>, Vec<ParseProjectError>> {
    let mut errors = Vec::new();
    let collection = CHashMap::new();

    for file in project.files() {
        log::debug!("Compiling {}", file.path().to_string_lossy());
        let ast = match salitescript::lazy_parse(file.contents()) {
            Ok(ast) => ast,
            Err(err) => {
                errors.push(ParseProjectError::ParseError {
                    path: file.path().to_path_buf(),
                    message: match err.message(file.contents()) {
                        Ok(output) => output,
                        Err(err) => {
                            errors.push(ParseProjectError::TextSpanOutOfBounds(err));
                            continue;
                        }
                    },
                    span: err.span(),
                });
                continue;
            }
        };
        collection.insert(file.path().to_path_buf(), ast);
    }

    if errors.is_empty() {
        Ok(collection)
    } else {
        Err(errors)
    }
}
