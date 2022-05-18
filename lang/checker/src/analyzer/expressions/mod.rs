use super::*;

mod function;
mod suffixed;
mod table;

pub use function::*;

#[allow(unused)]
pub use suffixed::*;
pub use table::*;

impl<'a, 'b> Validate<'a, 'b> for hir::Expr<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Expr::Literal(node) => node.validate(analyzer),
            hir::Expr::TypeAssertion(node) => node.validate(analyzer),
            hir::Expr::Table(node) => node.validate(analyzer),
            hir::Expr::Function(node) => node.validate(analyzer),
            hir::Expr::Suffixed(node) => node.validate(analyzer),
            hir::Expr::Library(..) => todo!(),
        }
    }
}

impl<'a, 'b> Validate<'a, 'b> for hir::Literal<'b> {
    type Output = ();

    fn validate(&self, _: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        // TODO(memothelemo): Do something with other literal expressions
        Ok(())
    }
}

impl<'a, 'b> Validate<'a, 'b> for hir::TypeAssertion<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        self.base.validate(analyzer)?;
        self.cast.validate(analyzer)?;
        // TODO(memothelemo): Add check cast something...
        analyzer.compare_types(self.base.typ(), &self.cast, self.span)
    }
}
