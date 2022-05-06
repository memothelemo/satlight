use super::*;

impl<'a> Validate<'a> for hir::Table<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for (key, value) in self.fields.iter() {
            if let hir::TableFieldKey::Computed(ref expr) = key {
                expr.validate(analyzer)?;
            }
            value.validate(analyzer)?;
        }
        Ok(())
    }
}
