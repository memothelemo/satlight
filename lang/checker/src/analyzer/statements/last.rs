use super::*;

impl<'a, 'b> Validate<'a, 'b> for hir::Return<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        for expr in self.exprs.iter() {
            expr.validate(analyzer)?;
        }

        // check the whole statement, if it does match?
        if let Some(expected_type) = analyzer.expected_type.clone() {
            analyzer.compare_types(&self.concluding_typ, &expected_type, self.span)?;
        }

        Ok(())
    }
}

impl<'a, 'b> Validate<'a, 'b> for hir::LastStmt<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::LastStmt::None => Ok(()),
            hir::LastStmt::Return(node) => node.validate(analyzer),
            hir::LastStmt::Break(..) => Ok(()),
        }
    }
}
