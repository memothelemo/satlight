#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use crate::operator;

operator! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq)]
    pub enum UnopKind {
        fn is_right_associate(&self) = |_| false,

        Length => 7,
        Not => 7,
        Negate => 7,
    }

    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq)]
    pub enum BinopKind {
        fn is_right_associate(&self) = | k: &BinopKind | {
            matches!(k, Self::Concat | Self::Exponent)
        },
        NilshCoalescing => 11,
        Exponent => 10,
        Multiply => 7,
        FloorDivision => 7,
        Divide => 7,
        Modulo => 7,
        Add => 6,
        Subtract => 6,
        Concat => 5,
        Equality => 3,
        Inequality => 3,
        GreaterThan => 3,
        GreaterEqual => 3,
        LessThan => 3,
        LessEqual => 3,
        And => 2,
        Or => 1,
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Unop {
    pub kind: UnopKind,
    pub token: Token,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Binop {
    pub kind: BinopKind,
    pub token: Token,
}
