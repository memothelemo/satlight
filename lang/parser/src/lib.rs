mod exprs;

#[macro_use]
mod macros;

mod others;
mod stmts;
mod types;

pub use exprs::*;
use lunar_traits::SpannedNode;
pub use others::*;
pub use stmts::*;
pub use types::*;

mod prelude;
use prelude::*;

pub use lunar_common::errors::parser::*;

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

/// Parses into an AST file with state required for manual flexibility
pub fn parse_file_raw(state: &ParseState<'_>) -> Result<ast::File, ParseError> {
    match ParseBlock.parse(state) {
        Ok((_, block)) => Ok(ast::File::new(block.span(), block)),
        Err(err) => Err(err),
    }
}

/// Parses into an AST file with tokens required for complete parsing
pub fn parse_file(tokens: &[ast::Token]) -> Result<ast::File, ParseError> {
    let state = ParseState::new(tokens);
    parse_file_raw(&state)
}
