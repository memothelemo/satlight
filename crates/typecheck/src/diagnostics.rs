#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use lunar_ast::Span;
use lunar_errors::{AnyAstError, AstErrorWithSpan};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
}

impl AnyAstError for Diagnostic {
    fn as_with_span(&self) -> Option<&dyn AstErrorWithSpan> {
        Some(self)
    }

    fn as_normal(&self) -> Option<&dyn lunar_errors::AstError> {
        None
    }
}

impl AstErrorWithSpan for Diagnostic {
    fn message(&self, _: &str) -> Result<String, lunar_shared::RangeOutOfBounds> {
        Ok(self.message.to_string())
    }

    fn notes(&self) -> Vec<String> {
        Vec::new()
    }

    fn span(&self) -> Span {
        self.span
    }
}
