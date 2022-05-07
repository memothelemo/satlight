use super::*;

impl<'a> Validate<'a> for hir::Call<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        self.base.validate(analyzer)?;

        // making sure that the base expression is a callback type
        let function_info = match analyzer.solve_type(self.base.typ())? {
            Type::Function(info) => info,
            Type::Table(tbl) if tbl.metatable.is_some() => {
                let metatable = tbl.metatable.unwrap();

                // don't worry, it will ignore the span comparison.
                let value = match metatable.entries.get(&types::TableFieldKey::Name(
                    "__call".to_string(),
                    Span::invalid(),
                )) {
                    Some(value) => value,
                    None => {
                        return Err(AnalyzeError::NonCallExpression {
                            span: self.base.span(),
                        })
                    }
                };

                // check if it is a function, meh!
                let value = analyzer.solve_type(value)?;
                match value {
                    Type::Function(info) => info,
                    _ => {
                        return Err(AnalyzeError::InvalidMetamethod {
                            span: self.base.span(),
                            metamethod: "__call".to_string(),
                        })
                    }
                }
            }
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
