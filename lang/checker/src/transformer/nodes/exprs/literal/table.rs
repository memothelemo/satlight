use salite_common::dictionary::Dictionary;

use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::TableCtor {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let mut fields = Vec::new();
        let mut entries = Dictionary::new();
        let mut array_member_count = 0;
        for field in self.fields().iter() {
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
                    let value = value.transform(tfmr);
                    entries.insert(
                        variants::TableFieldKey::Computed(index.typ().clone(), index.span()),
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
            span: self.span(),
            entries,
            is_metatable: false,
            metatable: None,
        });
        hir::Expr::Table(hir::Table {
            span: self.span(),
            node_id: tfmr.ctx.nodes.alloc(self),
            fields,
            typ,
        })
    }
}
