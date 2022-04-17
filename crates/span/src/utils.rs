use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RangeOutOfBounds(pub Span);

impl std::fmt::Display for RangeOutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = &self.0;
        format!("attempt to convert from span ({} > {})", s.start(), s.end()).fmt(f)
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
