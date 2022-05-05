use salite_location::Span;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::SaliteError;

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

impl SaliteError for TokenizeErrorType {
    fn message(&self, _: &str) -> Result<String, super::TextSpanOutOfBounds> {
        Ok(self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TokenizeError {
    pub span: Span,
    pub ty: TokenizeErrorType,
}

impl SaliteError for TokenizeError {
    fn message(&self, code: &str) -> Result<String, super::TextSpanOutOfBounds> {
        self.ty.message(code)
    }
}
