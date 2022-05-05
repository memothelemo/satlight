use super::*;
use crate::ast;
use salite_traits::SpannedNode;

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
        ParseTypeCallback => ast::TypeInfo::Callback,
        ParseTypeReference => ast::TypeInfo::Reference,
        ParseTypeTable => ast::TypeInfo::Table,
    })
});

pub struct ParseTypeTableFieldNamed;
parser_struct!(
    ParseTypeTableFieldNamed,
    ast::Token,
    |_, state: &ParseState<'a>| {
        parse_either!(state, {
            ParseSymbol(ast::SymbolType::MetatableTag) => |e| e,
            ParseName => |e| e,
        })
    }
);

pub struct ParseTypeTableField;
parser_struct!(
    ParseTypeTableField,
    ast::TypeTableField,
    |_, state: &ParseState<'a>| {
        if let Ok((new_state, start)) = ParseSymbol(ast::SymbolType::OpenBracket).parse(state) {
            let (new_state, key) = expect!(&new_state, ParseTypeInfo, "<type>");
            let (new_state, _) =
                expect!(&new_state, ParseSymbol(ast::SymbolType::CloseBracket), "]");
            let (new_state, _) = expect!(&new_state, ParseSymbol(ast::SymbolType::Colon), ":");
            let (new_state, value) = expect!(&new_state, ParseTypeInfo, "<type>");
            return Ok((
                new_state,
                ast::TypeTableField::Computed {
                    span: ast::Span::new(start.span().start, value.span().end),
                    key: Box::new(key),
                    value,
                },
            ));
        } else if let Ok((new_state, index)) = ParseTypeTableFieldNamed.parse(state) {
            let start_span = index.span().start;
            if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Colon).parse(&new_state) {
                let (new_state, value) = expect!(&new_state, ParseTypeInfo, "<type>");
                return Ok((
                    new_state,
                    ast::TypeTableField::Named {
                        span: ast::Span::new(start_span, value.span().end),
                        name: index,
                        value,
                    },
                ));
            }
        }
        let (state, type_info) = ParseTypeInfo.parse(state)?;
        Ok((state, ast::TypeTableField::Array(type_info)))
    }
);

pub struct ParseTypeTable;
parser_struct!(
    ParseTypeTable,
    ast::TypeTable,
    |_, state: &ParseState<'a>| {
        // `{` [field, (field)*] `}`
        let (state, start_symbol) = ParseSymbol(ast::SymbolType::OpenCurly).parse(state)?;
        let start = start_symbol.span().start;
        let (state, fields) =
            ZeroOrMorePunctuatedTrailed(ParseTypeTableField, ParseSymbol(ast::SymbolType::Comma))
                .parse(&state)?;
        let (state, end_symbol) = expect!(&state, ParseSymbol(ast::SymbolType::CloseCurly), "}");
        Ok((
            state,
            ast::TypeTable::new(ast::Span::new(start, end_symbol.span().end), fields),
        ))
    }
);

pub struct ParseTypeParameter;
parser_struct!(
    ParseTypeParameter,
    ast::TypeParameter,
    |_, state: &ParseState<'a>| {
        // That (`:` Thing)? = Here?
        let (state, name) = ParseName.parse(state)?;
        let start = name.span().start;
        let end = name.span().end;
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
                let span_end = default.span().end;
                (new_state, Some(default), span_end)
            } else {
                (
                    state,
                    None,
                    typ.as_ref().map(|v| v.span().end).unwrap_or(end),
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

pub struct ParseTypeCallbackParameter;
parser_struct!(
    ParseTypeCallbackParameter,
    ast::TypeCallbackParameter,
    |_, state: &ParseState<'a>| {
        // check <name> `:`
        if let Ok((new_state, name)) = ParseName.parse(state) {
            if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Colon).parse(&new_state) {
                let (new_state, type_info) = expect!(&new_state, ParseTypeInfo, "<type>");
                return Ok((
                    new_state,
                    ast::TypeCallbackParameter::new(
                        ast::Span::new(name.span().start, type_info.span().end),
                        Some(name),
                        type_info,
                    ),
                ));
            }
        }
        let (state, type_info) = ParseTypeInfo.parse(state)?;
        Ok((
            state,
            ast::TypeCallbackParameter::new(type_info.span(), None, type_info),
        ))
    }
);

pub struct ParseTypeCallback;
parser_struct!(
    ParseTypeCallback,
    ast::TypeCallback,
    |_, state: &ParseState<'a>| {
        let (state, start_token) = ParseSymbol(ast::SymbolType::OpenParen).parse(state)?;
        let (state, params) = ZeroOrMorePunctuated(
            ParseTypeCallbackParameter,
            ParseSymbol(ast::SymbolType::Comma),
        )
        .parse(&state)?;
        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::CloseParen), ")");
        let (state, _) = ParseSymbol(ast::SymbolType::SkinnyArrow).parse(&state)?;
        let (state, return_type) = expect!(&state, ParseTypeInfo, "<type>");
        Ok((
            state,
            ast::TypeCallback::new(
                ast::Span::new(start_token.span().start, return_type.span().end),
                params,
                Box::new(return_type),
            ),
        ))
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
                (new_state, Some(collection), tko.span().end)
            } else {
                (state, None, name.span().end)
            };
        let name_span = name.span();
        Ok((
            state,
            ast::TypeReference::new(ast::Span::new(name_span.start, end_span), arguments, name),
        ))
    }
);
