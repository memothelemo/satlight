use crate::binder::Symbol;
use id_arena::Id;
use lunar_ast::Span;
use lunar_common::dictionary::Dictionary;

pub mod makers;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Type {
    Literal(LiteralType),
    Ref(RefType),
    Tuple(TupleType),
    Table(Table),
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Type::Literal(node) => node.span,
            Type::Ref(node) => node.span,
            Type::Tuple(node) => node.span,
            Type::Table(node) => node.span,
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
pub enum TableFieldKey {
    ///   |----|
    /// { Hello = "World" }
    Name(String, Span),

    ///   |------|
    /// { [string] = 10 }
    Computed(Type),

    /// Array like member
    None(usize),
}

impl PartialEq for TableFieldKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Name(l0, _), Self::Name(r0, _)) => l0 == r0,
            (Self::Computed(l0), Self::Computed(r0)) => l0 == r0,
            (Self::None(l0), Self::None(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl std::hash::Hash for TableFieldKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub span: Span,
    pub entries: Dictionary<TableFieldKey, Type>,
    pub metatable: Option<Box<Table>>,
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        // kind of dangerous way to do?
        self.span == other.span
    }
}

impl std::hash::Hash for Table {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entries.hash(state);
        self.metatable.hash(state);
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
