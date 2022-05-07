use super::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Diagnostic {
    #[error("Unknown type alias: {name}")]
    UnknownTypeAlias { name: String, span: Span },

    #[error("Unknown variable: {name}")]
    UnknownVariable { name: String, span: Span },

    #[error("Duplicated metatable")]
    DuplicatedMetatable { span: Span },
}