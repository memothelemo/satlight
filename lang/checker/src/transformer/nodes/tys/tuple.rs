use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeTuple {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let mut members = Vec::new();
        for member in self.members().iter() {
            members.push(member.transform(tfmr));
        }
        Type::Tuple(variants::Tuple {
            span: self.span(),
            members,
        })
    }
}
