use super::*;

impl<'a> Validate<'a> for hir::Return<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for expr in self.exprs.iter() {
            expr.validate(analyzer)?;
        }

        // check the whole statement, if it does match?
        if let Some(expected_type) = analyzer.expected_type.clone() {
            analyzer.check_lr_types(&self.concluding_typ, &expected_type, self.span)?;
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
