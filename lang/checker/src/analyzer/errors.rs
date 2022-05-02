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

    #[error("{left} is not extendable from {right}")]
    NotExtendable {
        left: String,
        right: String,
        span: Span,
    },
}

impl AnalyzeError {
    pub fn span(&self) -> Span {
        match self {
            AnalyzeError::NotDefined { span, .. } => *span,
            AnalyzeError::NotExtendable { span, .. } => *span,
        }
    }
}

impl lunar_common::errors::LunarError for AnalyzeError {
    fn message(&self, _: &str) -> Result<String, lunar_common::errors::TextSpanOutOfBounds> {
        Ok(self.to_string())
    }
}
