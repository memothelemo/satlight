use id_arena::{Arena, Id};
use salite_common::dictionary::Dictionary;

mod ctrl_flow;
mod diagnostics;
mod scope;
mod symbols;

#[allow(unused)]
#[allow(clippy::or_fun_call)]
mod visitor;

use crate::{
    hir::{self, TypeParameter},
    types::Type,
};
pub use ctrl_flow::*;
pub use diagnostics::*;
pub use scope::*;
pub use symbols::*;
pub use visitor::*;

use salite_ast::Span;

pub struct Binder<'a> {
    pub diagnostics: Vec<Diagnostic>,
    pub nodes: Arena<&'a dyn salite_ast::Node>,
    pub scopes: Arena<Scope>,
    pub stack: Vec<Id<Scope>>,
    pub symbols: Arena<Symbol>,
    pub var_globals: Dictionary<String, Id<Symbol>>,
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

impl<'a> Binder<'a> {
    pub fn new(block: &'a salite_ast::File) -> (Binder<'a>, hir::File<'a>) {
        let mut binder: Binder<'a> = Self {
            diagnostics: Vec::new(),
            var_globals: Dictionary::new(),
            nodes: Arena::new(),
            scopes: Arena::new(),
            stack: Vec::new(),
            symbols: Arena::new(),
        };
        binder.load_intrinsics();

        let block = binder.visit_file(block);
        let output = hir::File { block };

        (binder, output)
    }

    pub fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }

    pub(crate) fn load_intrinsics(&mut self) {
        use crate::types;

        macro_rules! lazy_declare {
			{as type = {
				$( $name:expr => $typ:expr, )*
			}} => {
				$( self.insert_type_alias($name, SymbolFlags::TypeAlias, None, $typ, None); )*
			};
		}

        self.push_scope(ScopeKind::Module);

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

        macro_rules! any_table {
            () => {
                Type::Table(types::Table {
                    is_metatable: false,
                    span: Span::invalid(),
                    entries: {
                        let mut dictionary = Dictionary::new();
                        dictionary.insert(
                            types::TableFieldKey::Computed(types::makers::any(Span::invalid())),
                            types::makers::any(Span::invalid()),
                        );
                        dictionary
                    },
                    metatable: None,
                })
            };
        }

        let symbol_id = self.insert_variable(
            "setmetatable",
            SymbolFlags::Intrinsic,
            Some(Span::invalid()),
            Type::Function(types::FunctionType {
                span: Span::invalid(),
                parameters: vec![
                    types::FunctionParameter {
                        optional: false,
                        span: Span::invalid(),
                        name: Some("table".to_string()),
                        typ: any_table!(),
                    },
                    types::FunctionParameter {
                        optional: false,
                        span: Span::invalid(),
                        name: Some("metatable".to_string()),
                        typ: any_table!(),
                    },
                ],
                varidiac_param: None,
                return_type: Box::new(types::makers::void(Span::invalid())),
            }),
        );
        self.var_globals
            .insert("setmetatable".to_string(), symbol_id);
    }

    pub(crate) fn register_symbol(
        &mut self,
        definitions: Vec<Span>,
        flags: SymbolFlags,
        typ: Option<Type>,
        type_parameters: Option<Vec<TypeParameter>>,
    ) -> Id<Symbol> {
        self.symbols.alloc(Symbol {
            definitions,
            flags,
            typ,
            type_parameters,
        })
    }
}

impl<'a> Binder<'a> {
    pub(crate) fn push_scope(&mut self, kind: ScopeKind) {
        let scope = Scope::new(kind, self.stack.last().cloned());
        let scope_id = self.scopes.alloc(scope);
        self.stack.push(scope_id);
    }

    pub(crate) fn pop_scope(&mut self) {
        self.stack.pop();
    }
}

impl<'a> Binder<'a> {
    pub(crate) fn current_scope_id(&self) -> Id<Scope> {
        *self.stack.last().unwrap()
    }

    pub(crate) fn current_scope(&self) -> &Scope {
        self.scopes.get(self.current_scope_id()).unwrap()
    }

    pub(crate) fn current_scope_mut(&mut self) -> &mut Scope {
        let scope_id = self.current_scope_id();
        self.scopes.get_mut(scope_id).unwrap()
    }
}

#[allow(unused)]
impl<'a> Binder<'a> {
    pub(crate) fn insert_variable(
        &mut self,
        name: &str,
        flags: SymbolFlags,
        span: Option<Span>,
        typ: Type,
    ) -> Id<Symbol> {
        // register a new variable :)
        let symbol_id = self.register_symbol(
            span.map(|v| vec![v]).unwrap_or(vec![Span::invalid()]),
            flags,
            Some(typ),
            None,
        );
        let scope = self.current_scope_mut();
        scope.vars.insert(name.to_string(), symbol_id);
        symbol_id
    }

    pub(crate) fn insert_type_alias(
        &mut self,
        name: &str,
        flags: SymbolFlags,
        span: Option<Span>,
        typ: Type,
        parameters: Option<Vec<TypeParameter>>,
    ) -> Id<Symbol> {
        let symbol_id = self.register_symbol(
            span.map(|v| vec![v]).unwrap_or(vec![Span::invalid()]),
            flags,
            Some(typ),
            parameters,
        );
        let scope = self.current_scope_mut();
        scope.types.insert(name.to_string(), symbol_id);
        symbol_id
    }
}
