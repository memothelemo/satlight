#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InternalParseError {
    NoMatch,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ParseErrorType {
    Internal(InternalParseError),
    UnexpectedToken(lunar_tokens::Token),
    Expected {
        expected: String,
        token: lunar_tokens::Token,
    },
}

impl LunarError for ParseErrorType {
    fn message(&self, code: &str) -> Result<String, TextSpanOutOfBounds> {
        Ok(match self {
            ParseErrorType::Internal(typ) => match typ {
                InternalParseError::NoMatch => "internal error: no match".to_string(),
            },
            ParseErrorType::UnexpectedToken(tok) => {
                format!("unexpected token `{}`", get_token_ranged(code, tok.span())?)
            }
            ParseErrorType::Expected { expected, token } => {
                format!(
                    "expected {} got `{}`",
                    expected,
                    get_token_ranged(code, token.span())?
                )
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ParseError {
    pub ty: ParseErrorType,
    pub span: lunar_location::Span,
}

impl LunarError for ParseError {
    fn message(&self, code: &str) -> Result<String, TextSpanOutOfBounds> {
        self.ty.message(code)
    }
}
