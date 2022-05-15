use super::*;

mod assertion;
mod literal;

pub use assertion::*;
pub use literal::*;

impl<'a, 'b> Transform<'a, 'b> for ast::Expr {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self {
            ast::Expr::Binary(_) => todo!(),
            ast::Expr::Literal(node) => node.transform(tfmr),
            ast::Expr::Paren(node) => node.transform(tfmr),
            ast::Expr::Suffixed(_) => todo!(),
            ast::Expr::TypeAssertion(node) => node.transform(tfmr),
            ast::Expr::Unary(_) => todo!(),
        }
    }
}
