use crate::{
    types::{Type, TypeTrait},
    *,
};
use id_arena::Id;
use salite_ast::Span;

mod nodes;
use salite_common::memory::SafePtr;

pub trait Transform<'a, 'b> {
    type Output: 'b;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output;
}

#[derive(Debug)]
pub struct Transformer<'a, 'b> {
    pub(crate) ctx: SafePtr<ModuleContext<'a, 'b>>,
    pub(crate) stack: Vec<Id<Scope>>,
    pub(crate) type_stack: Vec<String>,
}

impl<'a, 'b> Transformer<'a, 'b> {
    pub fn transform(
        ctx: SafePtr<ModuleContext<'a, 'b>>,
        file: &'b salite_ast::File,
    ) -> hir::File<'b> {
        let mut transformer = Self {
            ctx,
            stack: Vec::new(),
            type_stack: Vec::new(),
        };
        transformer.init_intrinsics();

        hir::File {
            block: file.block().transform(&mut transformer),
        }
    }

    pub(crate) fn init_intrinsics(&mut self) {
        macro_rules! lazy_declare {
			{as type = {
				$( $name:expr => $typ:expr, )*
			}} => {
				$( self.insert_type_alias($name, SymbolKind::TypeAlias(TypeAliasSymbol {
					name: $name.to_string(),
					typ: $typ,
					parameters: None,
				}), None); )*
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

        #[allow(unused_macros)]
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
    }
}

impl<'a, 'b> Transformer<'a, 'b> {
    pub(crate) fn combine_return_types(&mut self, typ: Type) {
        // We need to find a scope that is a:
        // - Module scope
        // - Function scope
        // - Has no expected type which it will be processed to the analyzer
        let mut parent = Some(self.current_scope_id());
        while let Some(real_parent) = parent {
            let mut scope = self.ctx.scopes.get_mut(real_parent).unwrap();
            parent = scope.parent;

            // check if it is a returnable scope and combine return types
            if scope.is_returnable() && scope.expected_type.is_none() {
                scope.actual_type = match &mut scope.actual_type {
                    Some(actual_type) if actual_type != &typ => match actual_type {
                        Type::Union(un) => {
                            un.members.push(typ);
                            break;
                        }
                        _ => Some(Type::Union(types::variants::Union {
                            span: actual_type.span(),
                            members: vec![actual_type.clone(), typ],
                        })),
                    },
                    None => Some(typ),
                    _ => break,
                };
                break;
            }
        }
    }

    pub(crate) fn push_scope(&mut self, kind: ScopeKind) {
        let scope = Scope::new(kind, self.stack.last().cloned());
        let scope_id = self.ctx.scopes.alloc(scope);
        self.stack.push(scope_id);
    }

    pub(crate) fn pop_scope(&mut self) {
        debug_assert!(!self.stack.is_empty(), "Attempt to pop the empty stack!");
        self.stack.pop();
    }
}

impl<'a, 'b> Transformer<'a, 'b> {
    pub(crate) fn current_scope_id(&self) -> Id<Scope> {
        debug_assert!(
            !self.stack.is_empty(),
            "Attempt to current scope with an empty stack!"
        );
        *self.stack.last().unwrap()
    }

    pub(crate) fn current_scope(&self) -> &Scope {
        self.ctx.scopes.get(self.current_scope_id()).unwrap()
    }

    pub(crate) fn current_scope_mut(&mut self) -> &mut Scope {
        let scope_id = self.current_scope_id();
        self.ctx.scopes.get_mut(scope_id).unwrap()
    }
}

impl<'a, 'b> Transformer<'a, 'b> {
    pub(crate) fn revisit_function_type(
        &mut self,
        mut base: types::variants::Function,
        assertion: types::variants::Function,
    ) -> (types::Type, types::Type) {
        for (idx, base_param) in base.parameters.iter_mut().enumerate() {
            if let Some(assertion_param) = assertion.parameters.get(idx) {
                if matches!(base_param.typ, types::Type::Any(..)) {
                    // overiding the parameter guess enough?
                    base_param.typ = assertion_param.typ.clone();
                    *base_param.typ.span_mut() = base_param.span;
                }
            }
        }
        (
            types::Type::Function(base),
            types::Type::Function(assertion),
        )
    }

    pub(crate) fn register_symbol(
        &mut self,
        definitions: Vec<Span>,
        kind: SymbolKind,
    ) -> Id<Symbol> {
        self.ctx.symbols.alloc(Symbol { definitions, kind })
    }

    pub(crate) fn insert_variable(
        &mut self,
        name: &str,
        kind: SymbolKind,
        span: Option<Span>,
    ) -> Id<Symbol> {
        let symbol_id =
            self.register_symbol(span.map(|v| vec![v]).unwrap_or(vec![Span::invalid()]), kind);

        let scope = self.current_scope_mut();
        scope.vars.insert(name.to_string(), symbol_id);
        symbol_id
    }

    pub(crate) fn insert_type_alias(
        &mut self,
        name: &str,
        kind: SymbolKind,
        span: Option<Span>,
    ) -> Id<Symbol> {
        let symbol_id =
            self.register_symbol(span.map(|v| vec![v]).unwrap_or(vec![Span::invalid()]), kind);

        let scope = self.current_scope_mut();
        scope.types.insert(name.to_string(), symbol_id);

        symbol_id
    }
}
