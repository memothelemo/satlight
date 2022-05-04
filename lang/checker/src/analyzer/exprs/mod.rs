use super::*;

mod table;
pub use table::*;

impl<'a> Validate<'a> for hir::Expr<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Expr::Literal(node) => node.validate(analyzer),
            hir::Expr::TypeAssertion(node) => node.validate(analyzer),
            hir::Expr::Table(node) => node.validate(analyzer),
        }
    }
}

impl<'a> Validate<'a> for hir::Literal<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        // TODO(memothelemo): Do something with other literal expressions
        Ok(())
    }
}

impl<'a> Validate<'a> for hir::TypeAssertion<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        self.base.validate(analyzer)?;
        self.cast.validate(analyzer)?;
        analyzer.resolve_type(self.base.typ(), &self.cast, self.span)
    }
}
