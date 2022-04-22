#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde[rename_all = "camelCase"])]
#[derive(Debug, Clone, PartialEq)]
pub struct CompilerOptions {
    pub multicore_typechecking: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            multicore_typechecking: false,
        }
    }
}
