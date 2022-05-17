use salite_ast::Span;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("{name} is referenced infinitely")]
    RecursiveType { name: String, span: Span },

    #[error("Invalid use of {lib}")]
    InvalidLibraryUse { lib: String, span: Span },

    #[error("Excessive varidiac parameter")]
    ExcessiveVarargParam { span: Span },

    #[error("Excessive parameter #{key}")]
    ExcessiveParameter { span: Span, key: usize },

    #[error("Excessive field {key}")]
    ExcessiveField { span: Span, key: String },

    #[error("Invalid field {key}: {reason}")]
    InvalidField {
        span: Span,
        key: String,
        reason: Box<ResolveError>,
    },

    #[error("Invalid metatable, did you forget to put @metatable before the table type?")]
    InvalidMetatable { span: Span },

    #[error("{metamethod} is used but it is invalid")]
    InvalidMetamethod { span: Span, metamethod: String },

    #[error("{variable} is not defined but it has explicit type of {explicit_type}")]
    NotDefined {
        explicit_type: String,
        variable: String,
        span: Span,
    },

    #[error("{name} is an invalid type")]
    InvalidType { name: String, span: Span },

    #[error("{base} expected type arguments")]
    NoArguments { span: Span, base: String },

    #[error("Missing type argument #{idx} as {expected_type}")]
    MissingTypeArgument {
        span: Span,
        idx: usize,
        expected_type: String,
    },

    #[error("Missing argument #{idx} as {expected_type}")]
    MissingArgument {
        span: Span,
        idx: usize,
        expected_type: String,
    },

    #[error("{base} has no parameters")]
    TypeHasNoParameters { span: Span, base: String },
}
