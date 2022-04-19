use super::*;
use id_arena::Id;

#[derive(Debug, PartialEq)]
pub struct Scope {
    pub parent: Option<Id<Scope>>,
    pub variables: Vec<Id<Variable>>,
}

impl Scope {
    pub fn new(parent: Option<Id<Scope>>) -> Self {
        Scope {
            parent,
            variables: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ScopeStack<'a> {
    pub(crate) checker: *mut Typechecker<'a>,
    pub(crate) stack: Vec<Id<Scope>>,
}

impl<'a> ScopeStack<'a> {
    pub fn uninit() -> Self {
        ScopeStack {
            checker: std::ptr::null_mut(),
            stack: Vec::new(),
        }
    }

    pub fn new(checker: *mut Typechecker<'a>) -> Self {
        ScopeStack {
            checker,
            stack: Vec::new(),
        }
    }
}

#[allow(unused)]
impl<'a> ScopeStack<'a> {
    /// dangerous method
    fn get_checker(&self) -> &Typechecker<'a> {
        unsafe { &*self.checker }
    }

    /// dangerous method
    fn get_checker_mut(&mut self) -> &mut Typechecker<'a> {
        unsafe { &mut *self.checker }
    }
}

impl<'a> ScopeStack<'a> {
    pub fn current_scope_id(&self) -> Id<Scope> {
        *self.stack.last().unwrap()
    }

    pub fn current_scope(&self) -> &Scope {
        self.get_checker()
            .scopes
            .get(self.current_scope_id())
            .unwrap()
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        unsafe {
            (*self.checker)
                .scopes
                .get_mut(self.current_scope_id())
                .unwrap()
        }
    }

    pub fn push_scope(&mut self) {
        let scope = Scope::new(self.stack.last().cloned());
        let scope_id = self.get_checker_mut().scopes.alloc(scope);
        self.stack.push(scope_id);
    }

    pub fn pop_scope(&mut self) {
        assert!(!self.stack.is_empty(), "Cannot pop of empty stack");
        self.stack.pop();
    }

    pub(crate) fn find_variable(&self, name: &str, ty: bool) -> VarSearchResult {
        for scope in self.stack.iter() {
            if let VarSearchResult::Found(id) = self.get_checker().find_variable(*scope, name, ty) {
                return VarSearchResult::Found(id);
            }
        }
        VarSearchResult::NotFound
    }
}

impl<'a> Drop for ScopeStack<'a> {
    fn drop(&mut self) {
        #[allow(clippy::drop_copy)]
        drop(self.checker);
    }
}
