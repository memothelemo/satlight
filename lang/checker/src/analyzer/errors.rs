use lunar_ast::Span;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum AnalyzeError {
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

    #[error("Expected argument #{idx} in {base} as {expected_type}")]
    MissingArgument {
        span: Span,
        idx: usize,
        base: String,
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
        }
    }
}

impl lunar_common::errors::LunarError for AnalyzeError {
    fn message(&self, _: &str) -> Result<String, lunar_common::errors::TextSpanOutOfBounds> {
        Ok(self.to_string())
    }
}
