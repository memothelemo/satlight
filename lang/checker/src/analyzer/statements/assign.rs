use super::*;

#[allow(clippy::or_fun_call)]
impl<'a, 'b> Validate<'a, 'b> for hir::LocalAssign<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
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

                    // assumption?
                    if !analyzer.ctx.declaration {
                        return Err(AnalyzeError::NotDefined {
                            variable: variable.name.to_string(),
                            explicit_type: utils::type_description(&analyzer.ctx, ty),
                            span: variable.name_span,
                        });
                    }
                }
                (Some(a), Some(b)) => {
                    a.validate(analyzer)?;
                    b.validate(analyzer)?;
                    analyzer.compare_types(
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
