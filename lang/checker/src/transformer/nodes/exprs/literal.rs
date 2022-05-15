use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::Literal {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self {
            ast::Literal::Bool(node) => hir::Expr::Literal(hir::Literal {
                span: node.span(),
                typ: types::makers::bool(node.span()),
                symbol: None,
                node_id: tfmr.ctx.nodes.alloc(self),
            }),
            ast::Literal::Function(_) => todo!(),
            ast::Literal::Name(node) => {
                let scope = tfmr.current_scope();
                let real_name = node.ty().as_name();

                let symbol = scope.search_variable(&tfmr.ctx, &real_name);
                if let Some(symbol_id) = symbol {
                    let symbol = tfmr.ctx.symbols.get(symbol_id).unwrap();
                    hir::Expr::Literal(hir::Literal {
                        // meh?
                        typ: symbol.get_type().unwrap().clone(),
                        span: node.span(),
                        symbol: Some(symbol_id),
                        node_id: tfmr.ctx.nodes.alloc(self),
                    })
                } else {
                    tfmr.ctx.diagnostics.push(Diagnostic::UnknownVariable {
                        name: real_name,
                        span: node.span(),
                    });
                    hir::Expr::Literal(hir::Literal {
                        typ: types::makers::any(node.span()),
                        span: node.span(),
                        symbol: Some(
                            tfmr.register_symbol(vec![node.span()], SymbolKind::UnknownVariable),
                        ),
                        node_id: tfmr.ctx.nodes.alloc(self),
                    })
                }
            }
            ast::Literal::Number(node) => hir::Expr::Literal(hir::Literal {
                span: node.span(),
                typ: types::makers::number(node.span()),
                symbol: None,
                node_id: tfmr.ctx.nodes.alloc(self),
            }),
            ast::Literal::Nil(node) => hir::Expr::Literal(hir::Literal {
                span: node.span(),
                typ: types::makers::nil(node.span()),
                symbol: None,
                node_id: tfmr.ctx.nodes.alloc(self),
            }),
            ast::Literal::Str(node) => hir::Expr::Literal(hir::Literal {
                span: node.span(),
                typ: types::makers::string(node.span()),
                symbol: None,
                node_id: tfmr.ctx.nodes.alloc(self),
            }),
            ast::Literal::Table(_) => todo!(),
            ast::Literal::Varargs(_) => todo!(),
        }
    }
}
