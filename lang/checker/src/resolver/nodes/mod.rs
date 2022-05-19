mod exprs;
mod stmts;
mod typs;

use super::*;

impl<'a, 'b> ResolveMut<'a, 'b> for hir::Block<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        for stmt in self.stmts.iter_mut() {
            stmt.resolve(resolver)?;
        }
        self.last_stmt.resolve(resolver)?;
        self.actual_type = self.actual_type.resolve(resolver)?;

        let expected_type = if let Some(expected) = &self.expected_type {
            Some(expected.resolve(resolver)?)
        } else {
            None
        };
        self.expected_type = expected_type;

        Ok(())
    }
}
