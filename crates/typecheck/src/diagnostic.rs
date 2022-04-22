use lunar_ast::Span;
use lunar_macros::PropertyGetter;
use lunar_shared::{AnyAstError, AstErrorWithSpan};

#[derive(Debug, Clone, PropertyGetter)]
pub struct Diagnostic {
    message: String,
    span: Span,
}

impl Diagnostic {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl AnyAstError for Diagnostic {
    fn as_with_span(&self) -> Option<&dyn AstErrorWithSpan> {
        Some(self)
    }

    fn as_normal(&self) -> Option<&dyn lunar_shared::AstError> {
        None
    }
}

impl AstErrorWithSpan for Diagnostic {
    fn message(&self, _: &str) -> Result<String, lunar_shared::RangeOutOfBounds> {
        Ok(self.message.to_string())
    }

    fn notes(&self) -> Vec<String> {
        vec![]
    }

    fn span(&self) -> Span {
        self.span
    }
}
