use crate::binder::Symbol;
use id_arena::Id;
use salite_ast::Span;
use salite_common::dictionary::Dictionary;

pub mod makers;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Type {
    Function(FunctionType),
    Literal(LiteralType),
    Ref(RefType),
    Tuple(TupleType),
    Table(Table),
}

impl Type {
    pub fn get_lit_type(&self) -> Option<&LiteralKind> {
        if let Type::Literal(lit) = self {
            Some(&lit.kind)
        } else {
            None
        }
    }

    pub fn span_mut(&mut self) -> &mut Span {
        match self {
            Type::Literal(node) => &mut node.span,
            Type::Ref(node) => &mut node.span,
            Type::Tuple(node) => &mut node.span,
            Type::Table(node) => &mut node.span,
            Type::Function(node) => &mut node.span,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Type::Literal(node) => node.span,
            Type::Ref(node) => node.span,
            Type::Tuple(node) => node.span,
            Type::Table(node) => node.span,
            Type::Function(node) => node.span,
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
pub struct FunctionParameter {
    pub span: Span,
    pub name: Option<String>,
    pub typ: Type,
}

impl PartialEq for FunctionParameter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.typ == other.typ
    }
}

impl std::hash::Hash for FunctionParameter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.typ.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct VaridiacParameter {
    pub span: Span,
    pub typ: Box<Type>,
}

impl PartialEq for VaridiacParameter {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ
    }
}

impl std::hash::Hash for VaridiacParameter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typ.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub span: Span,
    pub parameters: Vec<FunctionParameter>,
    pub varidiac_param: Option<VaridiacParameter>,
    pub return_type: Box<Type>,
}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        self.parameters == other.parameters && self.return_type == other.return_type
    }
}

impl std::hash::Hash for FunctionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parameters.hash(state);
        self.return_type.hash(state);
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
    pub is_metatable: bool,
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
