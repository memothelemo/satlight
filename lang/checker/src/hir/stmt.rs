use super::*;
use crate::{types::Type, Symbol};
use id_arena::Id;
use salite_ast::{Node, Span};

#[derive(Debug, Clone)]
pub enum LastStmt<'a> {
    None,
    Return(Return<'a>),
    Break(Span, Id<&'a dyn Node>),
}

#[derive(Debug, Clone)]
pub struct Return<'a> {
    pub concluding_typ: Type,
    pub exprs: Vec<Expr<'a>>,
    pub span: Span,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Call(Suffixed<'a>),
    Library(LibraryExpr<'a>),
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

impl PartialEq for TypeParameter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.explicit == other.explicit
            && self.default == other.default
            && self.name_span == other.name_span
            && self.span == other.span
    }
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration<'a> {
    pub name: String,
    pub parameters: Option<Vec<TypeParameter>>,
    pub value: Type,
    pub node_id: Id<&'a dyn Node>,
}

#[derive(Debug, Clone)]
pub struct LocalAssignVar {
    pub name: String,
    pub name_symbol: Id<Symbol>,
    pub name_span: Span,
    pub explicit_type: Option<Type>,
    pub expr_source: Option<Span>,
    pub expr_id: usize,
    pub expr: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct LocalAssign<'a> {
    pub variables: Vec<LocalAssignVar>,
    pub span: Span,
    pub exprs: Vec<Expr<'a>>,
    pub node_id: Id<&'a dyn Node>,
}
