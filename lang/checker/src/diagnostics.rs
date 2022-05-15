use salite_ast::Span;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Info,
    Error,
}

#[derive(Error, Debug)]
pub enum Diagnostic {
    #[error("Invalid use of {lib}")]
    InvalidLibraryUse { lib: String, span: Span },

    #[error("Unknown type alias: {name}")]
    UnknownTypeAlias { name: String, span: Span },

    #[error("Unknown variable: {name}")]
    UnknownVariable { name: String, span: Span },

    #[error("Duplicated metatable")]
    DuplicatedMetatable { span: Span },
}

impl Diagnostic {
    pub fn level(&self) -> DiagnosticLevel {
        match self {
            Diagnostic::InvalidLibraryUse { .. } => DiagnosticLevel::Error,
            Diagnostic::UnknownTypeAlias { .. } => DiagnosticLevel::Info,
            Diagnostic::UnknownVariable { .. } => DiagnosticLevel::Info,
            Diagnostic::DuplicatedMetatable { .. } => DiagnosticLevel::Error,
        }
    }
}
