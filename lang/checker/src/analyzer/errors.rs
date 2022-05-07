use salite_ast::Span;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum AnalyzeError {
    #[error("Invalid use of {lib}")]
    InvalidLibraryUse { lib: String, span: Span },

    #[error("Attempt to call with a non-call value or expression")]
    NonCallExpression { span: Span },

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
        reason: Box<AnalyzeError>,
    },

    #[error("Missing field {key}, which it expects {expected}")]
    MissingField {
        span: Span,
        key: String,
        expected: String,
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

    #[error("{value} is not extendable from {assertion}")]
    NotExtendable {
        value: String,
        assertion: String,
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

impl AnalyzeError {
    pub fn span(&self) -> Span {
        match self {
            AnalyzeError::NotDefined { span, .. } => *span,
            AnalyzeError::NotExtendable { span, .. } => *span,
            AnalyzeError::InvalidType { span, .. } => *span,
            AnalyzeError::MissingArgument { span, .. } => *span,
            AnalyzeError::NoArguments { span, .. } => *span,
            AnalyzeError::TypeHasNoParameters { span, .. } => *span,
            AnalyzeError::InvalidMetamethod { span, .. } => *span,
            AnalyzeError::MissingField { span, .. } => *span,
            AnalyzeError::InvalidField { span, .. } => *span,
            AnalyzeError::ExcessiveField { span, .. } => *span,
            AnalyzeError::InvalidMetatable { span } => *span,
            AnalyzeError::NonCallExpression { span } => *span,
            AnalyzeError::MissingTypeArgument { span, .. } => *span,
            AnalyzeError::ExcessiveParameter { span, .. } => *span,
            AnalyzeError::ExcessiveVarargParam { span, .. } => *span,
            AnalyzeError::InvalidLibraryUse { span, .. } => *span,
        }
    }
}

impl salite_common::errors::SaliteError for AnalyzeError {
    fn message(&self, _: &str) -> Result<String, salite_common::errors::TextSpanOutOfBounds> {
        Ok(self.to_string())
    }
}
