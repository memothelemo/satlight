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

#[derive(Debug)]
pub struct Binder {
    pub scopes: Arena<Scope>,
    pub stack: Vec<Id<Scope>>,
    pub symbols: Arena<Symbol>,
}

use crate::{
    hir::{self, TypeParameter},
    types::Type,
};
use lunar_ast::{AstVisitor, Span};

impl Binder {
    #[allow(clippy::new_without_default)]
    pub fn new(block: &lunar_ast::File) -> (Self, hir::Block) {
        let mut binder = Self {
            scopes: Arena::new(),
            stack: Vec::new(),
            symbols: Arena::new(),
        };
        binder.init_intrisnics();

        let output = binder.visit_file(block);
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

    fn visit_file(&mut self, file: &lunar_ast::File) -> hir::Block {
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

impl Binder {
    pub fn push_scope(&mut self, kind: ScopeKind) {
        let scope = Scope::new(kind, self.stack.last().cloned());
        let scope_id = self.scopes.alloc(scope);
        self.stack.push(scope_id);
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }
}

impl Binder {
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

impl Binder {
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
