use super::*;
use crate::types::Type;
use id_arena::Id;
use salite_ast::{Node, Span};

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Function(Function<'a>),
    Literal(Literal<'a>),
    TypeAssertion(TypeAssertion<'a>),
    Table(Table<'a>),
}

impl<'a> Expr<'a> {
    pub fn typ(&self) -> &Type {
        match self {
            Expr::Function(node) => &node.typ,
            Expr::Literal(node) => &node.typ,
            Expr::TypeAssertion(node) => &node.cast,
            Expr::Table(node) => &node.typ,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Expr::Function(node) => node.span,
            Expr::Literal(node) => node.span,
            Expr::TypeAssertion(node) => node.span,
            Expr::Table(node) => node.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub span: Span,
    pub name: String,
    pub typ: Type,
    pub default: bool,
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub span: Span,
    pub typ: Type,
    pub defaults: Vec<Option<Expr<'a>>>,
    pub block: Block<'a>,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug, Clone)]
pub struct Literal<'a> {
    pub span: Span,
    pub typ: Type,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug, Clone)]
pub struct TypeAssertion<'a> {
    pub base: Box<Expr<'a>>,
    pub cast: Type,
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
    pub span: Span,
}
