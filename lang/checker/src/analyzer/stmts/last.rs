use super::*;

impl<'a> Validate<'a> for hir::Return<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for expr in self.exprs.iter() {
            expr.validate(analyzer)?;
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for hir::LastStmt<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::LastStmt::None => Ok(()),
            hir::LastStmt::Return(node) => node.validate(analyzer),
            hir::LastStmt::Break(..) => Ok(()),
        }
    }
}
