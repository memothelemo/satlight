use crate::{types::TypeTrait, *};
use id_arena::Id;
use salite_ast::Span;

mod nodes;
pub use nodes::*;
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
        transformer.push_scope(ScopeKind::Module);
        hir::File {
            block: file.block().transform(&mut transformer),
        }
    }
}

impl<'a, 'b> Transformer<'a, 'b> {
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
