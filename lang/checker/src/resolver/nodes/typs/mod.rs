use super::*;
use crate::types::Type;

impl<'a, 'b> Resolve<'a, 'b> for Type {
    type Output = Type;

    fn resolve(&self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        resolver.resolve_type(self)
    }
}
