use crate::*;
use salite_ast::Span;
use std::hash::Hash;

pub mod makers;
pub mod utils;

pub trait TypeTrait: std::fmt::Debug {
    fn span(&self) -> Span;
    fn span_mut(&mut self) -> &mut Span;
}

pub mod variants {
    use id_arena::Id;
    use salite_common::dictionary::Dictionary;

    use super::*;

    macro_rules! member_typed {
        {
			$( $name:ident, )*
		} => {
            $( #[derive(Debug, Clone)]
            pub struct $name {
                pub span: Span,
                pub members: Vec<Type>,
            }

            impl PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    self.members == other.members
                }
            }

            impl Hash for $name {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.members.hash(state);
                    self.span.hash(state);
                }
            }

			impl TypeTrait for $name {
				fn span(&self) -> Span {
					self.span
				}

    			fn span_mut(&mut self) -> &mut Span {
					&mut self.span
				}
			} )*
        };
    }

    member_typed! {
        Union,
        Intersection,
        Tuple,
    }

    #[derive(Debug, Clone, PartialEq, Hash)]
    pub enum LiteralType {
        Bool,
        Number,
        Nil,
        String,
        Void,
    }

    #[derive(Debug, Clone)]
    pub struct Literal {
        pub span: Span,
        pub typ: LiteralType,
    }

    impl PartialEq for Literal {
        fn eq(&self, other: &Self) -> bool {
            self.typ == other.typ
        }
    }

    impl std::hash::Hash for Literal {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.span.hash(state);
            self.typ.hash(state);
        }
    }

    impl TypeTrait for Literal {
        fn span(&self) -> Span {
            self.span
        }

        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }

    #[derive(Debug, Clone)]
    pub struct Reference {
        pub span: Span,
        pub name: String,
        pub symbol: Id<Symbol>,
        pub arguments: Option<Vec<Type>>,
    }

    impl PartialEq for Reference {
        fn eq(&self, other: &Self) -> bool {
            self.symbol == other.symbol
        }
    }

    impl std::hash::Hash for Reference {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.span.hash(state);
            self.name.hash(state);
            self.symbol.hash(state);
            self.arguments.hash(state);
        }
    }

    impl TypeTrait for Reference {
        fn span(&self) -> Span {
            self.span
        }

        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }

    #[derive(Debug, Clone)]
    pub struct Unresolved {
        pub span: Span,
        pub symbol: Id<Symbol>,
    }

    impl PartialEq for Unresolved {
        fn eq(&self, other: &Self) -> bool {
            self.symbol == other.symbol
        }
    }

    impl std::hash::Hash for Unresolved {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.span.hash(state);
            self.symbol.hash(state);
        }
    }

    impl TypeTrait for Unresolved {
        fn span(&self) -> Span {
            self.span
        }

        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }

    #[derive(Debug, Clone)]
    pub struct FunctionParameter {
        pub span: Span,
        pub name: Option<String>,
        pub optional: bool,
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
    pub struct Function {
        pub span: Span,
        pub parameters: Vec<FunctionParameter>,
        pub varidiac_param: Option<VaridiacParameter>,
        pub return_type: Box<Type>,
    }

    impl PartialEq for Function {
        fn eq(&self, other: &Self) -> bool {
            self.parameters == other.parameters && self.return_type == other.return_type
        }
    }

    impl std::hash::Hash for Function {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.parameters.hash(state);
            self.return_type.hash(state);
        }
    }

    impl TypeTrait for Function {
        fn span(&self) -> Span {
            self.span
        }

        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }

    #[derive(Debug, Clone)]
    pub enum TableFieldKey {
        ///   |----|
        /// { Hello = "World" }
        Name(String, Span),

        ///   |------|
        /// { [string] = 10 }
        Computed(Type, Span),

        /// Array like member
        None(usize, Span),
    }

    impl PartialEq for TableFieldKey {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Name(l0, ..), Self::Name(r0, ..)) => l0 == r0,
                (Self::Computed(l0, ..), Self::Computed(r0, ..)) => l0 == r0,
                (Self::None(l0, ..), Self::None(r0, ..)) => l0 == r0,
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

    impl Table {
        pub fn combine(&mut self, tbl: &Table, span: Span) {
            for (key, right) in tbl.entries.iter() {
                // TODO(memothelemo): Make an utility thing where it tries
                // to combine left and right types instead of this approach.
                let current_value = self.entries.get_mut(key);
                if let Some(current_value) = current_value {
                    *current_value = Type::Intersection(Intersection {
                        span,
                        members: vec![current_value.clone(), right.clone()],
                    });
                } else {
                    self.entries.insert(key.clone(), right.clone());
                }
            }
        }
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

    impl TypeTrait for Table {
        fn span(&self) -> Span {
            self.span
        }

        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }

    #[derive(Debug, Clone)]
    pub struct Recursive {
        pub span: Span,
        pub symbol: Id<Symbol>,
    }

    impl PartialEq for Recursive {
        fn eq(&self, other: &Self) -> bool {
            self.symbol == other.symbol
        }
    }

    impl std::hash::Hash for Recursive {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.span.hash(state);
            self.symbol.hash(state);
        }
    }

    impl TypeTrait for Recursive {
        fn span(&self) -> Span {
            self.span
        }

        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Type {
    Any(Span),
    Function(variants::Function),
    Literal(variants::Literal),
    Intersection(variants::Intersection),
    Reference(variants::Reference),
    Recursive(variants::Recursive),
    Unknown(Span),
    Union(variants::Union),
    Unresolved(variants::Unresolved),
    Table(variants::Table),
    Tuple(variants::Tuple),
}

impl Type {
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

impl TypeTrait for Type {
    fn span(&self) -> Span {
        match self {
            Type::Any(span) => *span,
            Type::Function(node) => node.span(),
            Type::Literal(node) => node.span(),
            Type::Intersection(node) => node.span(),
            Type::Reference(node) => node.span(),
            Type::Unknown(span) => *span,
            Type::Union(node) => node.span(),
            Type::Unresolved(node) => node.span(),
            Type::Table(node) => node.span(),
            Type::Tuple(node) => node.span(),
            Type::Recursive(node) => node.span(),
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        match self {
            Type::Any(span) => span,
            Type::Function(node) => node.span_mut(),
            Type::Literal(node) => node.span_mut(),
            Type::Intersection(node) => node.span_mut(),
            Type::Reference(node) => node.span_mut(),
            Type::Unknown(span) => span,
            Type::Union(node) => node.span_mut(),
            Type::Unresolved(node) => node.span_mut(),
            Type::Table(node) => node.span_mut(),
            Type::Tuple(node) => node.span_mut(),
            Type::Recursive(node) => node.span_mut(),
        }
    }
}
