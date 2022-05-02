use super::*;
use crate::ast;

use crate::{expect, no_match_ignore, optional, parse_either, parser_struct};
use lunar_traits::Node;

pub struct ParseBreakStmt;
parser_struct!(ParseBreakStmt, ast::Token, |_, state: &ParseState<'a>| {
    ParseSymbol(ast::SymbolType::Break).parse(state)
});

pub struct ParseCallStmt;
parser_struct!(ParseCallStmt, ast::Expr, |_, state: &ParseState<'a>| {
    let (state, suffix) = ParsePresuffixExpr.parse(state)?;
    if let ast::Expr::Suffixed(ref s) = suffix {
        if matches!(s.suffix(), ast::SuffixKind::Call(_)) {
            return Ok((state, suffix));
        }
    }
    no_match!(state)
});

pub struct ParseDoStmt;
parser_struct!(ParseDoStmt, ast::DoStmt, |_, state: &ParseState<'a>| {
    let (state, r#do) = ParseSymbol(ast::SymbolType::Do).parse(state)?;
    let (state, block) = ParseBlock.parse(&state)?;
    let (state, end) = expect!(&state, ParseSymbol(ast::SymbolType::End), "end");
    Ok((
        state,
        ast::DoStmt::new(ast::Span::merge(r#do.span(), end.span()), block),
    ))
});

pub struct ParseVarAssignName;
parser_struct!(
    ParseVarAssignName,
    ast::VarAssignName,
    |_, state: &ParseState<'a>| {
        let (state, suffix) = ParsePresuffixExpr.parse(state)?;
        match suffix {
            ast::Expr::Suffixed(base) => Ok((
                state.next(0),
                ast::VarAssignName::Suffixed(match &base.suffix() {
                    ast::SuffixKind::Computed(..) => base,
                    ast::SuffixKind::Name(..) => base,

                    #[allow(unreachable_code)]
                    _ => return no_match!(state),
                }),
            )),
            ast::Expr::Literal(ast::Literal::Name(name)) => {
                Ok((state, ast::VarAssignName::Name(name)))
            }
            _ => no_match!(state),
        }
    }
);

pub struct ParseVarAssign;
parser_struct!(
    ParseVarAssign,
    ast::VarAssign,
    |_, state: &ParseState<'a>| {
        let (state, names) = {
            let (new_state, first_name) = ParseVarAssignName.parse(state)?;
            if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Comma).parse(&new_state) {
                let (new_state, mut collections) = OneOrMorePunctuated(
                    ParseVarAssignName,
                    ParseSymbol(ast::SymbolType::Comma),
                    "<id>".into(),
                )
                .parse(&new_state)?;
                collections.insert(0, first_name);
                (new_state, collections)
            } else {
                (new_state, vec![first_name])
            }
        };
        let (state, _) = ParseSymbol(ast::SymbolType::Equal).parse(&state)?;
        let (state, exprlist) = ParseExprListRequired.parse(&state)?;
        Ok((
            state,
            ast::VarAssign::new(
                ast::Span::merge(
                    names.first().unwrap().span(),
                    exprlist.last().unwrap().span(),
                ),
                names,
                exprlist,
            ),
        ))
    }
);

pub struct ParseLocalAssignName;
parser_struct!(
    ParseLocalAssignName,
    ast::LocalAssignName,
    |_, state: &ParseState<'a>| {
        let (state, name) = ParseName.parse(state)?;
        let (state, type_info) = if let Ok((new_state, _)) =
            ParseSymbol(ast::SymbolType::Colon).parse(&state)
        {
            let (new_state, type_info) = expect!(&new_state, ParseTypeInfo, "<type>".to_string());
            (new_state, Some(type_info))
        } else {
            (state, None)
        };
        Ok((
            state,
            ast::LocalAssignName::new(
                ast::Span::merge(
                    name.span(),
                    type_info
                        .as_ref()
                        .map(|v| v.span())
                        .unwrap_or_else(|| name.span()),
                ),
                name,
                type_info,
            ),
        ))
    }
);

pub struct ParseLocalAssign;
#[rustfmt::skip]
parser_struct!(ParseLocalAssign, ast::LocalAssign, |_, state: &ParseState<'a>| {
    let (state, start) = ParseSymbol(ast::SymbolType::Local).parse(state)?;
    let (state, names) = {
        let (new_state, first_name) = ParseLocalAssignName.parse(&state)?;
        if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Comma).parse(&new_state) {
            // weird way to do this
            let (new_state, mut collections) = OneOrMorePunctuated(
                ParseLocalAssignName,
                ParseSymbol(ast::SymbolType::Comma),
                "<id>".into(),
            )
            .parse(&new_state)?;
            collections.insert(0, first_name);
            (new_state, collections)
        } else {
            (new_state, vec![first_name])
        }
    };
    let (state, exprlist) =
        if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Equal).parse(&state) {
            let (new_state, coll) = ParseExprListRequired.parse(&new_state)?;
            (new_state, coll)
        } else {
            (state, Vec::new())
        };
    Ok((
        state,
        ast::LocalAssign::new(
            ast::Span::merge(
                start.span(),
                exprlist
                    .last()
                    .map(|v| v.span())
                    .unwrap_or_else(|| names.last().unwrap().span())
            ),
            names,
            exprlist,
        ),
    ))
});

pub struct ParseLocalFunction;
parser_struct!(
    ParseLocalFunction,
    ast::LocalFunction,
    |_, state: &ParseState<'a>| {
        let (state, start) = ParseSymbol(ast::SymbolType::Local).parse(state)?;
        let (state, _) = ParseSymbol(ast::SymbolType::Function).parse(&state)?;
        let (state, name) = expect!(&state, ParseName, "<id>");
        let (state, body) = ParseFunctionBody.parse(&state)?;
        Ok((
            state,
            ast::LocalFunction::new(ast::Span::merge(start.span(), body.span()), name, body),
        ))
    }
);

pub struct ParseRepeatStmt;
parser_struct!(ParseRepeatStmt, ast::RepeatStmt, |_,
                                                  state: &ParseState<
    'a,
>| {
    let (state, start) = ParseSymbol(ast::SymbolType::Repeat).parse(state)?;
    let (state, block) = ParseBlock.parse(&state)?;
    let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Until), "until");
    let (state, condition) = expect!(&state, ParseExpr, "<exp>");
    Ok((
        state,
        ast::RepeatStmt::new(
            ast::Span::merge(start.span(), condition.span()),
            block,
            condition,
        ),
    ))
});

pub struct ParseReturnStmt;
#[rustfmt::skip]
parser_struct!(ParseReturnStmt, ast::ReturnStmt, |_, state: &ParseState<'a>| {
    let (state, tok) = ParseSymbol(ast::SymbolType::Return).parse(state)?;
    let (state, list) = ParseExprList.parse(&state)?;
    Ok((state, ast::ReturnStmt::new(
        ast::Span::merge(
            tok.span(),
            #[allow(clippy::or_fun_call)]
            list.last().map(|v| v.span()).unwrap_or(tok.span())
        ),
        list
    )))
});

pub struct ParseWhileStmt;
parser_struct!(
    ParseWhileStmt,
    ast::WhileStmt,
    |_, state: &ParseState<'a>| {
        let (state, start) = ParseSymbol(ast::SymbolType::While).parse(state)?;
        let (state, condition) = expect!(&state, ParseExpr, "<exp>");
        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Do), "do");
        let (state, block) = ParseBlock.parse(&state)?;
        let (state, end) = expect!(&state, ParseSymbol(ast::SymbolType::End), "end");
        Ok((
            state,
            ast::WhileStmt::new(ast::Span::merge(start.span(), end.span()), condition, block),
        ))
    }
);

pub struct ParseFunctionAssignName;
parser_struct!(
    ParseFunctionAssignName,
    ast::FunctionAssignName,
    |_, state: &ParseState<'a>| {
        let (mut state, name) = ParseName.parse(state)?;
        let mut name = ast::FunctionAssignName::Name(name);
        loop {
            if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Dot).parse(&state) {
                let (new_state, indexer) = expect!(&new_state, ParseName, "<id>");
                name = ast::FunctionAssignName::Property(Box::new(name), indexer);
                state = new_state;
            } else if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Colon).parse(&state) {
                let (new_state, indexer) = expect!(&new_state, ParseName, "<id>");
                name = ast::FunctionAssignName::Method(Box::new(name), indexer);
                state = new_state;
                break;
            } else {
                break;
            }
        }
        Ok((state, name))
    }
);

pub struct ParseFunctionAssign;
parser_struct!(
    ParseFunctionAssign,
    ast::FunctionAssign,
    |_, state: &ParseState<'a>| {
        let (state, start) = ParseSymbol(ast::SymbolType::Function).parse(state)?;
        let (state, name) = expect!(&state, ParseFunctionAssignName, "<id>");
        let (state, body) = ParseFunctionBody.parse(&state)?;
        Ok((
            state,
            ast::FunctionAssign::new(ast::Span::merge(start.span(), body.span()), name, body),
        ))
    }
);

pub struct ParseGenericFor;
parser_struct!(ParseGenericFor, ast::GenericFor, |_,
                                                  state: &ParseState<
    'a,
>| {
    let (state, start) = ParseSymbol(ast::SymbolType::For).parse(state)?;
    let (state, names) = {
        let (new_state, first_name) = ParseName.parse(&state)?;
        if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Comma).parse(&new_state) {
            // weird way to do this
            let (new_state, mut collections) = OneOrMorePunctuated(
                ParseName,
                ParseSymbol(ast::SymbolType::Comma),
                "<id>".into(),
            )
            .parse(&new_state)?;
            collections.insert(0, first_name);
            (new_state, collections)
        } else {
            (new_state, vec![first_name])
        }
    };
    let (state, _) = ParseSymbol(ast::SymbolType::In).parse(&state)?;
    let (state, exprlist) = ParseExprListRequired.parse(&state)?;
    let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Do), "do");
    let (state, block) = ParseBlock.parse(&state)?;
    let (state, end) = expect!(&state, ParseSymbol(ast::SymbolType::End), "end");
    Ok((
        state,
        ast::GenericFor::new(
            ast::Span::merge(start.span(), end.span()),
            names,
            exprlist,
            block,
        ),
    ))
});

pub struct ParseNumericFor;
parser_struct!(ParseNumericFor, ast::NumericFor, |_,
                                                  state: &ParseState<
    'a,
>| {
    let (state, for_token) = ParseSymbol(ast::SymbolType::For).parse(state)?;
    let (state, name) = ParseName.parse(&state)?;
    let (state, _) = ParseSymbol(ast::SymbolType::Equal).parse(&state)?;
    let (state, start) = expect!(&state, ParseExpr, "<exp>");
    let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Comma), ",");
    let (state, end_exp) = expect!(&state, ParseExpr, "<exp>");
    let (state, step) =
        if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Comma).parse(&state) {
            let (new_state, expr) = expect!(&new_state, ParseExpr, "<exp>");
            (new_state, Some(expr))
        } else {
            (state, None)
        };
    let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Do), "do");
    let (state, block) = ParseBlock.parse(&state)?;
    let (state, end) = expect!(&state, ParseSymbol(ast::SymbolType::End), "end");
    Ok((
        state,
        ast::NumericFor::new(
            ast::Span::merge(for_token.span(), end.span()),
            name,
            Box::new(start),
            Box::new(end_exp),
            step,
            block,
        ),
    ))
});

pub struct ParseElseIfClause;
parser_struct!(
    ParseElseIfClause,
    ast::ElseIfClause,
    |_, state: &ParseState<'a>| {
        let (state, start) = ParseSymbol(ast::SymbolType::ElseIf).parse(state)?;
        let (state, condition) = expect!(&state, ParseExpr, "<exp>");
        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Then), "then");
        let (state, block) = ParseBlock.parse(&state)?;
        Ok((
            state,
            ast::ElseIfClause::new(
                ast::Span::merge(start.span(), block.span()),
                condition,
                block,
            ),
        ))
    }
);

pub struct ParseIfStmt;
parser_struct!(ParseIfStmt, ast::IfStmt, |_, state: &ParseState<'a>| {
    let (state, start) = ParseSymbol(ast::SymbolType::If).parse(state)?;
    let (state, condition) = expect!(&state, ParseExpr, "<exp>");
    let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Then), "then");
    let (state, block) = ParseBlock.parse(&state)?;
    let (state, elseifs) = ZeroOrMore(ParseElseIfClause).parse(&state)?;
    let (state, else_block) = {
        if let Ok((new_state, _)) = ParseSymbol(ast::SymbolType::Else).parse(&state) {
            let (new_state, else_block) = ParseBlock.parse(&new_state)?;
            (new_state, Some(else_block))
        } else {
            (state, None)
        }
    };
    let (state, end) = expect!(&state, ParseSymbol(ast::SymbolType::End), "end");
    Ok((
        state,
        ast::IfStmt::new(
            ast::Span::merge(start.span(), end.span()),
            condition,
            block,
            elseifs,
            else_block,
        ),
    ))
});

pub struct ParseTypeDeclaration;
parser_struct!(
    ParseTypeDeclaration,
    ast::TypeDeclaration,
    |_, state: &ParseState<'a>| {
        let (state, start_tok) = ParseSymbol(ast::SymbolType::Type).parse(state)?;
        let start = start_tok.span().start;
        let (state, name) = expect!(&state, ParseName, "<id>");
        let (state, params) =
            if let Ok((new_state, params)) = no_match_ignore!(&state, ParseTypeParameters) {
                (new_state, Some(params))
            } else {
                (state, None)
            };
        let (state, _) = expect!(&state, ParseSymbol(ast::SymbolType::Equal), "=");
        let (state, typ) = expect!(&state, ParseTypeInfo, "<type>");
        let end = typ.span().end;
        Ok((
            state,
            ast::TypeDeclaration::new(ast::Span::new(start, end), name, params, typ),
        ))
    }
);

pub struct ParseStmt;
parser_struct!(ParseStmt, ast::Stmt, |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseFunctionAssign => ast::Stmt::FunctionAssign,
        ParseCallStmt => ast::Stmt::Call,
        ParseDoStmt => ast::Stmt::Do,
        ParseGenericFor => ast::Stmt::GenericFor,
        ParseIfStmt => ast::Stmt::If,
        ParseLocalAssign => ast::Stmt::LocalAssign,
        ParseLocalFunction => ast::Stmt::LocalFunction,
        ParseNumericFor => ast::Stmt::NumericFor,
        ParseRepeatStmt => ast::Stmt::Repeat,
        ParseWhileStmt => ast::Stmt::While,
        ParseVarAssign => ast::Stmt::VarAssign,
        ParseTypeDeclaration => ast::Stmt::TypeDeclaration,
    })
});

pub struct ParseLastStmt;
parser_struct!(ParseLastStmt, ast::Stmt, |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseBreakStmt => ast::Stmt::Break,
        ParseReturnStmt => ast::Stmt::Return,
    })
});

pub struct ParseEndOfBlock;
parser_struct!(ParseEndOfBlock, (), |_, state: &ParseState<'a>| {
    parse_either!(state, {
        ParseSymbol(ast::SymbolType::End) => |_| (),
        ParseSymbol(ast::SymbolType::ElseIf) => |_| (),
        ParseSymbol(ast::SymbolType::Else) => |_| (),
        ParseSymbol(ast::SymbolType::Until) => |_| (),
        ParseToken(ast::TokenType::Eof) => |_| (),
    })
});

pub struct ParseBlock;
parser_struct!(ParseBlock, ast::Block, |_, state: &ParseState<'a>| {
    let start_position = state.current().unwrap().span();

    let mut state = state.next(0);
    let mut stmts: Vec<ast::Stmt> = Vec::new();

    let mut last_stmt = None;

    while let Ok((ns, stmt)) = no_match_ignore!(&state, ParseStmt) {
        let (ns, _) = optional!(&ns, ParseSymbol(ast::SymbolType::Semicolon));
        state = ns;
        stmts.push(stmt);
    }

    if let Ok((ns, stmt)) = ParseLastStmt.parse(&state) {
        let (ns, _) = optional!(&ns, ParseSymbol(ast::SymbolType::Semicolon));
        last_stmt = Some(Box::new(stmt));
        state = ns;
    }

    // checks if it is in eof, end, elseif or else
    let _ = expect!(&state, ParseEndOfBlock, "end of block");
    let end_position = state.current().unwrap().span();
    Ok((
        state,
        ast::Block::new(
            ast::Span::merge(start_position, end_position),
            stmts,
            last_stmt,
        ),
    ))
});
