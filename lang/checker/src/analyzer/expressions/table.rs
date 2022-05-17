use super::*;

impl<'a, 'b> Validate<'a, 'b> for hir::Table<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        for (key, value) in self.fields.iter() {
            if let hir::TableFieldKey::Computed(ref expr) = key {
                expr.validate(analyzer)?;
            }
            value.validate(analyzer)?;
        }
        Ok(())
    }
}
