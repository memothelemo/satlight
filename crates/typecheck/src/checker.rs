use lunar_config::CompilerOptions;

use crate::*;

#[derive(Debug)]
pub struct Typechecker {
    binders: Vec<Binder>,
    diagnostics: Vec<Diagnostic>,
    opt: CompilerOptions,
}

pub type CheckResult = Result<(), Diagnostic>;

impl Typechecker {
    #[allow(clippy::new_without_default)]
    pub fn new(opt: CompilerOptions) -> Self {
        Typechecker {
            binders: Vec::new(),
            opt,
            diagnostics: Vec::new(),
        }
    }

    pub fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }

    pub fn options(&self) -> &CompilerOptions {
        &self.opt
    }

    pub fn bind_block(
        &mut self,
        block: &lunar_ast::Block,
        builtin: Option<&mut dyn LanguageBuiltin>,
    ) {
        let mut binder = Binder::from_block(builtin, block);
        self.diagnostics.append(&mut binder.diagnostics);
        self.binders.push(binder);
    }

    pub fn check_all(&mut self) {
        let checker_ptr: *mut Typechecker = self;
        for binder in self.binders.iter() {
            // the reason we did this for compatibility to multithreading checking
            let mut visitor = TypecheckVisitor {
                binder,
                checker: checker_ptr,
            };
            if let Err(diag) =
                visitor.resolve_block(binder.block_result().as_ref().expect("No output"))
            {
                self.diagnostics.push(diag);
            }
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct TypecheckVisitor<'a> {
    binder: &'a Binder,
    checker: *mut Typechecker,
}

#[allow(unused)]
impl<'a> TypecheckVisitor<'a> {
    fn checker(&self) -> &Typechecker {
        unsafe { self.checker.as_ref().expect("No typechecker") }
    }

    fn checker_mut(&mut self) -> &mut Typechecker {
        unsafe { self.checker.as_mut().expect("No typechecker") }
    }

    pub fn caught_error_node(&self, span: lunar_ast::Span) -> CheckResult {
        Err(Diagnostic::new("Caught error node".to_string(), span))
    }

    pub fn resolve_typ(&self, typ: Typ, span: lunar_ast::Span) -> CheckResult {
        if matches!(typ, Typ::Error) {
            self.caught_error_node(span)
        } else {
            Ok(())
        }
    }

    pub fn typ_description(&self, typ: Typ) -> String {
        match typ {
            Typ::Error => "!!ERROR!!".to_string(),
            Typ::Number => "number".to_string(),
            Typ::Nil => "nil".to_string(),
            Typ::Unknown => "unknown".to_string(),
            Typ::Void => "void".to_string(),
            Typ::Bool => "boolean".to_string(),
            Typ::String => "string".to_string(),
            Typ::Variable(id) => self.binder.storage.get_symbol(id).unwrap().name,
        }
    }

    #[inline]
    pub fn skip_downwards(&self, typ: Typ) -> Typ {
        // instead of recursion, we use while loops
        let mut result = typ;
        while let Typ::Variable(id) = result {
            result = match self.binder.storage.get_symbol(id).unwrap().typ {
                SymbolTyp::Variable(v) => v,
                SymbolTyp::Type(v) => v,
            };
        }
        result
    }

    pub fn resolve_type_match(
        &mut self,
        left: Typ,
        right: Typ,
        span: lunar_ast::Span,
    ) -> CheckResult {
        let message = format!(
            "`{}` does not match with `{}`",
            self.typ_description(left),
            self.typ_description(right)
        );
        let left = self.skip_downwards(left);
        let right = self.skip_downwards(right);
        match (left, right) {
            (_, Typ::Unknown) => Ok(()),
            (Typ::Void, Typ::Nil) => Ok(()),
            _ if left == right => Ok(()),
            _ => Err(Diagnostic::new(message, span)),
        }
    }

    pub fn resolve_cast_match(
        &mut self,
        left: Typ,
        right: Typ,
        span: lunar_ast::Span,
    ) -> CheckResult {
        let message = format!(
            "`{}` does not match with `{}`",
            self.typ_description(left),
            self.typ_description(right)
        );
        let left = self.skip_downwards(left);
        let right = self.skip_downwards(right);
        match (left, right) {
            (_, Typ::Unknown) => Ok(()),
            (Typ::Unknown, _) => Ok(()),
            (Typ::Nil, Typ::Void) => Ok(()),
            (Typ::Void, Typ::Nil) => Ok(()),
            _ if left == right => Ok(()),
            _ => Err(Diagnostic::new(message, span)),
        }
    }

    pub fn resolve_expr(&mut self, expr: &bind_ast::Expr) -> CheckResult {
        match &expr.kind {
            bind_ast::ExprKind::Assertion(base, cast) => {
                self.resolve_cast_match(base.typ(), *cast, expr.span())
            }
            bind_ast::ExprKind::Error => self.caught_error_node(expr.span),
            bind_ast::ExprKind::Literal(n) => self.resolve_typ(*n, expr.span()),
            bind_ast::ExprKind::Name(n) => self.resolve_typ(*n, expr.span()),
        }
    }

    pub fn resolve_local_assign(&mut self, stmt: &bind_ast::LocalAssign) -> CheckResult {
        for variable in stmt.variables().iter() {
            #[allow(clippy::or_fun_call)]
            let (span, real_type) = variable
                .expr()
                .as_ref()
                .map(|v| (v.span(), v.typ()))
                .unwrap_or((*variable.name_span(), Typ::Nil));

            // we're going to evaluate if explicit type matches with real type
            if let Some(explicit_type) = variable.explicit_type() {
                self.resolve_type_match(*explicit_type, real_type, span)?;
            }
        }
        Ok(())
    }

    pub fn resolve_stmt(&mut self, stmt: &bind_ast::Stmt) -> CheckResult {
        match stmt {
            bind_ast::Stmt::Error(span) => self.caught_error_node(*span),
            bind_ast::Stmt::LocalAssign(node) => self.resolve_local_assign(node),
        }
    }

    pub fn resolve_last_stmt(&mut self, stmt: &bind_ast::LastStmt) -> CheckResult {
        match stmt {
            bind_ast::LastStmt::Break => Ok(()),
            bind_ast::LastStmt::Error(span) => self.caught_error_node(*span),
        }
    }

    pub fn resolve_block(&mut self, block: &bind_ast::Block) -> CheckResult {
        for stmt in block.stmts().iter() {
            self.resolve_stmt(stmt)?;
        }
        if let Some(stmt) = block.last_stmt() {
            self.resolve_last_stmt(stmt)?;
        }
        Ok(())
    }
}

// impl<'a> Drop for TypecheckVisitor<'a> {
//     fn drop(&mut self) {
//         unsafe {
//             self.checker.drop_in_place();
//         }
//     }
// }
