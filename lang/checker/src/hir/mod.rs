mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

use crate::types::Type;
use salite_ast::Span;

#[derive(Debug, Clone)]
pub struct Block<'a> {
    pub span: Span,
    pub stmts: Vec<Stmt<'a>>,
    pub last_stmt: LastStmt<'a>,
    pub actual_type: Type,
    pub expected_type: Option<Type>,
}

#[derive(Debug)]
pub struct File<'a> {
    pub block: Block<'a>,
}
