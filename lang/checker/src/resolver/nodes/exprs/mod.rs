use super::*;

impl<'a, 'b> ResolveMut<'a, 'b> for hir::SuffixKind<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        match self {
            hir::SuffixKind::Call(args) => {
                for arg in args.iter_mut() {
                    arg.resolve(resolver)?;
                }
                Ok(())
            }
        }
    }
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::Suffixed<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        self.base.resolve(resolver)?;
        self.kind.resolve(resolver)
    }
}

// impl<'a, 'b> ResolveMut<'a, 'b> for hir::Call<'b> {
//     type Output = ();

//     fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
//         self.base.resolve(resolver)?;
//         for arg in self.arguments.iter_mut() {
//             arg.resolve(resolver)?;
//         }
//         Ok(())
//     }
// }

impl<'a, 'b> ResolveMut<'a, 'b> for hir::LibraryExpr<'b> {
    type Output = ();

    fn resolve(&mut self, _: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        // TODO(memothelemo): later
        todo!()
    }
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::Function<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        self.block.resolve(resolver)?;
        for expr in self.defaults.iter_mut().flatten() {
            expr.resolve(resolver)?;
        }
        self.typ = self.typ.resolve(resolver)?;
        Ok(())
    }
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::Literal<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        self.typ = self.typ.resolve(resolver)?;
        Ok(())
    }
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::TypeAssertion<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        self.base.resolve(resolver)?;
        self.cast = self.cast.resolve(resolver)?;
        Ok(())
    }
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::Table<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        self.typ = self.typ.resolve(resolver)?;

        for (field, expr) in self.fields.iter_mut() {
            if let hir::TableFieldKey::Computed(expr) = field {
                expr.resolve(resolver)?;
            }
            expr.resolve(resolver)?;
        }

        Ok(())
    }
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::Expr<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        match self {
            hir::Expr::Suffixed(node) => node.resolve(resolver),
            hir::Expr::Function(node) => node.resolve(resolver),
            hir::Expr::Library(node) => node.resolve(resolver),
            hir::Expr::Literal(node) => node.resolve(resolver),
            hir::Expr::TypeAssertion(node) => node.resolve(resolver),
            hir::Expr::Table(node) => node.resolve(resolver),
        }
    }
}
