use super::*;

impl<'a, 'b> Validate<'a, 'b> for hir::Function<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        let function = if let Type::Function(function) = &self.typ {
            function
        } else {
            unreachable!()
        };
        for (idx, param) in function.parameters.iter().enumerate() {
            param.typ.validate(analyzer)?;
            if let Some(expr) = self.defaults.get(idx).unwrap() {
                analyzer.compare_types(expr.typ(), &param.typ, self.span)?;
            }
        }
        if let Some(param) = &function.varidiac_param {
            param.typ.validate(analyzer)?;
        }
        function.return_type.validate(analyzer)?;
        self.block.validate(analyzer)
    }
}
