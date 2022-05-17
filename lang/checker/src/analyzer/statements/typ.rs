use super::*;

impl<'a, 'b> Validate<'a, 'b> for hir::TypeDeclaration<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        // do we have to evaluate default value of a parameter?
        if let Some(ref parameters) = self.parameters {
            for param in parameters.iter() {
                match (&param.explicit, &param.default) {
                    (Some(a), None) => a.validate(analyzer)?,
                    (None, Some(a)) => a.validate(analyzer)?,
                    (Some(a), Some(b)) => {
                        a.validate(analyzer)?;
                        b.validate(analyzer)?;
                        analyzer.compare_types(a, b, param.span)?;
                    }
                    // No checking neccessary
                    _ => {}
                };
            }
        }
        self.value.validate(analyzer)
    }
}
