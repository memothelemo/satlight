use super::{Binder, ConditionFacts, Symbol};
use id_arena::Id;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ScopeKind {
    Block,
    Function,
    TypeAliasValue,
}

#[derive(Debug)]
pub struct Scope {
    pub(crate) facts: ConditionFacts,

    #[allow(unused)]
    pub(crate) kind: ScopeKind,
    pub(crate) parent: Option<Id<Scope>>,

    pub(crate) types: HashMap<String, Id<Symbol>>,
    pub(crate) vars: HashMap<String, Id<Symbol>>,
}

impl Scope {
    pub fn new(kind: ScopeKind, parent: Option<Id<Scope>>) -> Self {
        Self {
            facts: ConditionFacts::default(),
            kind,
            parent,
            types: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
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

    pub fn find_symbol_type(&self, analyzer: &Binder, name: &String) -> Option<Id<Symbol>> {
        if let Some(fact) = self.facts.types.get(name) {
            return Some(*fact);
        }

        if let Some(symbol) = self.types.get(name) {
            return Some(*symbol);
        }

        self.parent
            .as_ref()
            .and_then(|scope_id| analyzer.scopes.get(*scope_id))
            .and_then(|scope| scope.find_symbol_type(analyzer, name))
    }

    pub fn find_symbol_var(&self, analyzer: &Binder, name: &String) -> Option<Id<Symbol>> {
        if let Some(fact) = self.facts.vars.get(name) {
            return Some(*fact);
        }

        if let Some(symbol) = self.vars.get(name) {
            return Some(*symbol);
        }

        self.parent
            .as_ref()
            .and_then(|scope_id| analyzer.scopes.get(*scope_id))
            .and_then(|scope| scope.find_symbol_var(analyzer, name))
    }
}
