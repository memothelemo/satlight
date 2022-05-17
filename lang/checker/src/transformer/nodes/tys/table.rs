use super::*;
use salite_common::dictionary::Dictionary;

pub(crate) fn visit_type_table_inner<'a, 'b>(
    tfmr: &mut Transformer<'a, 'b>,
    node: &'b ast::TypeTable,
    is_metatable: bool,
) -> Type {
    let mut entries = Dictionary::new();
    let mut array_member_count = 0;
    for field in node.fields().iter() {
        match field {
            ast::TypeTableField::Computed { key, value, .. } => {
                let key = key.transform(tfmr);
                let key_span = key.span();
                let value = value.transform(tfmr);
                entries.insert(variants::TableFieldKey::Computed(key, key_span), value);
            }
            ast::TypeTableField::Named { name, value, .. } => {
                let real_name = name.ty().as_name();
                let value = value.transform(tfmr);
                entries.insert(variants::TableFieldKey::Name(real_name, name.span()), value);
            }
            ast::TypeTableField::Array(value) => {
                array_member_count += 1;
                let value = value.transform(tfmr);
                entries.insert(
                    variants::TableFieldKey::None(array_member_count, value.span()),
                    value,
                );
            }
        }
    }
    types::Type::Table(variants::Table {
        span: node.span(),
        entries,
        is_metatable,
        metatable: None,
    })
}

impl<'a, 'b> Transform<'a, 'b> for ast::TypeTable {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        visit_type_table_inner(tfmr, self, false)
    }
}

impl<'a, 'b> Transform<'a, 'b> for ast::TypeMetatable {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        visit_type_table_inner(tfmr, self.table(), true)
    }
}
