#![allow(non_upper_case_globals)]

use bitflags::bitflags;
use salite_ast::Span;

use crate::{hir::TypeParameter, types::Type};

bitflags! {
    pub struct SymbolFlags: u32 {
        const BlockVariable = 0b00000001;
        const FunctionParameter = 0b00000010;

        const Function = 0b00000100;

        const TypeAlias = 0b00001000;
        const TypeParameter = 0b00010000;
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub definitions: Vec<Span>,
    pub flags: SymbolFlags,
    pub id: usize,
    pub typ: Option<Type>,
    pub parameters: Option<Vec<TypeParameter>>,
}
