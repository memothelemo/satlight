pub mod parser;
pub mod tokenizer;

#[derive(Debug, Clone, PartialEq)]
pub struct TextSpanOutOfBounds(pub lunar_location::Span);

impl std::fmt::Display for TextSpanOutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = &self.0;
        std::fmt::Display::fmt(
            &format!("attempt to convert from span ({} > {})", s.start, s.end),
            f,
        )
    }
}

impl std::error::Error for TextSpanOutOfBounds {}

#[allow(clippy::needless_lifetimes)]
pub(super) fn get_token_ranged<'a>(
    template: &'a str,
    span: lunar_location::Span,
) -> Result<&str, TextSpanOutOfBounds> {
    if !span.is_valid() {
        Err(TextSpanOutOfBounds(span))
    } else {
        let result = &template[span.start..span.end];
        if result.is_empty() {
            Ok("<eof>")
        } else {
            Ok(result)
        }
    }
}

pub trait LunarError {
    fn message(&self, code: &str) -> Result<String, TextSpanOutOfBounds>;
}
