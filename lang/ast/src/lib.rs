extern crate smol_str;

mod exprs;
mod stmts;
mod types;
mod visitors;

use std::fmt::Debug;

pub use exprs::*;
pub use stmts::*;
pub use types::*;
pub use visitors::*;

#[macro_use]
mod macros;

pub use salite_location::*;
pub use salite_tokens::*;
pub use salite_traits::SpannedNode;

use salite_macros::{CtorCall, FieldCall};

pub trait Node: Debug + Sync + Send {
    fn as_expr(&self) -> Option<Expr>;
    fn as_stmt(&self) -> Option<Stmt>;
}

#[derive(Debug, Clone, PartialEq, CtorCall, FieldCall)]
pub struct File {
    span: Span,
    block: Block,
}

pub fn filter_non_trivia_tokens(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .drain(..)
        .filter(|token| !token.ty().is_trivia())
        .collect()
}
