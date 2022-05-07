#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;

use salite_location::Span;
use salite_macros::{CtorCall, FieldCall};
use salite_traits::SpannedNode;

mod op;
pub use op::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Paren(Box<Expr>),
    Suffixed(Suffixed),
    TypeAssertion(TypeAssertion),
    Unary(Unary),
}

impl Node for Expr {
    fn as_expr(&self) -> Option<Expr> {
        match self {
            Expr::Binary(node) => node.as_expr(),
            Expr::Literal(node) => node.as_expr(),
            Expr::Paren(node) => node.as_expr(),
            Expr::Suffixed(node) => node.as_expr(),
            Expr::TypeAssertion(node) => node.as_expr(),
            Expr::Unary(node) => node.as_expr(),
        }
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Binary(node) => node.span(),
            Expr::Literal(node) => node.span(),
            Expr::Paren(node) => node.span(),
            Expr::Suffixed(node) => node.span(),
            Expr::TypeAssertion(node) => node.span(),
            Expr::Unary(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct TypeAssertion {
    base: Box<Expr>,
    cast: TypeInfo,
}

impl Node for TypeAssertion {
    fn as_expr(&self) -> Option<Expr> {
        Some(Expr::TypeAssertion(self.clone()))
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for TypeAssertion {
    fn span(&self) -> Span {
        Span::merge(self.base.span(), self.cast.span())
    }
}

pub type ExprList = Vec<Expr>;

/// Due to the limitations of implementing traits with a type
/// that is not belonged to a crate.
///
/// This function will help, but it will return as `Span(0,0)` if
/// the vector is empty.
pub fn vector_span<N: SpannedNode>(vec: &[N]) -> Span {
    let first = vec.first().map(|v| v.span().start).unwrap_or(0);
    let last = vec.last().map(|v| v.span().start).unwrap_or(0);
    Span::new(first, last)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Args {
    ExprList(ExprList),
    Table(TableCtor),
    Str(Token),
}

impl SpannedNode for Args {
    fn span(&self) -> Span {
        match self {
            Args::ExprList(node) => vector_span(node),
            Args::Table(node) => node.span(),
            Args::Str(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(Token),
    Function(FunctionExpr),
    Name(Token),
    Number(Token),
    Nil(Token),
    Str(Token),
    Table(TableCtor),
    Varargs(Token),
}

impl Node for Literal {
    fn as_expr(&self) -> Option<Expr> {
        Some(Expr::Literal(self.clone()))
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for Literal {
    fn span(&self) -> Span {
        match self {
            Literal::Bool(node) => node.span(),
            Literal::Function(node) => node.span(),
            Literal::Name(node) => node.span(),
            Literal::Number(node) => node.span(),
            Literal::Nil(node) => node.span(),
            Literal::Str(node) => node.span(),
            Literal::Table(node) => node.span(),
            Literal::Varargs(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SuffixKind {
    Call(Args),
    Computed(Box<Expr>),
    Method(Token),
    Name(Token),
}

impl SpannedNode for SuffixKind {
    fn span(&self) -> Span {
        match self {
            SuffixKind::Call(node) => node.span(),
            SuffixKind::Computed(node) => node.span(),
            SuffixKind::Method(node) => node.span(),
            SuffixKind::Name(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct Suffixed {
    base: Box<Expr>,
    suffix_span: Span,
    suffix: SuffixKind,
}

impl Node for Suffixed {
    fn as_expr(&self) -> Option<Expr> {
        Some(Expr::Suffixed(self.clone()))
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for Suffixed {
    fn span(&self) -> Span {
        Span::merge(self.base.span(), self.suffix_span)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TableField {
    Array(Box<Expr>),
    Named {
        span: Span,
        name: Token,
        value: Box<Expr>,
    },
    Expr {
        span: Span,
        index: Box<Expr>,
        value: Box<Expr>,
    },
}

impl SpannedNode for TableField {
    fn span(&self) -> Span {
        match self {
            TableField::Array(exp) => exp.span(),
            TableField::Expr { span, .. } => *span,
            TableField::Named { span, .. } => *span,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct TableCtor {
    #[exclude]
    span: Span,
    fields: Vec<TableField>,
}

impl Node for TableCtor {
    fn as_expr(&self) -> Option<Expr> {
        Some(Expr::Literal(Literal::Table(self.clone())))
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for TableCtor {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct Binary {
    left: Box<Expr>,
    op: Binop,
    right: Box<Expr>,
}

impl Node for Binary {
    fn as_expr(&self) -> Option<Expr> {
        Some(Expr::Binary(self.clone()))
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for Binary {
    fn span(&self) -> Span {
        Span::merge(self.left.span(), self.right.span())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct Unary {
    op: Unop,
    expr: Box<Expr>,
}

impl Node for Unary {
    fn as_expr(&self) -> Option<Expr> {
        Some(Expr::Unary(self.clone()))
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for Unary {
    fn span(&self) -> Span {
        Span::merge(self.op.token.span(), self.expr.span())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct Param {
    pub span: Span,
    pub name: Token,
    pub optional: bool,
    pub explicit_type: Option<TypeInfo>,
    pub default: Option<Expr>,
}

impl SpannedNode for Param {
    fn span(&self) -> salite_location::Span {
        self.span
    }
}

pub type ParamList = Vec<Param>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct VaridiacParam {
    pub span: Span,
    pub typ: Option<TypeInfo>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct FunctionBody {
    #[exclude]
    span: Span,
    params: ParamList,
    varidiac: Option<VaridiacParam>,
    block: Block,
    return_type: Option<TypeInfo>,
}

impl SpannedNode for FunctionBody {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct FunctionExpr {
    #[exclude]
    span: Span,
    body: FunctionBody,
}

impl Node for FunctionExpr {
    fn as_expr(&self) -> Option<Expr> {
        Some(Expr::Literal(Literal::Function(self.clone())))
    }

    fn as_stmt(&self) -> Option<Stmt> {
        None
    }
}

impl SpannedNode for FunctionExpr {
    fn span(&self) -> Span {
        self.span
    }
}
