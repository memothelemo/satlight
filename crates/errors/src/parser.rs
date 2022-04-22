use lunar_ast as ast;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InternalParseError {
    NoMatch,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ParseErrorType {
    Internal(InternalParseError),
    Expected { expected: String, token: ast::Token },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ParseError {
    pub ty: ParseErrorType,
    pub span: ast::Span,
}

use crate::{AnyAstError, AstErrorWithSpan};
use lunar_shared::{get_text_ranged, RangeOutOfBounds};

impl AstErrorWithSpan for ParseError {
    fn message(&self, code: &str) -> Result<String, RangeOutOfBounds> {
        Ok(match &self.ty {
            ParseErrorType::Internal(e) => format!(
                r#"Internal error occurred ({}), please report on GitHub."#,
                match e {
                    InternalParseError::NoMatch => "No match",
                },
            ),
            ParseErrorType::Expected { expected, token } => format!(
                "Expected {} got {}",
                expected,
                get_text_ranged(code, token.span())?
            ),
        })
    }

    fn notes(&self) -> Vec<String> {
        vec![]
    }

    fn span(&self) -> ast::Span {
        self.span
    }
}

impl AnyAstError for ParseError {
    fn as_with_span(&self) -> Option<&dyn AstErrorWithSpan> {
        Some(self)
    }

    fn as_normal(&self) -> Option<&dyn lunar_shared::AstError> {
        None
    }
}
