use super::*;

mod arys;
mod assertion;

#[macro_use]
mod literal;
mod suffixed;

pub use arys::*;
pub use assertion::*;

#[allow(unused)]
pub use literal::*;
pub use suffixed::*;

impl<'a, 'b> Transform<'a, 'b> for ast::Expr {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self {
            ast::Expr::Binary(node) => node.transform(tfmr),
            ast::Expr::Literal(node) => node.transform(tfmr),
            ast::Expr::Paren(node) => node.transform(tfmr),
            ast::Expr::Suffixed(node) => node.transform(tfmr),
            ast::Expr::TypeAssertion(node) => node.transform(tfmr),
            ast::Expr::Unary(node) => node.transform(tfmr),
        }
    }
}
