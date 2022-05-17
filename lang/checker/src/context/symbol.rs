use crate::{hir::TypeParameter, types::Type};
use salite_ast::Span;

#[derive(Debug, PartialEq)]
pub struct TypeAliasSymbol {
    pub name: String,
    pub typ: Type,
    pub parameters: Option<Vec<TypeParameter>>,
}

#[derive(Debug, PartialEq)]
pub struct BlockVariableSymbol {
    pub name: String,
    pub typ: Type,
    pub explicit: bool,
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    BlockVariable(BlockVariableSymbol),
    FunctionParameter(String, Type, bool),
    TypeParameter(String, Type),
    TypeAlias(TypeAliasSymbol),
    UnknownVariable,
    Value(Type),
}

#[derive(Debug)]
pub struct Symbol {
    pub definitions: Vec<Span>,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn get_type(&self) -> Option<&Type> {
        match &self.kind {
            SymbolKind::BlockVariable(ty) => Some(&ty.typ),
            SymbolKind::TypeParameter(.., ty) => Some(ty),
            SymbolKind::TypeAlias(ty) => Some(&ty.typ),
            SymbolKind::UnknownVariable => None,
            SymbolKind::Value(ty) => Some(ty),
            SymbolKind::FunctionParameter(_, ty, ..) => Some(ty),
        }
    }
}
