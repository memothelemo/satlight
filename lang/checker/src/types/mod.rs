use id_arena::Id;
use lunar_ast::Span;

use crate::binder::Symbol;

pub mod makers;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Type {
    Literal(LiteralType),
    Ref(RefType),
    Tuple(TupleType),
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Type::Literal(node) => node.span,
            Type::Ref(node) => node.span,
            Type::Tuple(node) => node.span,
        }
    }

    #[inline(always)]
    fn deref_tuples_inner(&self, vec: &mut Vec<Type>) {
        match &self {
            Type::Tuple(tup) => {
                for typ in tup.members.iter() {
                    typ.deref_tuples_inner(vec);
                }
            }
            _ => vec.push(self.clone()),
        }
    }

    pub fn deref_tuples(self) -> Vec<Type> {
        let mut types = Vec::new();
        self.deref_tuples_inner(&mut types);
        types
    }
}

#[derive(Debug, Clone)]
pub struct RefType {
    pub span: Span,
    pub name: String,
    pub symbol: Id<Symbol>,
    pub arguments: Option<Vec<Type>>,
}

impl PartialEq for RefType {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl std::hash::Hash for RefType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.span.hash(state);
        self.name.hash(state);
        self.symbol.hash(state);
        self.arguments.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct TupleType {
    pub span: Span,
    pub members: Vec<Type>,
}

impl PartialEq for TupleType {
    fn eq(&self, other: &Self) -> bool {
        self.members == other.members
    }
}

impl std::hash::Hash for TupleType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.span.hash(state);
        self.members.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum LiteralKind {
    Any,
    Bool,
    Number,
    Nil,
    String,
    Unknown,
    Void,
}

#[derive(Debug, Clone)]
pub struct LiteralType {
    pub span: Span,
    pub kind: LiteralKind,
}

impl PartialEq for LiteralType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl std::hash::Hash for LiteralType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.span.hash(state);
        self.kind.hash(state);
    }
}
