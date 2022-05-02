extern crate smol_str;

mod exprs;
mod stmts;
mod types;
mod visitors;

pub use exprs::*;
pub use stmts::*;
pub use types::*;
pub use visitors::*;

#[macro_use]
mod macros;

pub use lunar_location::*;
pub use lunar_tokens::*;
pub use lunar_traits::Node;

use lunar_macros::{CtorCall, FieldCall};

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
