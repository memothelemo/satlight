use crate::types::Type;

use super::{Binder, ConditionFacts, Symbol};
use id_arena::Id;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ScopeKind {
    Module,
    Block,
    Function,
    TypeAliasValue,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Scope {
    pub(crate) facts: ConditionFacts,

    pub(crate) actual_type: Option<Type>,
    pub(crate) expected_type: Option<Type>,

    #[allow(unused)]
    pub(crate) kind: ScopeKind,
    pub(crate) parent: Option<Id<Scope>>,

    pub(crate) types: HashMap<String, Id<Symbol>>,
    pub(crate) vars: HashMap<String, Id<Symbol>>,
}

impl Scope {
    pub fn is_returnable(&self) -> bool {
        matches!(self.kind, ScopeKind::Function | ScopeKind::Module)
    }
}

impl Scope {
    pub fn new(kind: ScopeKind, parent: Option<Id<Scope>>) -> Self {
        Self {
            actual_type: None,
            expected_type: None,
            facts: ConditionFacts::default(),
            kind,
            parent,
            types: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            actual_type: None,
            expected_type: None,
            facts: ConditionFacts::default(),
            kind: ScopeKind::Block,
            parent: None,
            types: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn depth(&self, analyzer: &Binder) -> usize {
        let mut depth = 0;
        let mut opt_parent = self.parent;
        while let Some(parent) = opt_parent {
            depth += 1;
            opt_parent = analyzer.scopes.get(parent).unwrap().parent;
        }
        depth
    }

    pub fn symbol_from_type_alias(&self, analyzer: &Binder, name: &String) -> Option<Id<Symbol>> {
        if let Some(symbol_id) = self.types.get(name) {
            // fact checking
            if let Some(fact) = self.facts.types.get(symbol_id) {
                return Some(*fact);
            }
            return Some(*symbol_id);
        }

        self.parent
            .as_ref()
            .and_then(|scope_id| analyzer.scopes.get(*scope_id))
            .and_then(|scope| scope.symbol_from_type_alias(analyzer, name))
    }

    pub fn symbol_from_variable(&self, analyzer: &Binder, name: &String) -> Option<Id<Symbol>> {
        if let Some(symbol_id) = self.vars.get(name) {
            // fact checking
            if let Some(fact) = self.facts.vars.get(symbol_id) {
                return Some(*fact);
            }
            return Some(*symbol_id);
        }

        self.parent
            .as_ref()
            .and_then(|scope_id| analyzer.scopes.get(*scope_id))
            .and_then(|scope| scope.symbol_from_variable(analyzer, name))
    }
}
