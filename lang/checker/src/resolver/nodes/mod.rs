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
        Ok(())
    }
}
