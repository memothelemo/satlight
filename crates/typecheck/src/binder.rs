pub mod bind_ast;
mod types;
mod visitor;

pub use types::*;
pub use visitor::*;

use crate::{Diagnostic, LanguageBuiltin, LunarBuiltin, SymbolStorage};
use id_arena::{Arena, Id};
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
        match &self.typ {
            SymbolTyp::Variable(ty) => Some(ty.clone()),
            SymbolTyp::Type(..) => None,
        }
    }

    pub fn as_variable_type(&self) -> Option<Typ> {
        match &self.typ {
            SymbolTyp::Variable(..) => None,
            SymbolTyp::Type(ty) => Some(ty.clone()),
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    pub(crate) is_looping: bool,
    pub(crate) expected_type: Option<Typ>,
    pub(crate) real_type: Option<Typ>,
    pub(crate) parent: Option<Id<Scope>>,
    symbols: Vec<Id<Symbol>>,
}

#[derive(Debug)]
pub enum VarDeclareResult {
    Occupied(Id<Symbol>),
    Created(Id<Symbol>),
}

impl VarDeclareResult {
    pub fn occupied(&self) -> bool {
        matches!(self, VarDeclareResult::Occupied(..))
    }

    pub fn get_symbol_id(self) -> Id<Symbol> {
        match self {
            VarDeclareResult::Created(id) => id,
            VarDeclareResult::Occupied(id) => id,
        }
    }
}

impl Scope {
    // Typ has no 'Default' trait in it, clippy. :)
    #[allow(clippy::new_without_default)]
    pub fn new(parent: Option<Id<Scope>>, expected_type: Option<Typ>) -> Self {
        Scope {
            is_looping: false,
            expected_type,
            parent,
            real_type: None,
            symbols: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Scope {
            is_looping: false,
            expected_type: None,
            parent: None,
            real_type: None,
            symbols: Vec::new(),
        }
    }

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

#[derive(Debug)]
pub struct ScopeStack {
    pub binder: *mut Binder,
    scopes: Vec<Id<Scope>>,
}

impl ScopeStack {
    pub fn empty(binder: *mut Binder) -> Self {
        ScopeStack {
            binder,
            scopes: Vec::new(),
        }
    }

    fn binder(&self) -> &Binder {
        unsafe { self.binder.as_ref().expect("Pointer failed") }
    }

    fn binder_mut(&mut self) -> &mut Binder {
        unsafe { self.binder.as_mut().expect("Pointer failed") }
    }

    pub fn push(&mut self, scope: Scope) {
        let id = self.binder_mut().scopes.alloc(scope);
        self.scopes.push(id);
    }

    pub fn pop(&mut self) -> Option<Id<Scope>> {
        self.scopes.pop()
    }

    pub fn current_id(&self) -> Id<Scope> {
        *self.scopes.last().unwrap()
    }

    pub fn current(&self) -> &Scope {
        self.binder().scopes.get(self.current_id()).unwrap()
    }

    pub fn current_mut(&mut self) -> &mut Scope {
        let id = self.current_id();
        self.binder_mut().scopes.get_mut(id).unwrap()
    }

    pub fn to_real_scopes(&self) -> Vec<&Scope> {
        let mut scopes = Vec::new();
        for scope_id in self.scopes.iter() {
            scopes.push(self.binder().scopes.get(*scope_id).unwrap());
        }
        scopes
    }

    pub fn lookup_var(
        &self,
        name: &str,
        storage: &mut SymbolStorage,
    ) -> Option<(Symbol, Id<Symbol>)> {
        for scope in self.to_real_scopes().iter().rev() {
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
        for scope in self.to_real_scopes().iter().rev() {
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
    pub(crate) scopes: Arena<Scope>,
    pub(crate) stack: ScopeStack,
    pub(crate) storage: SymbolStorage,
}

impl Binder {
    pub fn get_real_scope(&self, scope_id: Id<Scope>) -> &Scope {
        self.scopes.get(scope_id).unwrap()
    }

    pub fn empty(builtin: Option<&mut dyn LanguageBuiltin>) -> Self {
        let mut binder = Binder {
            block_result: None,
            diagnostics: Vec::new(),
            scopes: Arena::new(),
            stack: ScopeStack::empty(std::ptr::null_mut()),
            storage: SymbolStorage::new(),
        };
        binder.stack.binder = &mut binder;
        binder.stack.push(Scope::empty());
        binder.init_builtin(builtin.unwrap_or(&mut LunarBuiltin));
        binder
    }

    pub fn from_block(builtin: Option<&mut dyn LanguageBuiltin>, block: &lunar_ast::Block) -> Self {
        let mut binder = Self::empty(builtin);
        binder.block_result = Some(binder.real_visit_block(block));
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
