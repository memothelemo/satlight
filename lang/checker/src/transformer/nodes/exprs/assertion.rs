use super::*;
use ast::SpannedNode;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeAssertion {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let typ = self.cast().transform(tfmr);
        hir::Expr::TypeAssertion(hir::TypeAssertion {
            base: Box::new(self.base().transform(tfmr)),
            cast: typ,
            span: self.span(),
            node_id: tfmr.ctx.nodes.alloc(self),
        })
    }
}
