use super::*;

impl Validate for hir::TypeDeclaration {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        // do we have to evaluate default value of a parameter?
        if let Some(ref parameters) = self.parameters {
            for param in parameters.iter() {
                match (&param.explicit, &param.default) {
                    (Some(a), None) => a.validate(analyzer)?,
                    (None, Some(a)) => a.validate(analyzer)?,
                    (Some(a), Some(b)) => {
                        a.validate(analyzer)?;
                        b.validate(analyzer)?;
                        analyzer.resolve_type(a, b, param.span)?;
                    }
                    // No checking neccessary
                    _ => {}
                };
            }
        }
        self.value.validate(analyzer)
    }
}
