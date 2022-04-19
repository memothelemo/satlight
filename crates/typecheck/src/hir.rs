use crate::Type;
use id_arena::Id;
use lunar_macros::{HirNode, PropertyGetter};
use lunar_shared::Node;

pub trait HirNode<'a> {
    fn original(&self) -> Id<&'a dyn Node>;
}

pub trait HirExpr<'a> {
    fn ty(&self) -> Type;
}

#[derive(Debug, Clone, PartialEq, HirNode, PropertyGetter)]
pub struct Literal<'a> {
    pub(crate) ty: Type,
    original: Id<&'a dyn Node>,
}

impl<'a> HirExpr<'a> for Literal<'a> {
    fn ty(&self) -> Type {
        self.ty.clone()
    }
}

#[derive(Debug, Clone, PartialEq, HirNode, PropertyGetter)]
pub struct TypeAssertion<'a> {
    pub(crate) base: Box<Expr<'a>>,
    pub(crate) casted: Type,
    original: Id<&'a dyn Node>,
}

impl<'a> HirExpr<'a> for TypeAssertion<'a> {
    fn ty(&self) -> Type {
        self.casted.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    TypeAssertion(TypeAssertion<'a>),
}

impl<'a> From<Literal<'a>> for Expr<'a> {
    fn from(l: Literal<'a>) -> Self {
        Expr::Literal(l)
    }
}

impl<'a> From<TypeAssertion<'a>> for Expr<'a> {
    fn from(l: TypeAssertion<'a>) -> Self {
        Expr::TypeAssertion(l)
    }
}

impl<'a> HirNode<'a> for Expr<'a> {
    fn original(&self) -> Id<&'a dyn Node> {
        match self {
            Expr::Literal(node) => *node.original(),
            Expr::TypeAssertion(node) => *node.original(),
        }
    }
}

impl<'a> HirExpr<'a> for Expr<'a> {
    fn ty(&self) -> Type {
        match self {
            Expr::Literal(node) => node.ty.clone(),
            Expr::TypeAssertion(node) => node.ty(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct LocalAssignVariable<'a> {
    real_name: String,
    name_span: lunar_ast::Span,
    name_type: Option<Type>,
    type_span: Option<lunar_ast::Span>,
    expr: Option<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq, HirNode, PropertyGetter)]
pub struct LocalAssign<'a> {
    variables: Vec<LocalAssignVariable<'a>>,
    original: Id<&'a dyn Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    LocalAssign(LocalAssign<'a>),
}

impl<'a> HirNode<'a> for Stmt<'a> {
    fn original(&self) -> Id<&'a dyn Node> {
        match self {
            Stmt::LocalAssign(node) => node.original,
        }
    }
}

#[derive(Debug, Clone, PartialEq, HirNode, PropertyGetter)]
pub struct Block<'a> {
    stmts: Vec<Stmt<'a>>,
    original: Id<&'a dyn Node>,
    return_type: Type,
}
