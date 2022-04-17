extern crate lunar_span;
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
mod tokens;

pub use lunar_span::{Position, Span};
pub use tokens::*;

pub fn filter_non_trivia_tokens(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .drain(..)
        .filter(|token| !token.ty().is_trivia())
        .collect()
}
