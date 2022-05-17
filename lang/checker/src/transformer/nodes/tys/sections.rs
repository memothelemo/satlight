use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeUnion {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let mut members = Vec::new();
        for member in self.members().iter() {
            members.push(member.transform(tfmr));
        }
        Type::Union(variants::Union {
            span: self.span(),
            members,
        })
    }
}

impl<'a, 'b> Transform<'a, 'b> for ast::TypeIntersection {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let mut members = Vec::new();
        for member in self.members().iter() {
            members.push(member.transform(tfmr));
        }
        Type::Intersection(variants::Intersection {
            span: self.span(),
            members,
        })
    }
}
