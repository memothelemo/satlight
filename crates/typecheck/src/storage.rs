use crate::Symbol;
use id_arena::{Arena, Id};

#[derive(Debug, Default)]
pub struct SymbolStorage(Arena<Symbol>);

impl SymbolStorage {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_symbol(&mut self, sym: Symbol) -> Id<Symbol> {
        self.0.alloc(sym)
    }

    pub fn get_symbol(&self, id: Id<Symbol>) -> Option<Symbol> {
        self.0.get(id).cloned()
    }

    pub fn get_symbol_mut(&mut self, id: Id<Symbol>) -> Option<Symbol> {
        self.0.get_mut(id).cloned()
    }

    pub fn get_arena(&self) -> &Arena<Symbol> {
        &self.0
    }

    pub fn get_arena_mut(&mut self) -> &mut Arena<Symbol> {
        &mut self.0
    }
}
