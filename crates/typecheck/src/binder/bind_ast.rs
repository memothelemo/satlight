use super::*;
use lunar_ast::Span;
use lunar_macros::PropertyGetter;

#[derive(Debug)]
pub enum ExprKind {
    Assertion(Box<Expr>, Typ),
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

#[derive(Debug, PropertyGetter)]
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

    pub fn typ(&self) -> Typ {
        match self.kind {
            ExprKind::Assertion(_, ty) => ty,
            ExprKind::Error => {
                unimplemented!("'Error' expression kind is not allowed, evaluate first.")
            }
            ExprKind::Literal(ty) => ty,
            ExprKind::Name(ty) => ty,
        }
    }
}

#[derive(Debug, PropertyGetter)]
pub struct LocalAssignVar {
    pub(crate) name: String,
    pub(crate) name_span: Span,
    pub(crate) explicit_type: Option<Typ>,
    pub(crate) expr: Option<Expr>,
}

#[derive(Debug, PropertyGetter)]
pub struct LocalAssign {
    pub(crate) span: Span,
    pub(crate) variables: Vec<LocalAssignVar>,
}

#[derive(Debug)]
pub enum Stmt {
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
        }
    }
}

#[derive(Debug)]
pub enum LastStmt {
    Break,
    Error(Span),
}

#[derive(Debug, PropertyGetter)]
pub struct Block {
    pub(crate) last_stmt: Option<LastStmt>,
    pub(crate) stmts: Vec<Stmt>,
    pub(crate) span: Span,
}

impl Block {
    pub fn span(&self) -> Span {
        self.span
    }
}
