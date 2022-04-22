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

mod span;
pub use span::*;

pub use tokens::*;

pub trait Node {
    fn span(&self) -> Span;
}

pub fn filter_non_trivia_tokens(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .drain(..)
        .filter(|token| !token.ty().is_trivia())
        .collect()
}
