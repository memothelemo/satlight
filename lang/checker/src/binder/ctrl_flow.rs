use id_arena::Id;

use super::Symbol;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ConditionFacts {
    pub types: HashMap<String, Id<Symbol>>,
    pub vars: HashMap<String, Id<Symbol>>,
}

impl ConditionFacts {
    pub fn extend(&mut self, other: Self) {
        self.types.extend(other.types);
        self.vars.extend(other.vars);
    }
}
