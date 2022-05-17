mod scope;
mod symbol;

use std::{path::PathBuf, sync::Arc};

use id_arena::Arena;
use salite_ast::Node;
use salite_common::{memory::SafePtr, Config as ProjectCfg};

pub use scope::*;
pub use symbol::*;

use crate::{hir, Diagnostic, DiagnosticLevel, Transformer};

#[derive(Debug)]
pub struct ModuleResult<'a, 'b> {
    pub ctx: Arc<ModuleContext<'a, 'b>>,
    pub file: hir::File<'b>,
}

#[derive(Debug)]
pub struct EnvContext<'a, 'b> {
    pub(crate) cfg: &'a ProjectCfg,
    pub(crate) modules: Vec<(PathBuf, ModuleResult<'a, 'b>)>,
}

unsafe impl<'a, 'b> std::marker::Send for EnvContext<'a, 'b> {}

unsafe impl<'a, 'b> std::marker::Sync for EnvContext<'a, 'b> {}

impl<'a, 'b> EnvContext<'a, 'b> {
    pub fn cfg(&self) -> &'a ProjectCfg {
        self.cfg
    }

    pub fn modules_mut(&mut self) -> &mut Vec<(PathBuf, ModuleResult<'a, 'b>)> {
        &mut self.modules
    }

    pub fn modules(&self) -> &Vec<(PathBuf, ModuleResult<'a, 'b>)> {
        &self.modules
    }

    pub fn new(cfg: &'a ProjectCfg) -> Self {
        Self {
            cfg,
            modules: Vec::new(),
        }
    }

    pub fn add_module(
        &mut self,
        path: PathBuf,
        file: &'b salite_ast::File,
    ) -> &ModuleResult<'a, 'b> {
        let ptr = SafePtr::from_ptr(self as *mut _);
        let mut ctx = ModuleContext::new(*file.declaration(), ptr, Some(path.clone()));
        let file = Transformer::transform(SafePtr::from_ptr(&mut ctx as *mut _), file);
        self.modules.push((
            path.clone(),
            ModuleResult {
                ctx: Arc::new(ctx),
                file,
            },
        ));

        self.get_module_result(&path).unwrap()
    }

    pub fn get_module_result(&self, path: &PathBuf) -> Option<&ModuleResult<'a, 'b>> {
        for (entry_path, result) in self.modules.iter() {
            if entry_path == path {
                return Some(result);
            }
        }
        None
    }
}

pub struct ModuleContext<'env, 'node> {
    pub(crate) declaration: bool,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) env: SafePtr<EnvContext<'env, 'node>>,
    pub(crate) file_path: Option<PathBuf>,
    pub(crate) nodes: Arena<&'node dyn Node>,
    pub(crate) scopes: Arena<Scope>,
    pub(crate) symbols: Arena<Symbol>,
}

impl<'a, 'b> std::fmt::Debug for ModuleContext<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleContext")
            .field("diagnostics", &self.diagnostics)
            .field("env", &self.env)
            .field("file_path", &self.file_path)
            .field("scopes", &self.scopes)
            .field("symbols", &self.symbols)
            .finish()
    }
}

impl<'env, 'node> ModuleContext<'env, 'node> {
    pub fn new(
        declaration: bool,
        env: SafePtr<EnvContext<'env, 'node>>,
        file_path: Option<PathBuf>,
    ) -> Self {
        Self {
            env,
            declaration,
            diagnostics: Vec::new(),
            file_path,
            scopes: Arena::new(),
            symbols: Arena::new(),
            nodes: Arena::new(),
        }
    }

    pub(crate) fn add_diagnostic(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }

    pub fn has_errors(&self) -> bool {
        for diag in self.diagnostics.iter() {
            if diag.level() == DiagnosticLevel::Error {
                return true;
            }
        }
        false
    }
}
