use crate::meta::Checker;

use super::*;
use std::borrow::Borrow;

impl<'a> Validate<'a> for types::Type {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            Type::Table(tbl) => {
                for (key, value) in tbl.entries.iter() {
                    if let ctypes::TableFieldKey::Computed(ref expr) = key {
                        expr.validate(analyzer)?;
                    }
                    value.validate(analyzer)?;
                }
                if let Some(ref metatable) = tbl.metatable {
                    let metatable: &ctypes::Table = metatable.borrow();
                    crate::meta::Standard::check(analyzer, metatable)?;
                }
                Ok(())
            }
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
