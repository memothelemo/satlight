mod atom;

use atom::*;
use logos::Logos;

use salite_ast::*;
use salite_common::errors::tokenizer::*;

type UnknownToken = (Result<TokenType, TokenizeErrorType>, Span);

fn tokenize_atom(atom: Atom, slice: &str) -> Result<TokenType, TokenizeErrorType> {
    match atom {
        Atom::Comment => {
            // check for any multi-comment patterns there
            let equals = atom::get_equal_brackets(&slice[2..]);
            let len = slice.len();
            let multiline_len = equals.map(|v| v + 2).unwrap_or(0);
            let last_deduct = if slice.ends_with('\n') { 1 } else { 0 };
            Ok(TokenType::Comment(
                slice[2 + multiline_len..(len - multiline_len - last_deduct)].into(),
            ))
        }
        Atom::Number => Ok(TokenType::Number(slice.into())),
        Atom::QuoteString | Atom::ApostropheString => {
            let len = slice.len();
            Ok(TokenType::Str(slice[1..len - 1].into()))
        }
        Atom::BracketString => {
            let equals = atom::get_equal_brackets(slice).unwrap();
            let len = slice.len();
            Ok(TokenType::Str(slice[2 + equals..len - (2 + equals)].into()))
        }
        Atom::Whitespace => Ok(TokenType::Whitespace(slice.into())),
        Atom::Unknown => Err(match slice.chars().next().unwrap_or('\0') {
            '\'' | '"' | '[' => TokenizeErrorType::IncompleteString,
            '-' => TokenizeErrorType::IncompleteComment,
            '#' => TokenizeErrorType::IncorrectShebang,
            c => TokenizeErrorType::UnexpectedCharacter(c),
        }),
        Atom::Shebang => Err(TokenizeErrorType::IncorrectShebang),
        Atom::Identifier => Ok(TokenType::Identifier(slice.into())),
        _ => Ok(TokenType::Symbol(SymbolType::parse(slice).unwrap())),
    }
}

macro_rules! next_if {
    ($lexer:expr, $cmp:expr) => {
        if $lexer.clone().next() == Some($cmp) {
            $lexer.next();
            Some($lexer.slice().into())
        } else {
            None
        }
    };
}

fn tokenize_code(input: &'_ str) -> Vec<UnknownToken> {
    let mut lexer = Atom::lexer(input);
    let mut tokens: Vec<UnknownToken> = Vec::new();

    if let Some(text) = next_if!(lexer, Atom::Shebang) {
        tokens.push((
            Ok(TokenType::Shebang(text)),
            Span::new(lexer.span().start, lexer.span().end),
        ))
    }

    while let Some(atom) = lexer.next() {
        tokens.push((
            tokenize_atom(atom, lexer.slice()),
            Span::new(lexer.span().start, lexer.span().end),
        ));
    }

    tokens
}

pub fn tokenize(input: &'_ str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut current_span = Span::new(0, 0);

    for (raw_token, span) in tokenize_code(input) {
        let ty = raw_token.map_err(|ty| TokenizeError { ty, span })?;
        current_span = span;
        tokens.push(Token::new(ty, span));
    }

    tokens.push(Token::new(
        TokenType::Eof,
        Span::new(current_span.end, current_span.end),
    ));
    Ok(tokens)
}
