use crate::{binder::Symbol, types::Type};
use id_arena::Id;
use lunar_ast::{Node, Span};

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    TypeAssertion(TypeAssertion<'a>),
}

impl<'a> Expr<'a> {
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
pub struct Literal<'a> {
    pub typ: Type,
    pub symbol: Option<Id<Symbol>>,
    pub span: Span,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug)]
pub struct TypeAssertion<'a> {
    pub base: Box<Expr<'a>>,
    pub cast: Type,
    pub symbol: Option<Id<Symbol>>,
    pub span: Span,
    pub node_id: Id<&'a dyn Node>,
}
