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
        self.value = self.value.resolve(resolver)?;
        Ok(())
    }
}
