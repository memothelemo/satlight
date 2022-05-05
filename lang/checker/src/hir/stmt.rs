use super::*;
use crate::{binder::Symbol, types::Type};
use id_arena::Id;
use salite_ast::{Node, Span};

#[derive(Debug)]
pub enum LastStmt<'a> {
    None,
    Return(Return<'a>),
    Break(Span, Id<&'a dyn Node>),
}

#[derive(Debug)]
pub struct Return<'a> {
    pub exprs: Vec<Expr<'a>>,
    pub span: Span,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug)]
pub enum Stmt<'a> {
    LocalAssign(LocalAssign<'a>),
    TypeDeclaration(TypeDeclaration<'a>),
}

#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub name: String,
    pub explicit: Option<Type>,
    pub default: Option<Type>,
    pub name_span: Span,
    pub span: Span,
}

#[derive(Debug)]
pub struct TypeDeclaration<'a> {
    pub name: String,
    pub parameters: Option<Vec<TypeParameter>>,
    pub value: Type,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug)]
pub struct LocalAssignVar {
    pub name: String,
    pub name_symbol: Id<Symbol>,
    pub name_span: Span,
    pub explicit_type: Option<Type>,
    pub expr_source: Option<Span>,
    pub expr_id: usize,
    pub expr: Option<Type>,
}

#[derive(Debug)]
pub struct LocalAssign<'a> {
    pub variables: Vec<LocalAssignVar>,
    pub span: Span,
    pub exprs: Vec<Expr<'a>>,
    pub node_id: Id<&'a dyn Node>,
}
