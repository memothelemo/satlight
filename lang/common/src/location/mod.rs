use salite_location::Span;

mod file;
pub use file::*;

#[derive(Debug)]
pub struct FileLocation {
    pub file_path: FilePath,
    pub span: Span,
}
