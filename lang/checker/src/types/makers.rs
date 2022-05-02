use super::*;

pub fn any(span: Span) -> Type {
    Type::Literal(LiteralType {
        kind: LiteralKind::Any,
        span,
    })
}

pub fn bool(span: Span) -> Type {
    Type::Literal(LiteralType {
        kind: LiteralKind::Bool,
        span,
    })
}

pub fn number(span: Span) -> Type {
    Type::Literal(LiteralType {
        kind: LiteralKind::Number,
        span,
    })
}

pub fn nil(span: Span) -> Type {
    Type::Literal(LiteralType {
        kind: LiteralKind::Nil,
        span,
    })
}

pub fn reference(span: Span, symbol: Id<Symbol>, name: String, args: Option<Vec<Type>>) -> Type {
    Type::Ref(RefType {
        name,
        symbol,
        arguments: args,
        span,
    })
}

pub fn string(span: Span) -> Type {
    Type::Literal(LiteralType {
        kind: LiteralKind::String,
        span,
    })
}

pub fn tuple(span: Span, members: Vec<Type>) -> Type {
    Type::Tuple(TupleType { members, span })
}

pub fn unknown(span: Span) -> Type {
    Type::Literal(LiteralType {
        kind: LiteralKind::Unknown,
        span,
    })
}

pub fn void(span: Span) -> Type {
    Type::Literal(LiteralType {
        kind: LiteralKind::Void,
        span,
    })
}
