use super::*;
use lunar_ast::Span;
use lunar_macros::PropertyGetter;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Assertion(Box<Expr>, Typ),
    Call(Box<Expr>, Vec<Expr>),
    Callback(Block, Typ),
    Error,
    Literal(Typ),
    Name(Typ),
}

impl ExprKind {
    #[inline]
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Error)
    }
}

#[derive(Debug, Clone, PropertyGetter)]
pub struct Expr {
    pub(crate) kind: ExprKind,
    pub(crate) span: Span,
}

impl Expr {
    pub fn err(span: Span) -> Self {
        Expr {
            kind: ExprKind::Error,
            span,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn literal(typ: Typ, span: Span) -> Self {
        Expr {
            kind: ExprKind::Literal(typ),
            span,
        }
    }

    pub fn is_error(&self) -> bool {
        self.kind.is_err()
    }
}

pub type ExprListMemberSource = (Expr, Typ, usize);

#[derive(Debug, Clone, PropertyGetter)]
pub struct LocalAssignVar {
    pub(crate) name: String,
    pub(crate) name_span: Span,
    pub(crate) explicit_type: Option<Typ>,
    pub(crate) expr: Option<ExprListMemberSource>,
}

#[derive(Debug, Clone, PropertyGetter)]
pub struct LocalAssign {
    pub(crate) span: Span,
    pub(crate) variables: Vec<LocalAssignVar>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Call(Expr),
    Error(Span),
    LocalAssign(LocalAssign),
}

impl Stmt {
    #[inline]
    pub fn error(span: Span) -> Stmt {
        Stmt::Error(span)
    }

    pub fn span(&self) -> Span {
        match self {
            Stmt::LocalAssign(node) => node.span,
            Stmt::Error(span) => *span,
            Stmt::Call(node) => node.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LastStmt {
    Break,
    Error(Span),
    Return(Vec<Expr>, Span),
}

impl LastStmt {
    pub fn get_return_exprs(&self) -> Option<&Vec<Expr>> {
        match self {
            LastStmt::Return(t, ..) => Some(t),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PropertyGetter)]
pub struct Block {
    pub(crate) expected_type: Option<Typ>,
    pub(crate) last_stmt: Option<LastStmt>,
    pub(crate) stmts: Vec<Stmt>,
    pub(crate) scope: Id<Scope>,
    pub(crate) span: Span,
}

impl Block {
    pub fn span(&self) -> Span {
        self.span
    }
}
