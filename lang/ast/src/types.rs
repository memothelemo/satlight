#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use lunar_macros::{CtorCall, FieldCall};
use lunar_traits::SpannedNode;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct TypeParameter {
    #[exclude]
    span: Span,
    name: Token,
    typ: Option<TypeInfo>,
    default: Option<TypeInfo>,
}

impl SpannedNode for TypeParameter {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct TypeCallbackParameter {
    #[exclude]
    span: Span,
    name: Option<Token>,
    type_info: TypeInfo,
}

impl SpannedNode for TypeCallbackParameter {
    fn span(&self) -> Span {
        self.span
    }
}

// ( [ [<Name> `:`] <typeinfo> ( [<Name> `:`] <typeinfo> )* ] ) -> <typeinfo>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct TypeCallback {
    #[exclude]
    span: Span,
    parameters: Vec<TypeCallbackParameter>,
    return_type: Box<TypeInfo>,
}

impl SpannedNode for TypeCallback {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct TypeReference {
    #[exclude]
    span: Span,
    arguments: Option<Vec<TypeInfo>>,
    name: Token,
}

impl SpannedNode for TypeReference {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TypeInfo {
    Callback(TypeCallback),
    Reference(TypeReference),
}

impl SpannedNode for TypeInfo {
    fn span(&self) -> Span {
        match self {
            TypeInfo::Callback(node) => node.span(),
            TypeInfo::Reference(node) => node.span(),
        }
    }
}
