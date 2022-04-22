use super::*;

use crate::{expect, no_match_ignore, optional, parse_either, parser_struct};
use lunar_ast::Node;

pub struct ParseToken(pub ast::TokenType);
parser_struct!(
    ParseToken,
    ast::Token,
    |this: &ParseToken, state: &ParseState<'a>| {
        if let Some(token) = state.current() {
            if token.ty() == &this.0 {
                return Ok((state.next(1), token.clone()));
            }
        }
        no_match!(state)
    }
);

pub struct ParseSymbol(pub ast::SymbolType);
parser_struct!(
    ParseSymbol,
    ast::Token,
    |this: &ParseSymbol, state: &ParseState<'a>| {
        if let Some(token) = state.current() {
            if token.ty() == &ast::TokenType::Symbol(this.0.clone()) {
                return Ok((state.next(1), token.clone()));
            }
        }
        no_match!(state)
    }
);

pub struct ParseName;
parser_struct!(ParseName, ast::Token, |_, state: &ParseState<'a>| {
    if let Some(token) = state.current() {
        if token.kind() == ast::TokenKind::Identifier {
            return Ok((state.next(1), token.clone()));
        }
    }
    no_match!(state)
});

pub struct ParseNumber;
parser_struct!(ParseNumber, ast::Token, |_, state: &ParseState<'a>| {
    if let Some(token) = state.current() {
        if token.kind() == ast::TokenKind::Number {
            return Ok((state.next(1), token.clone()));
        }
    }
    no_match!(state)
});

pub struct ParseStr;
parser_struct!(ParseStr, ast::Token, |_, state: &ParseState<'a>| {
    if let Some(token) = state.current() {
        if token.kind() == ast::TokenKind::Str {
            return Ok((state.next(1), token.clone()));
        }
    }
    no_match!(state)
});

pub struct ParseTableFieldSep;
parser_struct!(ParseTableFieldSep, (), |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseSymbol(ast::SymbolType::Semicolon) => |_| (),
        ParseSymbol(ast::SymbolType::Comma) => |_| (),
    })
});

pub struct ParseTableField;
#[rustfmt::skip]
parser_struct!(ParseTableField, ast::TableField, |_, state: &ParseState<'a>| {
    if let Ok((ns, tok)) = ParseSymbol(ast::SymbolType::OpenBracket).parse(state) {
        let start_span = tok.span().start();
        let (ns, index) = expect!(&ns, ParseExpr, "<exp>");
        let (ns, _) = expect!(&ns, ParseSymbol(ast::SymbolType::CloseBracket), "]");
        let (ns, _) = expect!(&ns, ParseSymbol(ast::SymbolType::Equal), "=");
        let (ns, value) = expect!(&ns, ParseExpr, "<exp>");
        return Ok((
            ns,
            ast::TableField::Expr {
                span: ast::Span::new(start_span, value.span().end()),
                index: Box::new(index),
                value: Box::new(value),
            },
        ));
    } else if let Ok((ns, index)) = ParseName.parse(state) {
        let start_span = index.span().start();
        if let Ok((ns, _)) = ParseSymbol(ast::SymbolType::Equal).parse(&ns) {
            let (ns, value) = expect!(&ns, ParseExpr, "<exp>");
            return Ok((
                ns,
                ast::TableField::Expr {
                    span: ast::Span::new(start_span, value.span().end()),
                    index: Box::new(ast::Expr::Literal(ast::Literal::Name(index))),
                    value: Box::new(value),
                },
            ));
        }
    }
    let (ns, exp) = ParseExpr.parse(state)?;
    Ok((ns, ast::TableField::Array(Box::new(exp))))
});

pub struct ParseTableCtor;
#[rustfmt::skip]
parser_struct!(ParseTableCtor, ast::TableCtor, |_, state: &ParseState<'a>| {
    let (state, start) = ParseSymbol(ast::SymbolType::OpenCurly).parse(state)?;
    let (state, fields) = ZeroOrMorePunctuatedTrailed(
        ParseTableField,
        ParseTableFieldSep
    ).parse(&state)?;
    let (state, end) = expect!(&state, ParseSymbol(ast::SymbolType::CloseCurly), "}");
    Ok((state, ast::TableCtor::new(
        ast::Span::from_two_spans(
            start.span(),
            end.span()
        ),
        fields,
    )))
});

pub struct ParseLiteral;
parser_struct!(ParseLiteral, ast::Literal, |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseSymbol(ast::SymbolType::False) => ast::Literal::Bool,
        ParseSymbol(ast::SymbolType::True) => ast::Literal::Bool,
        ParseFunctionExpr => ast::Literal::Function,
        ParseName => ast::Literal::Name,
        ParseNumber => ast::Literal::Number,
        ParseSymbol(ast::SymbolType::Nil) => ast::Literal::Nil,
        ParseStr => ast::Literal::Str,
        ParseSymbol(ast::SymbolType::TripleDot) => ast::Literal::Varargs,
        ParseTableCtor => ast::Literal::Table,
    })
});

pub struct ParseParen;
parser_struct!(ParseParen, ast::Expr, |_, state: &ParseState<'a>| {
    let (state, _) = ParseSymbol(ast::SymbolType::OpenParen).parse(state)?;
    let (state, exp) = expect!(&state, ParseExpr, "<exp>");
    let (state, _) = ParseSymbol(ast::SymbolType::CloseParen).parse(&state)?;
    Ok((state, exp))
});

pub struct ParseSimpleExpr;
parser_struct!(ParseSimpleExpr, ast::Expr, |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseLiteral => ast::Expr::Literal,
        ParseParen => |e| ast::Expr::Paren(Box::new(e)),
    })
});

pub struct ParseExprList;
parser_struct!(ParseExprList, ast::ExprList, |_, state: &ParseState<'a>| {
    let mut collection = Vec::new();
    let mut state = state.next(0);
    loop {
        match ParseExpr.parse(&state) {
            Ok((ns, member)) => {
                let (ns, punct) = optional!(&ns, ParseSymbol(ast::SymbolType::Comma));
                let has_punct = punct.is_some();
                let is_varargs = matches!(member, ast::Expr::Literal(ast::Literal::Varargs(..)));
                state = ns;
                collection.push(member);
                if is_varargs || !has_punct {
                    break;
                }
            }
            Err(ParseError {
                ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                ..
            }) => break,
            Err(err) => return Err(err),
        }
    }
    Ok((state, collection))
});

pub struct ParseExprListRequired;
parser_struct!(
    ParseExprListRequired,
    ast::ExprList,
    |_, state: &ParseState<'a>| {
        let mut collection = Vec::new();
        let state = state.next(0);

        // expect something the first one there
        let (mut state, member_0) = expect!(&state, ParseExpr, "<exp>");
        collection.push(member_0);

        if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Comma).parse(&state) {
            state = new_state;
            loop {
                match ParseExpr.parse(&state) {
                    Ok((ns, member)) => {
                        let (ns, punct) = optional!(&ns, ParseSymbol(ast::SymbolType::Comma));
                        let has_punct = punct.is_some();
                        let is_varargs =
                            matches!(member, ast::Expr::Literal(ast::Literal::Varargs(..)));
                        state = ns;
                        collection.push(member);
                        if is_varargs || !has_punct {
                            break;
                        }
                    }
                    Err(ParseError {
                        ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                        ..
                    }) => break,
                    Err(err) => return Err(err),
                }
            }
        }

        Ok((state, collection))
    }
);

pub struct ParseArgs;
parser_struct!(ParseArgs, ast::Args, |_, state: &ParseState<'a>| {
    if let Ok((state, _)) = ParseSymbol(ast::SymbolType::OpenParen).parse(state) {
        let (state, args) = ParseExprList.parse(&state)?;
        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::CloseParen), ")");
        Ok((state, ast::Args::ExprList(args)))
    } else if let Ok((state, ctor)) = ParseTableCtor.parse(state) {
        Ok((state, ast::Args::Table(ctor)))
    } else if let Ok((state, str)) = ParseStr.parse(state) {
        Ok((state, ast::Args::Str(str)))
    } else {
        no_match!(state)
    }
});

pub struct ParseSuffixKind;
#[rustfmt::skip]
parser_struct!(ParseSuffixKind, ast::SuffixKind, |_,state: &ParseState<'a>| {
    if let Ok((state, _)) = ParseSymbol(ast::SymbolType::OpenBracket).parse(state) {
        let (state, exp) = expect!(&state, ParseExpr, "<exp>");
        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::CloseBracket), "]");
        Ok((state, ast::SuffixKind::Computed(Box::new(exp))))
    } else if let Ok((state, _)) = ParseSymbol(ast::SymbolType::Colon).parse(state) {
        let (state, name) = expect!(&state, ParseName, "<id>");
        Ok((state, ast::SuffixKind::Method(name)))
    } else if let Ok((state, _)) = ParseSymbol(ast::SymbolType::Dot).parse(state) {
        let (state, name) = expect!(&state, ParseName, "<id>");
        Ok((state, ast::SuffixKind::Name(name)))
    } else {
        let (state, args) = ParseArgs.parse(state)?;
        Ok((state, ast::SuffixKind::Call(args)))
    }
});

pub struct ParsePresuffixExpr;
#[rustfmt::skip]
parser_struct!(ParsePresuffixExpr, ast::Expr, |_, state: &ParseState<'a>| {
    let (mut state, mut base) = ParseSimpleExpr.parse(state)?;
    let start_suffix_span = state.current().unwrap().span().start();
    while let Ok((ns, suffix)) = no_match_ignore!(&state, ParseSuffixKind) {
        let end_suffix = suffix.span().end();
        base = ast::Expr::Suffixed(ast::Suffixed::new(
            Box::new(base),
            ast::Span::new(start_suffix_span, end_suffix),
            suffix,
        ));
        state = ns;
    }
    // check for method access and throw an error!
    if let ast::Expr::Suffixed(ref s) = base
    {
        if matches!(s.suffix(), ast::SuffixKind::Method(_)) {
            no_match!(state)
        }
    }
    Ok((state, base))
});

pub struct ParseBinopKind;
parser_struct!(
    ParseBinopKind,
    ast::BinopKind,
    |_, state: &ParseState<'a>| {
        parse_either!(state, {
            // NilshCoalescing
            ParseSymbol(ast::SymbolType::Caret) => |_| ast::BinopKind::Exponent,
            ParseSymbol(ast::SymbolType::Asterisk) => |_| ast::BinopKind::Multiply,
            // FloorDivision
            ParseSymbol(ast::SymbolType::Slash) => |_| ast::BinopKind::Divide,
            ParseSymbol(ast::SymbolType::Percent) => |_| ast::BinopKind::Modulo,
            ParseSymbol(ast::SymbolType::Cross) => |_| ast::BinopKind::Add,
            ParseSymbol(ast::SymbolType::Dash) => |_| ast::BinopKind::Subtract,
            ParseSymbol(ast::SymbolType::DoubleDot) => |_| ast::BinopKind::Concat,
            ParseSymbol(ast::SymbolType::DoubleEqual) => |_| ast::BinopKind::Equality,
            ParseSymbol(ast::SymbolType::TildeEqual) => |_| ast::BinopKind::Inequality,
            ParseSymbol(ast::SymbolType::GreaterThan) => |_| ast::BinopKind::GreaterThan,
            ParseSymbol(ast::SymbolType::GreaterEqual) => |_| ast::BinopKind::GreaterEqual,
            ParseSymbol(ast::SymbolType::LessThan) => |_| ast::BinopKind::LessThan,
            ParseSymbol(ast::SymbolType::LessEqual) => |_| ast::BinopKind::LessEqual,
            ParseSymbol(ast::SymbolType::GreaterThan) => |_| ast::BinopKind::GreaterThan,
            ParseSymbol(ast::SymbolType::And) => |_| ast::BinopKind::And,
            ParseSymbol(ast::SymbolType::Or) => |_| ast::BinopKind::Or,
        })
    }
);

pub struct ParseBinop;
parser_struct!(ParseBinop, ast::Binop, |_, state: &ParseState<'a>| {
    // to preserve the token
    let token = state.current();
    let (state, kind) = ParseBinopKind.parse(state)?;
    let token = token.unwrap();
    Ok((
        state,
        ast::Binop {
            kind,
            token: token.clone(),
        },
    ))
});

pub struct ParseUnopKind;
parser_struct!(ParseUnopKind, ast::UnopKind, |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseSymbol(ast::SymbolType::Hash) => |_| ast::UnopKind::Length,
        ParseSymbol(ast::SymbolType::Not) => |_| ast::UnopKind::Not,
        ParseSymbol(ast::SymbolType::Dash) => |_| ast::UnopKind::Negate,
    })
});

pub struct ParseUnop;
parser_struct!(ParseUnop, ast::Unop, |_, state: &ParseState<'a>| {
    // to preserve the token
    let token = state.current();
    let (state, kind) = ParseUnopKind.parse(state)?;
    let token = token.unwrap();
    Ok((
        state,
        ast::Unop {
            kind,
            token: token.clone(),
        },
    ))
});

pub struct ParseUnary;
parser_struct!(ParseUnary, ast::Unary, |_, state: &ParseState<'a>| {
    let (state, op) = ParseUnop.parse(state)?;
    let (state, expr) = expect!(
        &state,
        ParseExprWithPrecedence(op.kind.precedence()),
        "<exp>"
    );
    Ok((state, ast::Unary::new(op, Box::new(expr))))
});

pub struct ParseExprWithNoTypeAssertion;
parser_struct!(
    ParseExprWithNoTypeAssertion,
    ast::Expr,
    |_, state: &ParseState<'a>| {
        parse_either!(state, {
            ParsePresuffixExpr => |e| e,
            ParseUnary => ast::Expr::Unary,
        })
    }
);

pub struct ParseExprWithNoPrecedence;
parser_struct!(
    ParseExprWithNoPrecedence,
    ast::Expr,
    |_, state: &ParseState<'a>| {
        let (mut state, mut expr) = ParseExprWithNoTypeAssertion.parse(state)?;
        while let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::DoubleColon).parse(&state) {
            let (new_state, tt) = expect!(&new_state, ParseTypeInfo, "<type>".to_string());
            state = new_state;
            expr = ast::Expr::TypeAssertion(ast::TypeAssertion::new(Box::new(expr), tt));
        }
        Ok((state, expr))
    }
);

pub struct ParseExprWithPrecedence(usize);
parser_struct!(
    ParseExprWithPrecedence,
    ast::Expr,
    |this: &ParseExprWithPrecedence, state: &ParseState<'a>| {
        let (mut state, mut base) = ParseExprWithNoPrecedence.parse(state)?;
        while let Ok((ns, op)) = ParseBinop.parse(&state) {
            let pred = op.kind.precedence();
            if pred < this.0 {
                break;
            }

            let next_pred = if op.kind.is_right_associative() {
                pred + 1
            } else {
                pred
            };

            let (ns, right) = expect!(&ns, ParseExprWithPrecedence(next_pred), "<exp>");
            state = ns;
            base = ast::Expr::Binary(ast::Binary::new(Box::new(base), op, Box::new(right)));
        }
        Ok((state, base))
    }
);

struct ParseNameAsParam;
parser_struct!(ParseNameAsParam, ast::Param, |_, state: &ParseState<'a>| {
    let (state, name) = ParseName.parse(state)?;
    Ok((state, ast::Param::Name(name)))
});

pub struct ParseFunctionBody;
parser_struct!(
    ParseFunctionBody,
    ast::FunctionBody,
    |_, state: &ParseState<'a>| {
        let (state, open) = expect!(state, ParseSymbol(ast::SymbolType::OpenParen), "(");
        let start_span = open.span().start();

        // parse names?
        let (mut state, mut params) =
            ZeroOrMorePunctuated(ParseNameAsParam, ParseSymbol(ast::SymbolType::Comma))
                .parse(&state)?;
        if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Comma).parse(&state) {
            if let Ok((new_state, varargs)) =
                ParseSymbol(ast::SymbolType::TripleDot).parse(&new_state)
            {
                params.push(ast::Param::Varargs(varargs));
                state = new_state;
            }
        } else if let Ok((new_state, varargs)) =
            ParseSymbol(ast::SymbolType::TripleDot).parse(&state)
        {
            params.push(ast::Param::Varargs(varargs));
            state = new_state;
        }

        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::CloseParen), ")");
        let (state, block) = ParseBlock.parse(&state)?;
        let (state, end) = expect!(&state, ParseSymbol(ast::SymbolType::End), "end");
        let end_span = end.span().end();

        Ok((
            state,
            ast::FunctionBody::new(ast::Span::new(start_span, end_span), params, block),
        ))
    }
);

pub struct ParseFunctionExpr;
parser_struct!(
    ParseFunctionExpr,
    ast::FunctionExpr,
    |_, state: &ParseState<'a>| {
        let (state, start) = ParseSymbol(ast::SymbolType::Function).parse(state)?;
        let (state, body) = ParseFunctionBody.parse(&state)?;
        Ok((
            state,
            ast::FunctionExpr::new(
                ast::Span::new(start.span().start(), body.span().end()),
                body,
            ),
        ))
    }
);

pub struct ParseExpr;
parser_struct!(ParseExpr, ast::Expr, |_, state: &ParseState<'a>| {
    ParseExprWithPrecedence(1).parse(state)
});
