use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeReference {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let scope = tfmr.current_scope();
        let name = self.name().ty().as_name();

        let symbol = scope.search_type_alias(&tfmr.ctx, &name);
        if let Some(symbol_id) = symbol {
            let arguments = self.arguments().as_ref().map(|arguments| {
                let mut list = Vec::new();
                for arg in arguments.iter() {
                    list.push(arg.transform(tfmr));
                }
                list
            });
            Type::Reference(variants::Reference {
                span: self.span(),
                name,
                symbol: symbol_id,
                arguments,
            })
        } else {
            tfmr.ctx.diagnostics.push(Diagnostic::UnknownTypeAlias {
                name,
                span: self.span(),
            });
            types::makers::any(self.span())
        }
    }
}
