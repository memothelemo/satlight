mod diagnostics;
mod scopes;
mod types;
#[allow(unused)]
mod visitor;

use id_arena::{Arena, Id};

pub use diagnostics::*;
pub mod hir;
use lunar_ast::AstVisitorLifetime;
pub use scopes::*;
pub use types::*;
pub use visitor::*;

pub trait EnvProvider {
    fn load_std(&mut self, typechecker: &mut Typechecker<'_>);
}

pub struct LunarStandardProvider;

impl EnvProvider for LunarStandardProvider {
    fn load_std(&mut self, typechecker: &mut Typechecker<'_>) {
        typechecker.declare_variable("number", VariableKind::Type(), None, Type::Number);
        typechecker.declare_variable("nil", VariableKind::Type(), None, Type::Nil);
        typechecker.declare_variable("unknown", VariableKind::Type(), None, Type::Unknown);
        typechecker.declare_variable("void", VariableKind::Type(), None, Type::Void);
    }
}

pub struct Typechecker<'a> {
    pub(crate) original_nodes: Arena<&'a dyn lunar_shared::Node>,
    pub(crate) scopes: Arena<Scope>,
    pub(crate) stack: ScopeStack<'a>,
    pub(crate) top_scope: Option<Id<Scope>>,
    pub(crate) variables: Arena<Variable>,
    pub(crate) visitor: ScopeVisitor<'a>,
}

impl<'a> std::fmt::Debug for Typechecker<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Typechecker")
            .field("scopes", &self.scopes)
            .field("stack", &self.stack)
            .field("top_scope", &self.top_scope)
            .field("variables", &self.variables)
            .field("visitor", &self.visitor)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarSearchResult {
    Found(Id<Variable>),
    NotFound,
}

impl VarSearchResult {
    pub fn to_opt(&self) -> Option<Id<Variable>> {
        match self {
            VarSearchResult::Found(id) => Some(*id),
            VarSearchResult::NotFound => None,
        }
    }
}

impl<'a> Typechecker<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut checker = Self {
            original_nodes: Arena::new(),
            scopes: Arena::new(),
            stack: ScopeStack::uninit(),
            top_scope: None,
            variables: Arena::new(),
            visitor: ScopeVisitor {
                checker: std::ptr::null_mut(),
                diagnostics: Vec::new(),
            },
        };
        checker.stack.checker = &mut checker;
        checker.visitor.checker = &mut checker;
        checker
    }

    pub fn preload_enviroment(&mut self, provider: &mut dyn EnvProvider) {
        self.stack.push_scope();
        provider.load_std(self);
    }

    pub fn visit_block(&mut self, block: &'a lunar_ast::Block) -> hir::Block<'a> {
        self.visitor.visit_block(block)
    }

    pub fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.visitor.diagnostics
    }

    pub fn add_diagnostic(&mut self, diag: Diagnostic) {
        self.visitor.diagnostics.push(diag);
    }

    pub fn declare_variable(
        &mut self,
        name: &str,
        kind: VariableKind,
        span: Option<lunar_ast::Span>,
        ty: Type,
    ) {
        // check for existing variable marked as shadowed later on
        let shadowed = self.find_variable(
            self.stack.current_scope_id(),
            name,
            matches!(kind, VariableKind::Type(..)),
        );
        let variable = Variable {
            definitions: span.map(|v| vec![v]).unwrap_or_default(),
            kind,
            name: name.to_string(),
            shadowed: shadowed.to_opt(),
            ty,
        };
        let variable = self.variables.alloc(variable);
        self.stack.current_scope_mut().variables.push(variable);
    }

    pub(crate) fn find_variable(&self, scope: Id<Scope>, name: &str, ty: bool) -> VarSearchResult {
        for var_id in self.scopes.get(scope).unwrap().variables.iter().rev() {
            if let Some(var) = self.variables.get(*var_id) {
                if var.name == name && var.kind.is_type() == ty {
                    return VarSearchResult::Found(*var_id);
                }
            }
        }
        VarSearchResult::NotFound
    }
}
