use super::*;
use id_arena::Id;
use salite_common::dictionary::Dictionary;

#[derive(Debug, PartialEq)]
pub enum ScopeKind {
    Module,
    Block,
    Function,
}

#[derive(Debug, Default)]
pub struct ConditionFacts {
    pub types: Dictionary<Id<Symbol>, Id<Symbol>>,
    pub vars: Dictionary<Id<Symbol>, Id<Symbol>>,
}

impl ConditionFacts {
    pub fn extend(&mut self, other: Self) {
        self.types.extend(other.types);
        self.vars.extend(other.vars);
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Scope {
    pub(crate) facts: ConditionFacts,

    pub(crate) kind: ScopeKind,
    pub(crate) parent: Option<Id<Scope>>,

    pub(crate) types: Dictionary<String, Id<Symbol>>,
    pub(crate) vars: Dictionary<String, Id<Symbol>>,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            facts: Default::default(),
            kind: ScopeKind::Module,
            parent: None,
            types: Default::default(),
            vars: Default::default(),
        }
    }
}

impl Scope {
    pub fn new(kind: ScopeKind, parent: Option<Id<Scope>>) -> Self {
        Self {
            kind,
            parent,
            ..Default::default()
        }
    }

    #[inline]
    pub fn root() -> Self {
        Self::default()
    }
}

impl Scope {
    pub fn is_returnable(&self) -> bool {
        matches!(self.kind, ScopeKind::Function | ScopeKind::Module)
    }
}

impl Scope {
    pub fn depth(&self, ctx: &ModuleContext) -> usize {
        let mut depth = 0;
        let mut parento = self.parent;
        while let Some(parent) = parento {
            depth += 1;
            parento = ctx.scopes.get(parent).unwrap().parent;
        }
        depth
    }

    pub fn search_variable(&self, ctx: &ModuleContext, name: &String) -> Option<Id<Symbol>> {
        if let Some(symbol_id) = self.vars.get(name) {
            // fact checking
            if let Some(fact) = self.facts.vars.get(symbol_id) {
                return Some(*fact);
            } else {
                return Some(*symbol_id);
            }
        }

        if let Some(parent) = self.parent {
            let scope = ctx.scopes.get(parent).unwrap();
            scope.search_variable(ctx, name)
        } else {
            None
        }
    }

    pub fn search_type_alias(&self, ctx: &ModuleContext, name: &String) -> Option<Id<Symbol>> {
        if let Some(symbol_id) = self.types.get(name) {
            // fact checking
            if let Some(fact) = self.facts.types.get(symbol_id) {
                return Some(*fact);
            } else {
                return Some(*symbol_id);
            }
        }

        if let Some(parent) = self.parent {
            let scope = ctx.scopes.get(parent).unwrap();
            scope.search_type_alias(ctx, name)
        } else {
            None
        }
    }
}
