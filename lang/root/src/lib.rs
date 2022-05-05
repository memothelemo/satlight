#[cfg(any(feature = "ast", feature = "full"))]
pub use salite_ast as ast;

#[cfg(any(feature = "checker", feature = "full"))]
pub use salite_checker as checker;

#[cfg(any(feature = "common", feature = "full"))]
pub use salite_common as common;

#[cfg(any(feature = "macros", feature = "full"))]
pub use salite_macros as macros;

#[cfg(any(feature = "parser", feature = "full"))]
pub use salite_parser as parser;

#[cfg(any(feature = "tokenizer", feature = "full"))]
pub use salite_tokenizer as tokenizer;

#[cfg(any(feature = "lazy_parse", feature = "full"))]
use salite_common::errors;

#[cfg(any(feature = "lazy_parse", feature = "full"))]
#[derive(Debug)]
pub enum LazyParseError {
    ParseError(errors::parser::ParseError),
    TokenizeError(errors::tokenizer::TokenizeError),
}

#[cfg(any(feature = "lazy_parse", feature = "full"))]
impl LazyParseError {
    pub fn span(&self) -> ast::Span {
        match self {
            LazyParseError::ParseError(err) => err.span,
            LazyParseError::TokenizeError(err) => err.span,
        }
    }
}

#[cfg(any(feature = "lazy_parse", feature = "full"))]
impl errors::SaliteError for LazyParseError {
    fn message(&self, code: &str) -> Result<String, errors::TextSpanOutOfBounds> {
        match self {
            LazyParseError::ParseError(err) => err.message(code),
            LazyParseError::TokenizeError(err) => err.message(code),
        }
    }
}

/// Parses any file with a source code parameter accepted. This is meant
/// to lazily parse without any setups required.
#[cfg(any(feature = "lazy_parse", feature = "full"))]
pub fn lazy_parse(input: &str) -> Result<ast::File, LazyParseError> {
    let tokens = tokenizer::tokenize(input).map_err(LazyParseError::TokenizeError)?;
    let tokens = ast::filter_non_trivia_tokens(tokens);
    parser::parse_file(&tokens).map_err(LazyParseError::ParseError)
}
