use super::*;
use crate::types::Type;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeInfo {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        todo!()
    }
}
