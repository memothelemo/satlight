use crate::meta::Checker;

use super::*;

impl<'a> Validate<'a> for hir::SetMetatable<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        let base_type = analyzer.solve_type(self.base_table.typ())?;
        let metatable_type = analyzer.solve_type(self.metatable.typ())?;

        let (mut base_table, mut metatable) = match (base_type, metatable_type) {
            (Type::Table(a), Type::Table(b)) => (a, b),
            _ => {
                return Err(AnalyzeError::InvalidLibraryUse {
                    lib: "setmetatable".to_string(),
                    span: self.span,
                });
            }
        };

        metatable.is_metatable = true;
        base_table.metatable = Some(Box::new(metatable));

        let symbol_id = match self.return_type {
            Type::Procrastinated(id, ..) => id,
            _ => unreachable!(),
        };

        unsafe {
            let symbols = std::ptr::addr_of!(analyzer.binder.symbols).as_mut();

            // Am I validating Rust memory rules now?
            let symbol = (*symbols).get_mut(symbol_id).unwrap();
            symbol.typ = Some(types::Type::Table(base_table.clone()));
        }

        Type::Table(base_table).validate(analyzer)?;
        Ok(())
    }
}

impl<'a> Validate<'a> for hir::LibraryExpr<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::LibraryExpr::SetMetatable(node) => node.validate(analyzer),
        }
    }
}
