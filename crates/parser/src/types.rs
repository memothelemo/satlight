use super::*;
use crate::ast;

use crate::{expect, parse_either, parser_struct};

pub struct ParseTypeArguments;
parser_struct!(
    ParseTypeArguments,
    Vec<ast::TypeInfo>,
    |_, state: &ParseState<'a>| {
        OneOrMorePunctuated(
            ParseTypeInfo,
            ParseSymbol(ast::SymbolType::Comma),
            "<type>".into(),
        )
        .parse(state)
    }
);

pub struct ParseTypeInfo;
parser_struct!(ParseTypeInfo, ast::TypeInfo, |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseTypeReference => ast::TypeInfo::Reference,
    })
});

pub struct ParseTypeReference;
parser_struct!(
    ParseTypeReference,
    ast::TypeReference,
    |_, state: &ParseState<'a>| {
        let (state, name) = ParseName.parse(state)?;
        let (state, arguments, end_span) =
            if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::LessThan).parse(&state) {
                let (new_state, collection) = ParseTypeArguments.parse(&new_state)?;
                let (new_state, tko) =
                    expect!(&new_state, ParseSymbol(ast::SymbolType::GreaterThan), ">");
                (new_state, Some(collection), tko.span().end())
            } else {
                (state, None, name.span().end())
            };
        let name_span = name.span();
        Ok((
            state,
            ast::TypeReference::new(ast::Span::new(name_span.start(), end_span), arguments, name),
        ))
    }
);
