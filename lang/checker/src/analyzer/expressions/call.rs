use super::*;

impl<'a> Validate<'a> for hir::Call<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        self.base.validate(analyzer)?;

        // making sure that the base expression is a callback type
        let function_info = match analyzer.skip_downwards(self.base.typ().clone()) {
            Type::Function(info) => info,
            _ => {
                return Err(AnalyzeError::NonCallExpression {
                    span: self.base.span(),
                })
            }
        };

        // checking each by each parameter
        for (idx, param) in function_info.parameters.iter().enumerate() {
            let arg = self.arguments.get(idx);
            let arg = if let Some(arg) = arg {
                arg.typ()
            } else if param.optional {
                // automatically ignore it! :)
                continue;
            } else {
                return Err(AnalyzeError::MissingArgument {
                    span: self.span,
                    idx: idx + 1,
                    expected_type: analyzer.type_description(&param.typ),
                });
            };
            analyzer.check_lr_types(arg, &param.typ, arg.span())?;
        }

        Ok(())
    }
}
