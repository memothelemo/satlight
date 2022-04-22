use lunar_ast::{Position, Span};
use std::fmt::Debug;

pub trait AstErrorWithSpan: Debug {
    fn message(&self, text: &str) -> Result<String, RangeOutOfBounds>;
    fn notes(&self) -> Vec<String>;
    fn position(&self, text: &str) -> Position {
        Position::from_offset(self.span().start(), text)
    }
    fn span(&self) -> Span;

    fn highlight_code<'a>(&self, text: &'a str) -> Result<&'a str, RangeOutOfBounds> {
        get_text_ranged(text, self.span())
    }
}

pub trait AstError: Debug {
    fn message(&self) -> String;
    fn notes(&self) -> Vec<String>;
}

pub trait AnyAstError: Debug {
    fn as_with_span(&self) -> Option<&dyn AstErrorWithSpan>;
    fn as_normal(&self) -> Option<&dyn AstError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeOutOfBounds(pub Span);

impl std::fmt::Display for RangeOutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = &self.0;
        std::fmt::Display::fmt(
            &format!("attempt to convert from span ({} > {})", s.start(), s.end()),
            f
        )
    }
}

impl std::error::Error for RangeOutOfBounds {}

#[allow(clippy::needless_lifetimes)]
pub fn get_text_ranged<'a>(template: &'a str, span: Span) -> Result<&'a str, RangeOutOfBounds> {
    if !span.is_valid() {
        Err(RangeOutOfBounds(span))
    } else {
        Ok(&template[span.start()..span.end()])
    }
}

