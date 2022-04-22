#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use crate::Node;
use lunar_macros::PropertyGetter;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct TypeParameter {
    span: Span,
    name: Token,
    typ: Option<TypeInfo>,
    default: Option<TypeInfo>,
}

impl Node for TypeParameter {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct TypeReference {
    span: Span,
    arguments: Option<Vec<TypeInfo>>,
    name: Token,
}

impl Node for TypeReference {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TypeInfo {
    Reference(TypeReference),
}

impl Node for TypeInfo {
    fn span(&self) -> Span {
        match self {
            TypeInfo::Reference(node) => node.span(),
        }
    }
}
