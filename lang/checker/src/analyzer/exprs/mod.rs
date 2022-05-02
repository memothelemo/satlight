use super::*;

impl Validate for hir::Expr {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Expr::Literal(node) => node.validate(analyzer),
            hir::Expr::TypeAssertion(node) => node.validate(analyzer),
        }
    }
}

impl Validate for hir::Literal {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        todo!()
    }
}

impl Validate for hir::TypeAssertion {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        todo!()
    }
}
