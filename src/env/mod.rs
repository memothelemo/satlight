mod file;

pub use file::*;

/// Project bundle module
pub mod project;

use rayon::ThreadPoolBuilder;
use std::sync::{Arc, Mutex};

use salitescript::common::errors::{SaliteError, TextSpanOutOfBounds};
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

/// Types of file paths
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum FilePath {
    /// This file comes from the standard library
    Stdin,

    /// This file comes from a real file
    Filesystem(PathBuf),
}

impl FilePath {
    /// Checks if the file path is stdin
    pub fn is_stdin(&self) -> bool {
        matches!(self, FilePath::Stdin)
    }

    /// Tries to get `PathBuf` object
    pub fn to_buf(&self) -> Option<PathBuf> {
        match self {
            FilePath::Filesystem(buf) => Some(buf.clone()),
            _ => None,
        }
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilePath::Stdin => write!(f, "<stdin>"),
            FilePath::Filesystem(p) => write!(f, "{}", p.to_string_lossy()),
        }
    }
}

/// Errors are from `parse_project` function
#[derive(Debug, Error)]
pub enum ParseProjectError {
    /// This error caused by a parsing error
    #[error("{path}:{span}: {message}")]
    ParseError {
        /// The file path from where the error come from
        path: FilePath,

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
) -> Result<HashMap<FilePath, salitescript::ast::File>, Vec<ParseProjectError>> {
    let errors = Arc::new(Mutex::new(Vec::new()));
    let collection = Arc::new(Mutex::new(HashMap::new()));

    let threads = project
        .config()
        .get()
        .max_threads
        .unwrap_or(num_cpus::get());

    log::debug!(
        "Compiling {} files using {} threads",
        project.files().len(),
        threads
    );

    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let errors_c = Arc::clone(&errors);
    let collection_c = Arc::clone(&collection);

    pool.scope(move |s| {
        let errors = Arc::clone(&errors_c);
        let collection = Arc::clone(&collection_c);

        for file in project.files() {
            let errors = Arc::clone(&errors);
            let collection = Arc::clone(&collection);

            s.spawn(move |_| {
                let ast = match salitescript::lazy_parse(file.declaration(), file.contents()) {
                    Ok(ast) => ast,
                    Err(err) => {
                        let mut errors = errors.lock().unwrap();
                        errors.push(ParseProjectError::ParseError {
                            path: file.path().clone(),
                            message: match err.message(file.contents()) {
                                Ok(output) => output,
                                Err(err) => {
                                    errors.push(ParseProjectError::TextSpanOutOfBounds(err));
                                    return;
                                }
                            },
                            span: err.span(),
                        });
                        return;
                    }
                };
                collection.lock().unwrap().insert(file.path().clone(), ast);
            });
        }
    });
    drop(pool);

    let errors = Arc::try_unwrap(errors).unwrap().into_inner().unwrap();
    if errors.is_empty() {
        Ok(Arc::try_unwrap(collection).unwrap().into_inner().unwrap())
    } else {
        Err(errors)
    }
}
