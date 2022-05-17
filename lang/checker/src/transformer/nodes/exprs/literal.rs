use super::*;

mod function;
mod table;

#[allow(unused)]
pub use function::*;
use salite_common::dictionary::Dictionary;
pub use table::*;

#[macro_export]
macro_rules! literal {
    ($node:expr, $tfmr:expr, $self:expr, $ty:ident) => {
        hir::Expr::Literal(hir::Literal {
            span: $node.span(),
            typ: types::makers::$ty($node.span()),
            symbol: None,
            node_id: $tfmr.ctx.nodes.alloc($self),
        })
    };
}

impl<'a, 'b> Transform<'a, 'b> for ast::Literal {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self {
            ast::Literal::Bool(node) => literal!(node, tfmr, self, bool),
            ast::Literal::Function(node) => node.transform(tfmr),
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
            ast::Literal::Number(node) => literal!(node, tfmr, self, number),
            ast::Literal::Nil(node) => literal!(node, tfmr, self, nil),
            ast::Literal::Str(node) => literal!(node, tfmr, self, string),
            ast::Literal::Table(node) => {
                let mut fields = Vec::new();
                let mut entries = Dictionary::new();
                let mut array_member_count = 0;
                for field in node.fields().iter() {
                    let field = match field {
                        ast::TableField::Array(expr) => {
                            array_member_count += 1;
                            let expr = expr.transform(tfmr);
                            entries.insert(
                                variants::TableFieldKey::None(array_member_count, expr.span()),
                                expr.typ().clone(),
                            );
                            (hir::TableFieldKey::None, expr)
                        }
                        ast::TableField::Expr { index, value, .. } => {
                            let index = index.transform(tfmr);
                            let index_span = index.span();
                            let value = value.transform(tfmr);
                            entries.insert(
                                variants::TableFieldKey::Computed(index.typ().clone(), index_span),
                                value.typ().clone(),
                            );
                            (hir::TableFieldKey::Computed(index), value)
                        }
                        ast::TableField::Named { name, value, .. } => {
                            let value = value.transform(tfmr);
                            entries.insert(
                                variants::TableFieldKey::Name(name.ty().as_name(), name.span()),
                                value.typ().clone(),
                            );
                            (
                                hir::TableFieldKey::Name(name.ty().as_name(), name.span()),
                                value,
                            )
                        }
                    };
                    fields.push(field);
                }
                let typ = types::Type::Table(variants::Table {
                    span: node.span(),
                    entries,
                    is_metatable: false,
                    metatable: None,
                });
                hir::Expr::Table(hir::Table {
                    span: node.span(),
                    node_id: tfmr.ctx.nodes.alloc(node),
                    fields,
                    typ,
                })
            }
            ast::Literal::Varargs(_) => todo!(),
        }
    }
}
