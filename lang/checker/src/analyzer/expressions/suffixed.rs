use super::*;

pub(crate) fn validate_suffix_call<'a, 'b>(
    analyzer: &mut Analyzer<'a, 'b>,
    suffixed: &hir::Suffixed<'b>,
    args: &[hir::Expr<'b>],
) -> AnalyzeResult {
    // making sure that the base expression is a callback type
    let function_info = match suffixed.base.typ() {
        Type::Function(info) => info,
        Type::Table(tbl) if tbl.metatable.is_some() => {
            let metatable = tbl.metatable.as_ref().unwrap();

            // don't worry, it will ignore the span comparison.
            let value = match metatable.entries.get(&variants::TableFieldKey::Name(
                "__call".to_string(),
                Span::invalid(),
            )) {
                Some(value) => value,
                None => {
                    return Err(AnalyzeError::NonCallExpression {
                        span: suffixed.base.span(),
                    })
                }
            };

            // check if it is a function, meh!
            match value {
                Type::Function(info) => info,
                _ => {
                    return Err(AnalyzeError::InvalidMetamethod {
                        span: suffixed.base.span(),
                        metamethod: "__call".to_string(),
                    })
                }
            }
        }
        _ => {
            return Err(AnalyzeError::NonCallExpression {
                span: suffixed.base.span(),
            })
        }
    };

    // checking each by each parameter
    for (idx, param) in function_info.parameters.iter().enumerate() {
        let arg = args.get(idx);
        let arg = if let Some(arg) = arg {
            arg.typ()
        } else if param.optional {
            // automatically ignore it! :)
            continue;
        } else {
            return Err(AnalyzeError::MissingArgument {
                span: suffixed.span,
                idx: idx + 1,
                expected_type: utils::type_description(&analyzer.ctx, &param.typ),
            });
        };
        analyzer.compare_types(arg, &param.typ, arg.span())?;
    }

    Ok(())
}

impl<'a, 'b> Validate<'a, 'b> for hir::Suffixed<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        self.base.validate(analyzer)?;
        match &self.kind {
            hir::SuffixKind::Call(args) => validate_suffix_call(analyzer, self, args),
        }
    }
}
