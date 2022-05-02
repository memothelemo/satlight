use crate::{binder::Symbol, types::Type};
use id_arena::Id;
use lunar_ast::Span;

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    TypeAssertion(TypeAssertion),
}

impl Expr {
    pub fn typ(&self) -> &Type {
        match self {
            Expr::Literal(node) => &node.typ,
            Expr::TypeAssertion(node) => &node.cast,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(node) => node.span,
            Expr::TypeAssertion(node) => node.span,
        }
    }

    pub fn symbol(&self) -> Option<Id<Symbol>> {
        match self {
            Expr::Literal(node) => node.symbol,
            Expr::TypeAssertion(node) => node.symbol,
        }
    }
}

#[derive(Debug)]
pub struct Literal {
    pub typ: Type,
    pub symbol: Option<Id<Symbol>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct TypeAssertion {
    pub base: Box<Expr>,
    pub cast: Type,
    pub symbol: Option<Id<Symbol>>,
    pub span: Span,
}
