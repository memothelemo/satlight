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
        const UnknownVariable = 0b00100000;

        const Intrinsic = 0b01000000;
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub definitions: Vec<Span>,
    pub flags: SymbolFlags,
    pub typ: Option<Type>,
    pub type_parameters: Option<Vec<TypeParameter>>,
}
