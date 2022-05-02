use super::*;
use crate::{binder::Symbol, types::Type};
use id_arena::Id;
use lunar_ast::Span;

#[derive(Debug)]
pub enum LastStmt {
    None,
    Return(Return),
    Break(Span),
}

#[derive(Debug)]
pub struct Return {
    pub exprs: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug)]
pub enum Stmt {
    LocalAssign(LocalAssign),
}

#[derive(Debug)]
pub struct LocalAssignVar {
    pub name: String,
    pub name_symbol: Id<Symbol>,
    pub name_span: Span,
    pub explicit_type: Option<Type>,
    pub expr_source: Option<Span>,
    pub expr: Option<Type>,
}

#[derive(Debug)]
pub struct LocalAssign {
    pub variables: Vec<LocalAssignVar>,
    pub span: Span,
}
