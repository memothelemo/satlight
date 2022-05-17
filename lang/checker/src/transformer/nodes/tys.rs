use super::*;
use crate::types::Type;

mod callback;
mod reference;
mod sections;
mod table;
mod tuple;

pub use callback::*;
pub use reference::*;
pub use sections::*;
pub use tuple::*;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeInfo {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self {
            ast::TypeInfo::Callback(node) => node.transform(tfmr),
            ast::TypeInfo::Intersection(node) => node.transform(tfmr),
            ast::TypeInfo::Reference(node) => node.transform(tfmr),
            ast::TypeInfo::Metatable(node) => node.transform(tfmr),
            ast::TypeInfo::Table(node) => node.transform(tfmr),
            ast::TypeInfo::Tuple(node) => node.transform(tfmr),
            ast::TypeInfo::Union(node) => node.transform(tfmr),
        }
    }
}
