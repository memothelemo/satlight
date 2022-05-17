use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::Binary {
    type Output = hir::Expr<'b>;

    #[allow(unused)]
    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        todo!()
    }
}

impl<'a, 'b> Transform<'a, 'b> for ast::Unary {
    type Output = hir::Expr<'b>;

    #[allow(unused)]
    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        todo!()
    }
}
