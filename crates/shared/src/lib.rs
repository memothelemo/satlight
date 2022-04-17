use std::fmt::Debug;

pub use lunar_span::{get_text_ranged, Position, RangeOutOfBounds, Span};

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

pub trait Node {
    fn span(&self) -> Span;
}
