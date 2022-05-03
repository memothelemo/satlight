use super::*;

impl<'a> Validate<'a> for types::Type {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            Type::Ref(reference) => {
                // trying to get the symbol
                let symbol = analyzer.binder.symbols.get(reference.symbol).unwrap();
                if let Some(typ) = &symbol.typ {
                    // VALIDATING TYPE ARGUMENTS
                    analyzer.resolve_type(self, typ, self.span())?;
                    typ.validate(analyzer)?;
                    Ok(())
                } else {
                    Err(AnalyzeError::InvalidType {
                        name: reference.name.to_string(),
                        span: reference.span,
                    })
                }
            }
            Type::Tuple(tuple) => {
                for typ in tuple.members.iter() {
                    typ.validate(analyzer)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
