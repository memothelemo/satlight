use super::*;

impl<'a, 'b> ResolveMut<'a, 'b> for hir::LocalAssign<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        for var in self.variables.iter_mut() {
            var.explicit_type = if let Some(t) = &var.explicit_type {
                Some(t.resolve(resolver)?)
            } else {
                None
            };
            var.expr = if let Some(t) = &var.expr {
                Some(t.resolve(resolver)?)
            } else {
                None
            };
        }
        for expr in self.exprs.iter_mut() {
            expr.resolve(resolver)?;
        }
        Ok(())
    }
}
