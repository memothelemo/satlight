use crate::types::Type;

use super::*;
use ast::SpannedNode;

impl<'a, 'b> Transform<'a, 'b> for ast::LocalFunction {
    type Output = hir::Stmt<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let name = self.name().ty().as_name();
        let node_id = tfmr.ctx.nodes.alloc(self);

        let body = transform_function_body(tfmr, self.body(), self.span(), node_id);

        let symbol_id = tfmr.insert_variable(
            &name,
            SymbolKind::BlockVariable(BlockVariableSymbol {
                name: name.to_string(),
                explicit: false,
                typ: body.typ.clone(),
            }),
            Some(self.span()),
        );

        let stmt = hir::Stmt::LocalAssign(hir::LocalAssign {
            variables: vec![hir::LocalAssignVar {
                name,
                name_span: self.name().span(),
                name_symbol: symbol_id,
                explicit_type: None,
                expr_id: 0,
                expr_source: Some(body.span),
                expr: Some(body.typ.clone()),
            }],
            span: self.span(),
            node_id: body.node_id,
            exprs: vec![hir::Expr::Function(body)],
        });

        stmt
    }
}

impl<'a, 'b> Transform<'a, 'b> for ast::LocalAssign {
    type Output = hir::Stmt<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let exprs = {
            let mut exprs = Vec::new();
            for expr in self.exprlist().iter() {
                let expr_value = expr.transform(tfmr);
                let span = expr_value.span();
                let types = expr_value.typ().clone().deref_tuples();
                for typ in types {
                    exprs.push((span, typ, expr_value.clone()));
                }
            }
            exprs
        };
        let mut variables = Vec::new();
        for (id, name) in self.names().iter().enumerate() {
            let real_name = name.name().ty().as_name();
            let (expr_source, expr) = exprs
                .get(id)
                .cloned()
                .map(|v| (Some(v.0), Some(v.1)))
                .unwrap_or((None, None));

            let symbol_id = tfmr.insert_variable(
                &real_name,
                SymbolKind::BlockVariable(BlockVariableSymbol {
                    name: real_name.to_string(),
                    typ: expr.clone().unwrap_or(types::makers::any(name.span())),
                    explicit: name.type_info().is_some(),
                }),
                Some(name.span()),
            );

            let explicit_type = name.type_info().as_ref().map(|v| v.transform(tfmr));
            let (explicit_type, expr) = match (explicit_type, expr) {
                (Some(Type::Function(assertion)), Some(Type::Function(expr))) => {
                    let (v0, v1) = tfmr.revisit_function_type(expr, assertion);
                    (Some(v0), Some(v1))
                }
                a => (a.0, a.1),
            };

            variables.push(hir::LocalAssignVar {
                name: real_name,
                name_symbol: symbol_id,
                name_span: name.span(),
                explicit_type,
                expr_source,
                expr_id: id,
                expr,
            });
        }

        hir::Stmt::LocalAssign(hir::LocalAssign {
            variables,
            span: self.span(),
            node_id: tfmr.ctx.nodes.alloc(self),
            exprs: exprs.iter().map(|v| v.2.clone()).collect(),
        })
    }
}
