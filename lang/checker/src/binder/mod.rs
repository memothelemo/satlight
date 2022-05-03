use id_arena::{Arena, Id};

mod ctrl_flow;
mod scope;
mod symbols;

#[allow(unused)]
#[allow(clippy::or_fun_call)]
mod visitor;

pub use ctrl_flow::*;
pub use scope::*;
pub use symbols::*;
pub use visitor::*;

pub struct Binder<'a> {
    pub nodes: Arena<&'a dyn lunar_ast::Node>,
    pub scopes: Arena<Scope>,
    pub stack: Vec<Id<Scope>>,
    pub symbols: Arena<Symbol>,
}

impl<'a> std::fmt::Debug for Binder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binder")
            .field("scopes", &self.scopes)
            .field("stack", &self.stack)
            .field("symbols", &self.symbols)
            .finish()
    }
}

use crate::{
    hir::{self, TypeParameter},
    types::Type,
};
use lunar_ast::{AstVisitorWithLifetime, Span};

impl<'a> Binder<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new(block: &'a lunar_ast::File) -> (Binder<'a>, hir::File<'a>) {
        let mut binder: Binder<'a> = Self {
            nodes: Arena::new(),
            scopes: Arena::new(),
            stack: Vec::new(),
            symbols: Arena::new(),
        };
        binder.init_intrisnics();

        let block = binder.visit_file(block);
        let output = hir::File { block };

        (binder, output)
    }

    fn init_intrisnics(&mut self) {
        use crate::types;

        macro_rules! lazy_declare {
			{as type = {
				$( $name:expr => $typ:expr, )*
			}} => {
				$( self.declare_type_var($name, SymbolFlags::TypeAlias, None, $typ, None); )*
			};
		}

        self.push_scope(ScopeKind::Block);

        lazy_declare! {
            as type = {
                "any" => types::makers::any(Span::invalid()),
                "bool" => types::makers::bool(Span::invalid()),
                "number" => types::makers::number(Span::invalid()),
                "string" => types::makers::string(Span::invalid()),
                "unknown" => types::makers::unknown(Span::invalid()),
                "void" => types::makers::void(Span::invalid()),
            }
        }
    }

    fn visit_file(&mut self, file: &'a lunar_ast::File) -> hir::Block<'a> {
        self.push_scope(ScopeKind::Block);
        let block = self.visit_block(file.block());
        self.pop_scope();
        block
    }

    pub fn register_symbol(
        &mut self,
        flags: SymbolFlags,
        span: Vec<Span>,
        typ: Option<Type>,
        parameters: Option<Vec<TypeParameter>>,
    ) -> Id<Symbol> {
        self.symbols.alloc(Symbol {
            definitions: span,
            flags,
            id: self.symbols.len(),
            typ,
            parameters,
        })
    }
}

impl<'a> Binder<'a> {
    pub fn push_scope(&mut self, kind: ScopeKind) {
        let scope = Scope::new(kind, self.stack.last().cloned());
        let scope_id = self.scopes.alloc(scope);
        self.stack.push(scope_id);
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }
}

impl<'a> Binder<'a> {
    pub fn current_scope_id(&self) -> Id<Scope> {
        *self.stack.last().unwrap()
    }

    pub fn current_scope(&self) -> &Scope {
        self.scopes.get(self.current_scope_id()).unwrap()
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        let scope_id = self.current_scope_id();
        self.scopes.get_mut(scope_id).unwrap()
    }
}

impl<'a> Binder<'a> {
    pub fn declare_var(&mut self, name: &str, flags: SymbolFlags, span: Option<Span>, typ: Type) {
        let symbol_id = self.register_symbol(
            flags,
            span.map(|v| vec![v]).unwrap_or_default(),
            Some(typ),
            None,
        );
        let scope = self.current_scope_mut();
        scope.vars.insert(name.to_string(), symbol_id);
    }

    pub fn declare_type_var(
        &mut self,
        name: &str,
        flags: SymbolFlags,
        span: Option<Span>,
        typ: Type,
        parameters: Option<Vec<TypeParameter>>,
    ) -> Id<Symbol> {
        let symbol_id = self.register_symbol(
            flags,
            span.map(|v| vec![v]).unwrap_or_default(),
            Some(typ),
            parameters,
        );
        let scope = self.current_scope_mut();
        scope.types.insert(name.to_string(), symbol_id);
        symbol_id
    }
}
