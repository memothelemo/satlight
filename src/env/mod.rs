mod file;

pub use file::*;

/// Project bundle module
pub mod project;

#[cfg(feature = "multithreading")]
use rayon::ThreadPoolBuilder;

#[cfg(feature = "multithreading")]
use std::sync::{Arc, Mutex};

use salitescript::common::errors::{SaliteError, TextSpanOutOfBounds};
use std::{collections::HashMap, path::PathBuf};
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
) -> Result<HashMap<PathBuf, salitescript::ast::File>, Vec<ParseProjectError>> {
    #[cfg(feature = "multithreading")]
    {
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
                log::debug!("Compiling {}", file.path().to_string_lossy());

                let errors = Arc::clone(&errors);
                let collection = Arc::clone(&collection);

                s.spawn(move |_| {
                    let ast = match salitescript::lazy_parse(file.contents()) {
                        Ok(ast) => ast,
                        Err(err) => {
                            let mut errors = errors.lock().unwrap();
                            errors.push(ParseProjectError::ParseError {
                                path: file.path().to_path_buf(),
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
                    collection
                        .lock()
                        .unwrap()
                        .insert(file.path().to_path_buf(), ast);
                });
            }
        });

        let errors = Arc::try_unwrap(errors).unwrap().into_inner().unwrap();
        if errors.is_empty() {
            Ok(Arc::try_unwrap(collection).unwrap().into_inner().unwrap())
        } else {
            Err(errors)
        }
    }
    #[cfg(not(feature = "multithreading"))]
    {
        let mut errors = Vec::new();
        let mut collection = HashMap::new();

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
}
