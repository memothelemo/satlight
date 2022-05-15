use super::*;

use id_arena::Id;
use variants::*;

pub fn any(span: Span) -> Type {
    Type::Any(span)
}

pub fn bool(span: Span) -> Type {
    Type::Literal(Literal {
        typ: LiteralType::Bool,
        span,
    })
}

pub fn number(span: Span) -> Type {
    Type::Literal(Literal {
        typ: LiteralType::Number,
        span,
    })
}

pub fn nil(span: Span) -> Type {
    Type::Literal(Literal {
        typ: LiteralType::Nil,
        span,
    })
}

pub fn recursive(symbol: Id<Symbol>, span: Span) -> Type {
    Type::Recursive(Recursive { span, symbol })
}

pub fn unresolved(symbol: Id<Symbol>, span: Span) -> Type {
    Type::Unresolved(Unresolved { span, symbol })
}

pub fn reference(span: Span, symbol: Id<Symbol>, name: String, args: Option<Vec<Type>>) -> Type {
    Type::Reference(Reference {
        name,
        symbol,
        arguments: args,
        span,
    })
}

pub fn string(span: Span) -> Type {
    Type::Literal(Literal {
        typ: LiteralType::String,
        span,
    })
}

pub fn tuple(span: Span, members: Vec<Type>) -> Type {
    Type::Tuple(Tuple { members, span })
}

pub fn unknown(span: Span) -> Type {
    Type::Unknown(span)
}

pub fn void(span: Span) -> Type {
    Type::Literal(Literal {
        typ: LiteralType::Void,
        span,
    })
}
