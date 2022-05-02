mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

use lunar_ast::Span;

#[derive(Debug)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub last_stmt: LastStmt,
}
