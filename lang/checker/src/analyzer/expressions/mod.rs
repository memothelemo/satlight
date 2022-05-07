use super::*;

mod call;
mod function;
mod table;

pub use call::*;
pub use function::*;
pub use table::*;

impl<'a> Validate<'a> for hir::Expr<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Expr::Literal(node) => node.validate(analyzer),
            hir::Expr::TypeAssertion(node) => node.validate(analyzer),
            hir::Expr::Table(node) => node.validate(analyzer),
            hir::Expr::Function(node) => node.validate(analyzer),
            hir::Expr::Call(node) => node.validate(analyzer),
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
        // TODO(memothelemo): Add check cast something...
        analyzer.check_lr_types(self.base.typ(), &self.cast, self.span)
    }
}
