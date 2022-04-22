use super::*;
use crate::ast;
use ast::Node;

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

pub struct ParseTypeParameter;
parser_struct!(
    ParseTypeParameter,
    ast::TypeParameter,
    |_, state: &ParseState<'a>| {
        // That (`:` Thing)? = Here?
        let (state, name) = ParseName.parse(state)?;
        let start = name.span().start();
        let end = name.span().end();
        let (state, typ) =
            if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Colon).parse(&state) {
                let (new_state, info) = expect!(&new_state, ParseTypeInfo, "<type>");
                (new_state, Some(info))
            } else {
                (state, None)
            };
        let (state, default, end) =
            if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Equal).parse(&state) {
                let (new_state, default) = expect!(&new_state, ParseTypeInfo, "<type>");
                let span_end = default.span().end();
                (new_state, Some(default), span_end)
            } else {
                (
                    state,
                    None,
                    typ.as_ref().map(|v| v.span().end()).unwrap_or(end),
                )
            };
        Ok((
            state,
            ast::TypeParameter::new(ast::Span::new(start, end), name, typ, default),
        ))
    }
);

pub struct ParseTypeParameters;
parser_struct!(
    ParseTypeParameters,
    Vec<ast::TypeParameter>,
    |_, state: &ParseState<'a>| {
        // That (`:` Thing)? = Here?
        let (state, _) = ParseSymbol(ast::SymbolType::LessThan).parse(state)?;
        let (state, params) = OneOrMorePunctuated(
            ParseTypeParameter,
            ParseSymbol(ast::SymbolType::Comma),
            "<type>".to_string(),
        )
        .parse(&state)?;
        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::GreaterThan), ">");
        Ok((state, params))
    }
);

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
