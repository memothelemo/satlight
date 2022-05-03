use super::*;

impl Validate for types::Type {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            Type::Ref(reference) => {
                // TODO(memothelemo): check for type parameters in the base type :)
                if let Some(args) = &reference.arguments {
                    for arg in args.iter() {
                        arg.validate(analyzer)?;
                    }
                }

                // trying to get the symbol
                let symbol = analyzer.binder.symbols.get(reference.symbol).unwrap();
                if let Some(typ) = &symbol.typ {
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
