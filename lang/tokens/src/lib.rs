#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[macro_use]
mod macros;

use salite_location::Span;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eof,
    Comment,
    Identifier,
    Number,
    Shebang,
    Str,
    Symbol,
    Whitespace,
}

impl Eq for TokenKind {}

symbols! {
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[derive(Clone, PartialEq, Eq)]
    pub enum SymbolType {
        And => "and",
        Break => "break",
        Do => "do",
        Else => "else",
        ElseIf => "elseif",
        End => "end",
        False => "false",
        For => "for",
        Function => "function",
        If => "if",
        In => "in",
        Local => "local",
        Nil => "nil",
        Not => "not",
        Or => "or",
        Repeat => "repeat",
        Return => "return",
        Then => "then",
        True => "true",
        Type => "type",
        Until => "until",
        While => "while",

        TripleDot => "...",
        DoubleDot => "..",
        Dot => ".",

        SkinnyArrow => "->",

        GreaterEqual => ">=",
        LessEqual => "<=",
        DoubleEqual => "==",
        TildeEqual => "~=",

        GreaterThan => ">",
        LessThan => "<",
        Equal => "=",

        OpenParen => "(",
        CloseParen => ")",

        OpenBracket => "[",
        CloseBracket => "]",

        OpenCurly => "{",
        CloseCurly => "}",

        Semicolon  => ";",
        DoubleColon => "::",
        Colon => ":",
        Comma => ",",

        Cross => "+",
        Dash => "-",
        Asterisk => "*",
        Slash => "/",
        Percent => "%",
        Caret => "^",
        Hash => "#",

        MetatableTag => "@metatable",
        Question => "?",

        VerticalBar => "|",
        Ampersand => "&",
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Eof,
    Comment(SmolStr),
    Identifier(SmolStr),
    Number(SmolStr),
    Shebang(SmolStr),
    Str(SmolStr),
    Symbol(SymbolType),
    Whitespace(SmolStr),
}

impl TokenType {
    pub fn is_trivia(&self) -> bool {
        matches!(
            self,
            TokenType::Comment(..) | TokenType::Shebang(..) | TokenType::Whitespace(..)
        )
    }

    pub fn kind(&self) -> TokenKind {
        match self {
            TokenType::Eof => TokenKind::Eof,
            TokenType::Comment(_) => TokenKind::Comment,
            TokenType::Identifier(_) => TokenKind::Identifier,
            TokenType::Number(_) => TokenKind::Number,
            TokenType::Shebang(_) => TokenKind::Shebang,
            TokenType::Str(_) => TokenKind::Str,
            TokenType::Symbol(_) => TokenKind::Symbol,
            TokenType::Whitespace(_) => TokenKind::Whitespace,
        }
    }

    pub fn as_name(&self) -> String {
        if let TokenType::Identifier(name) = self {
            name.to_string()
        } else if let TokenType::Symbol(s) = self {
            s.to_str()
        } else {
            String::new()
        }
    }
}

impl Eq for TokenType {}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    ty: TokenType,
    span: Span,
}

impl salite_traits::SpannedNode for Token {
    fn span(&self) -> Span {
        self.span
    }
}

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Token {
        Token { ty, span }
    }

    pub fn ty(&self) -> &TokenType {
        &self.ty
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> TokenKind {
        self.ty.kind()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_symbols() {
        macro_rules! lazy_test {
			{
				$( $text:expr => $expected:expr, )*
			} => {
				$( assert_eq!(SymbolType::parse($text), $expected); )*
			};
		}
        lazy_test! {
            "." => Some(SymbolType::Dot),
            ".." => Some(SymbolType::DoubleDot),
            "..." => Some(SymbolType::TripleDot),
            "=" => Some(SymbolType::Equal),
            "==" => Some(SymbolType::DoubleEqual),
            "" => None,
        };
    }
}
