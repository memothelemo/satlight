use super::*;

impl Validate for hir::Return {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for expr in self.exprs.iter() {
            expr.validate(analyzer)?;
        }
        Ok(())
    }
}

impl Validate for hir::LastStmt {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::LastStmt::None => Ok(()),
            hir::LastStmt::Return(node) => node.validate(analyzer),
            hir::LastStmt::Break(_) => Ok(()),
        }
    }
}
