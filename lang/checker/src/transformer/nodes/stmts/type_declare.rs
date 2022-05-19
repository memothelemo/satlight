use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeDeclaration {
    type Output = hir::Stmt<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let parameters = if let Some(real_params) = self.params() {
            let mut params: Vec<hir::TypeParameter> = Vec::new();
            for param in real_params.iter() {
                let real_name = param.name().ty().as_name();
                let explicit_type = param.typ().as_ref().map(|v| v.transform(tfmr));
                let default_type = param.default().as_ref().map(|v| v.transform(tfmr));

                params.push(hir::TypeParameter {
                    name: real_name,
                    name_span: param.name().span(),
                    explicit: explicit_type,
                    default: default_type,
                    span: param.span(),
                });
            }
            Some(params)
        } else {
            None
        };

        // recursive types are allowed :)
        // unless it has to be explicit...
        let real_name = self.name().ty().as_name();
        let symbol_id = tfmr.insert_type_alias(
            &real_name,
            SymbolKind::TypeAlias(TypeAliasSymbol {
                name: real_name.to_string(),
                intrinsic: false,
                typ: types::makers::any(self.name().span()),
                parameters: parameters.clone(),
            }),
            Some(self.name().span()),
        );

        tfmr.push_scope(ScopeKind::TypeAliasDeclaration);

        // declare all of the parameters in an
        // isolated type declaration scope
        if let Some(ref parameters) = parameters {
            for param in parameters.iter() {
                // assume declare type variable?
                tfmr.insert_type_alias(
                    &param.name,
                    SymbolKind::TypeParameter(
                        param.name.to_string(),
                        param
                            .explicit
                            .clone()
                            .or(param.default.clone())
                            .unwrap_or(types::makers::any(param.name_span)),
                    ),
                    Some(param.name_span),
                );
            }
        }

        let value = self.typ().transform(tfmr);
        tfmr.pop_scope();

        let symbol = tfmr.ctx.symbols.get_mut(symbol_id).unwrap();
        match &mut symbol.kind {
            SymbolKind::TypeAlias(alias) => alias.typ = value.clone(),
            _ => unreachable!(),
        }

        hir::Stmt::TypeDeclaration(hir::TypeDeclaration {
            name: real_name,
            parameters,
            value,
            symbol: symbol_id,
            node_id: tfmr.ctx.nodes.alloc(self),
        })
    }
}
