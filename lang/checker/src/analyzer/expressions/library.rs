use super::*;

impl<'a> Validate<'a> for hir::SetMetatable<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        let base_type = analyzer.solve_type(self.base_table.typ())?;
        let metatable_type = analyzer.solve_type(self.metatable.typ())?;

        let (mut base_table, metatable) = match (base_type, metatable_type) {
            (Type::Table(a), Type::Table(b)) => (a, b),
            _ => {
                return Err(AnalyzeError::InvalidLibraryUse {
                    lib: "setmetatable".to_string(),
                    span: self.span,
                });
            }
        };

        base_table.metatable = Some(Box::new(metatable));
        println!("{:#?}", self);

        todo!()
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
