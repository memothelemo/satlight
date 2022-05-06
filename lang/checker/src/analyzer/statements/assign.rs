use super::*;

#[allow(clippy::or_fun_call)]
impl<'a> Validate<'a> for hir::LocalAssign<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for variable in self.variables.iter() {
            // real expression checks
            if let Some(expr) = self.exprs.get(variable.expr_id) {
                expr.validate(analyzer)?;
            }

            // fake expression types
            match (variable.expr.as_ref(), variable.explicit_type.as_ref()) {
                (Some(ty), None) => ty.validate(analyzer)?,
                (None, Some(ty)) => {
                    ty.validate(analyzer)?;
                    return Err(AnalyzeError::NotDefined {
                        variable: variable.name.to_string(),
                        explicit_type: analyzer.type_description(ty),
                        span: variable.name_span,
                    });
                }
                (Some(a), Some(b)) => {
                    a.validate(analyzer)?;
                    b.validate(analyzer)?;
                    analyzer.check_lr_types(
                        a,
                        b,
                        variable
                            .expr_source
                            .unwrap_or(variable.explicit_type.as_ref().unwrap().span()),
                    )?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
