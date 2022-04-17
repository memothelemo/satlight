mod exprs;

#[macro_use]
mod macros;

mod others;
mod stmts;
mod types;

pub use exprs::*;
pub use others::*;
pub use stmts::*;
pub use types::*;

mod prelude;
use prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseState<'a> {
    pub offset: usize,
    pub tokens: &'a [ast::Token],
}

impl<'a> ParseState<'a> {
    pub fn new(tokens: &'a [ast::Token]) -> Self {
        ParseState { offset: 0, tokens }
    }

    pub fn current(&self) -> Option<&'a ast::Token> {
        self.tokens.get(self.offset)
    }

    pub fn next(&self, offset: usize) -> ParseState<'a> {
        ParseState {
            offset: self.offset + offset,
            tokens: self.tokens,
        }
    }
}

pub type ParseResult<'a, T> = Result<(ParseState<'a>, T), ParseError>;

pub trait Parser<'a> {
    type Output: 'a;

    fn parse(&self, state: &ParseState<'a>) -> ParseResult<'a, Self::Output>;
}
