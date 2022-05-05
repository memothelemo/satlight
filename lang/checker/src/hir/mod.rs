mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

use salite_ast::Span;

#[derive(Debug)]
pub struct Block<'a> {
    pub span: Span,
    pub stmts: Vec<Stmt<'a>>,
    pub last_stmt: LastStmt<'a>,
}

#[derive(Debug)]
pub struct File<'a> {
    pub block: Block<'a>,
}
