use super::*;

impl<'a, 'b> Validate<'a, 'b> for Type {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        match self {
            Type::Function(node) => {
                for param in node.parameters.iter() {
                    param.typ.validate(analyzer)?;
                }
                if let Some(param) = &node.varidiac_param {
                    param.typ.validate(analyzer)?;
                }
                node.return_type.validate(analyzer)?;
                Ok(())
            }
            Type::Table(tbl) => {
                for (key, value) in tbl.entries.iter() {
                    if let variants::TableFieldKey::Computed(ref expr, ..) = key {
                        expr.validate(analyzer)?;
                    }
                    value.validate(analyzer)?;
                }
                if let Some(..) = tbl.metatable {
                    println!("Metatable checking is in progress");
                    //let metatable: &variantsTable = metatable.borrow();
                    //crate::meta::Standard::check(analyzer, metatable)?;
                }
                Ok(())
            }
            Type::Reference(reference) => {
                // trying to get the symbol
                let symbol = analyzer.ctx.symbols.get(reference.symbol).unwrap();
                let typ = symbol.get_type().cloned();

                if let Some(typ) = typ {
                    // VALIDATING TYPE ARGUMENTS
                    analyzer.compare_types(self, &typ, self.span())?;
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
