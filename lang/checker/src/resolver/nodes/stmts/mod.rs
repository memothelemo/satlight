use super::*;

mod local_assign;
mod type_declare;

impl<'a, 'b> ResolveMut<'a, 'b> for hir::LastStmt<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        match self {
            hir::LastStmt::Return(info) => {
                info.concluding_typ = info.concluding_typ.resolve(resolver)?;
                for expr in info.exprs.iter_mut() {
                    expr.resolve(resolver)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::Stmt<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        match self {
            hir::Stmt::Call(node) => node.resolve(resolver),
            hir::Stmt::Library(node) => node.resolve(resolver),
            hir::Stmt::LocalAssign(node) => node.resolve(resolver),
            hir::Stmt::TypeDeclaration(node) => node.resolve(resolver),
        }
    }
}
