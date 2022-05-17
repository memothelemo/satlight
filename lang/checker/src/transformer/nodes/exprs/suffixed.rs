use self::call::visit_call_expr_inner;
use super::*;

mod call;

impl<'a, 'b> Transform<'a, 'b> for ast::Suffixed {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self.suffix() {
            ast::SuffixKind::Call(args) => visit_call_expr_inner(tfmr, self, args),
            ast::SuffixKind::Computed(_) => todo!(),
            ast::SuffixKind::Method(_) => todo!(),
            ast::SuffixKind::Name(_) => todo!(),
        }
    }
}
