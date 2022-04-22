pub mod bind_ast;
mod types;
mod visitor;

pub use types::*;
pub use visitor::*;

use crate::{Diagnostic, LanguageBuiltin, LunarBuiltin, SymbolStorage};
use id_arena::Id;
use lunar_ast::AstVisitor;
use lunar_macros::PropertyGetter;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SymbolTyp {
    Variable(Typ),
    Type(Typ),
}

#[derive(Debug, PartialEq, Clone, PropertyGetter)]
pub struct Symbol {
    pub(crate) name: String,
    pub(crate) typ: SymbolTyp,
}

impl Symbol {
    pub fn as_variable(&self) -> Option<Typ> {
        match self.typ {
            SymbolTyp::Variable(ty) => Some(ty),
            SymbolTyp::Type(..) => None,
        }
    }

    pub fn as_variable_type(&self) -> Option<Typ> {
        match self.typ {
            SymbolTyp::Variable(..) => None,
            SymbolTyp::Type(ty) => Some(ty),
        }
    }
}

#[derive(Debug, Default)]
pub struct Scope {
    pub(crate) is_looping: bool,
    symbols: Vec<Id<Symbol>>,
}

#[derive(Debug)]
pub enum VarDeclareResult {
    Occupied(Id<Symbol>),
    Created(Id<Symbol>),
}

impl VarDeclareResult {
    pub fn get_symbol_id(self) -> Id<Symbol> {
        match self {
            VarDeclareResult::Created(id) => id,
            VarDeclareResult::Occupied(id) => id,
        }
    }
}

impl Scope {
    pub fn lookup_var(
        &self,
        name: &str,
        storage: &mut SymbolStorage,
    ) -> Option<(Symbol, Id<Symbol>)> {
        for symbol_id in self.symbols.iter() {
            let symbol = storage.get_arena().get(*symbol_id).unwrap();
            if symbol.name == name && matches!(symbol.typ, SymbolTyp::Variable(..)) {
                return Some((symbol.clone(), *symbol_id));
            }
        }
        None
    }

    pub fn lookup_typ(
        &self,
        name: &str,
        storage: &mut SymbolStorage,
    ) -> Option<(Symbol, Id<Symbol>)> {
        for symbol_id in self.symbols.iter() {
            let symbol = storage.get_arena().get(*symbol_id).unwrap();
            if symbol.name == name && matches!(symbol.typ, SymbolTyp::Type(..)) {
                return Some((symbol.clone(), *symbol_id));
            }
        }
        None
    }

    pub fn try_declare(
        &mut self,
        name: &str,
        typ: SymbolTyp,
        storage: &mut SymbolStorage,
    ) -> VarDeclareResult {
        if let Some((_, id)) = match typ {
            SymbolTyp::Type(..) => self.lookup_typ(name, storage),
            SymbolTyp::Variable(..) => self.lookup_var(name, storage),
        } {
            VarDeclareResult::Occupied(id)
        } else {
            let id = storage.create_symbol(Symbol {
                name: name.to_string(),
                typ,
            });
            self.symbols.push(id);
            VarDeclareResult::Created(id)
        }
    }
}

pub struct ScopeStack(Vec<Scope>);

impl std::fmt::Debug for ScopeStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ScopeStack {
    pub fn empty() -> Self {
        ScopeStack(Vec::new())
    }

    pub fn new(base: Scope) -> Self {
        ScopeStack(vec![base])
    }

    pub fn push(&mut self, scope: Scope) {
        self.0.push(scope);
    }

    pub fn pop(&mut self) -> Option<Scope> {
        self.0.pop()
    }

    pub fn current(&self) -> &Scope {
        self.0.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut Scope {
        self.0.last_mut().unwrap()
    }

    pub fn lookup_var(
        &self,
        name: &str,
        storage: &mut SymbolStorage,
    ) -> Option<(Symbol, Id<Symbol>)> {
        for scope in self.0.iter().rev() {
            if let Some(result) = scope.lookup_var(name, storage) {
                return Some(result);
            }
        }
        None
    }

    pub fn lookup_typ(
        &self,
        name: &str,
        storage: &mut SymbolStorage,
    ) -> Option<(Symbol, Id<Symbol>)> {
        for scope in self.0.iter().rev() {
            if let Some(result) = scope.lookup_typ(name, storage) {
                return Some(result);
            }
        }
        None
    }

    pub fn try_declare(
        &mut self,
        name: &str,
        typ: SymbolTyp,
        storage: &mut SymbolStorage,
    ) -> VarDeclareResult {
        self.current_mut().try_declare(name, typ, storage)
    }
}

#[derive(Debug)]
pub struct Binder {
    pub(crate) block_result: Option<bind_ast::Block>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) stack: ScopeStack,
    pub(crate) storage: SymbolStorage,
}

impl Binder {
    pub fn empty(builtin: Option<&mut dyn LanguageBuiltin>) -> Self {
        let mut binder = Binder {
            block_result: None,
            diagnostics: Vec::new(),
            stack: ScopeStack::new(Scope::default()),
            storage: SymbolStorage::new(),
        };
        binder.init_builtin(builtin.unwrap_or(&mut LunarBuiltin));
        binder
    }

    pub fn new(scope: Scope) -> Self {
        Binder {
            block_result: None,
            diagnostics: Vec::new(),
            stack: ScopeStack::new(scope),
            storage: SymbolStorage::new(),
        }
    }

    pub fn from_block(builtin: Option<&mut dyn LanguageBuiltin>, block: &lunar_ast::Block) -> Self {
        let mut binder = Self::empty(builtin);
        binder.block_result = Some(binder.visit_block(block));
        binder
    }

    pub fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }

    pub fn block_result(&self) -> &Option<bind_ast::Block> {
        &self.block_result
    }

    pub fn init_builtin(&mut self, builtin: &mut dyn LanguageBuiltin) {
        builtin.add_builtin_types(self.stack.current_mut(), &mut self.storage);
    }

    pub fn bind_block(&mut self, block: &lunar_ast::Block) -> bind_ast::Block {
        self.visit_block(block)
    }
}
