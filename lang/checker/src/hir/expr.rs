use crate::{binder::Symbol, types::Type};
use id_arena::Id;
use salite_ast::{Node, Span};

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    TypeAssertion(TypeAssertion<'a>),
    Table(Table<'a>),
}

impl<'a> Expr<'a> {
    pub fn typ(&self) -> &Type {
        match self {
            Expr::Literal(node) => &node.typ,
            Expr::TypeAssertion(node) => &node.cast,
            Expr::Table(node) => &node.typ,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(node) => node.span,
            Expr::TypeAssertion(node) => node.span,
            Expr::Table(node) => node.span,
        }
    }

    pub fn symbol(&self) -> Option<Id<Symbol>> {
        match self {
            Expr::Literal(node) => node.symbol,
            Expr::TypeAssertion(node) => node.symbol,
            Expr::Table(node) => node.symbol,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Literal<'a> {
    pub typ: Type,
    pub symbol: Option<Id<Symbol>>,
    pub span: Span,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug, Clone)]
pub struct TypeAssertion<'a> {
    pub base: Box<Expr<'a>>,
    pub cast: Type,
    pub symbol: Option<Id<Symbol>>,
    pub span: Span,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug, Clone)]
pub enum TableFieldKey<'a> {
    None,
    Name(String, Span),
    Computed(Expr<'a>),
}

#[derive(Debug, Clone)]
pub struct Table<'a> {
    pub typ: Type,
    pub node_id: Id<&'a dyn Node>,
    pub fields: Vec<(TableFieldKey<'a>, Expr<'a>)>,
    pub symbol: Option<Id<Symbol>>,
    pub span: Span,
}
