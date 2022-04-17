use lunar_ast::Span;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TokenizeErrorType {
    IncompleteString,
    IncompleteComment,
    IncorrectShebang,
    UnexpectedCharacter(char),
}

impl std::fmt::Display for TokenizeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeErrorType::IncompleteString => "incomplete string".to_string(),
            TokenizeErrorType::IncompleteComment => "incomplete comment".to_string(),
            TokenizeErrorType::IncorrectShebang => "incorrect shebang".to_string(),
            TokenizeErrorType::UnexpectedCharacter(c) => format!("unexpected character: {}", c),
        }
        .fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TokenizeError {
    pub span: Span,
    pub ty: TokenizeErrorType,
}

use lunar_shared::{AnyAstError, AstErrorWithSpan, RangeOutOfBounds};
impl AstErrorWithSpan for TokenizeError {
    fn message(&self, _: &str) -> Result<String, RangeOutOfBounds> {
        Ok(self.ty.to_string())
    }

    fn span(&self) -> Span {
        self.span
    }

    fn notes(&self) -> Vec<String> {
        vec![]
    }
}

impl AnyAstError for TokenizeError {
    fn as_with_span(&self) -> Option<&dyn AstErrorWithSpan> {
        Some(self)
    }

    fn as_normal(&self) -> Option<&dyn lunar_shared::AstError> {
        None
    }
}
