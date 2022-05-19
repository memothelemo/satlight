use super::*;

impl<'a, 'b> ResolveMut<'a, 'b> for hir::TypeDeclaration<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        if let Some(params) = &mut self.parameters {
            for param in params.iter_mut() {
                param.explicit = if let Some(explicit) = &param.explicit {
                    Some(explicit.resolve(resolver)?)
                } else {
                    None
                };
                param.default = if let Some(def) = &param.default {
                    Some(def.resolve(resolver)?)
                } else {
                    None
                };
            }
        }

        let value = self.value.resolve(resolver)?;
        let symbol = resolver.ctx.get_mut().symbols.get_mut(self.symbol).unwrap();

        if let crate::SymbolKind::TypeAlias(info) = &mut symbol.kind {
            info.intrinsic = matches!(value, Type::Literal(..));
            info.typ = value.clone();
        }

        self.value = value;
        Ok(())
    }
}
