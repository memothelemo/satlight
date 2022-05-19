use super::*;

impl<'a, 'b> Resolver<'a, 'b> {
    pub(crate) fn revisit_function_type(
        &mut self,
        mut base: types::variants::Function,
        assertion: types::variants::Function,
    ) -> (types::Type, types::Type) {
        for (idx, base_param) in base.parameters.iter_mut().enumerate() {
            if let Some(assertion_param) = assertion.parameters.get(idx) {
                if matches!(base_param.typ, types::Type::Any(..)) {
                    // overiding the parameter guess enough?
                    base_param.typ = assertion_param.typ.clone();
                    *base_param.typ.span_mut() = base_param.span;
                }
            }
        }
        (
            types::Type::Function(base),
            types::Type::Function(assertion),
        )
    }
}

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

            let (v0, v1) = if let (Some(Type::Function(assertion)), Some(Type::Function(value))) =
                (var.explicit_type.as_ref(), var.expr.as_ref())
            {
                let (v0, v1) = resolver.revisit_function_type(assertion.clone(), value.clone());
                (Some(v0), Some(v1))
            } else {
                (var.explicit_type.clone(), var.expr.clone())
            };
            var.explicit_type = v0;
            var.expr = v1;
        }
        for expr in self.exprs.iter_mut() {
            expr.resolve(resolver)?;
        }
        Ok(())
    }
}
