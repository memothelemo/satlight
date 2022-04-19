use std::collections::HashSet;

use crate::{hir::HirExpr, *};
use ast::{ExprVisitorLifetime, StmtVisitorLifetime, TypeVisitorLifetime};
use lunar_ast as ast;
use lunar_shared::Node;

#[derive(Debug)]
pub struct ScopeVisitor<'a> {
    pub checker: *mut Typechecker<'a>,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> ScopeVisitor<'a> {
    #[inline]
    pub fn alloc_node(&mut self, node: &'a dyn Node) -> Id<&'a dyn Node> {
        self.get_checker_mut().original_nodes.alloc(node)
    }

    #[inline]
    pub fn current_scope(&self) -> &Scope {
        self.get_checker().stack.current_scope()
    }

    #[inline]
    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.get_checker_mut().stack.current_scope_mut()
    }

    #[inline]
    pub fn get_checker(&self) -> &Typechecker<'a> {
        unsafe { &*self.checker }
    }

    #[inline]
    pub fn get_checker_mut(&mut self) -> &mut Typechecker<'a> {
        unsafe { &mut *self.checker }
    }

    #[inline]
    pub fn add_diagnostic(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn report_if_not_match(&mut self, left: Type, right: Type, span: ast::Span) {
        // check if it can be casted
        let checker = self.get_checker();
        if left.cast(&right, checker).is_none()
            && !matches!(left, Type::Invalid) | !matches!(right, Type::Invalid)
        {
            // do not log diagnostic if it is invalid, duh!
            #[allow(mutable_borrow_reservation_conflict)]
            self.add_diagnostic(Diagnostic {
                message: format!(
                    "Type '{}' is not matched with '{}'",
                    left.description(checker),
                    right.description(checker)
                ),
                span,
            });
        }
    }
}

impl<'a> Drop for ScopeVisitor<'a> {
    fn drop(&mut self) {
        #[allow(clippy::drop_copy)]
        drop(self.checker)
    }
}

impl<'a> ast::TypeVisitorLifetime<'a> for ScopeVisitor<'a> {
    type Output = Type;

    fn visit_type_reference(&mut self, node: &'a ast::TypeReference) -> Self::Output {
        let name = node.name().ty().as_name();
        match self.get_checker().stack.find_variable(&name, true) {
            VarSearchResult::Found(id) => self.get_checker().variables.get(id).unwrap().ty.clone(),
            VarSearchResult::NotFound => {
                self.add_diagnostic(Diagnostic {
                    message: format!("Type '{}' not found", name),
                    span: node.span(),
                });
                Type::Invalid
            }
        }
    }
}

impl<'a> ast::ExprVisitorLifetime<'a> for ScopeVisitor<'a> {
    type Output = hir::Expr<'a>;

    fn visit_bool_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        todo!()
    }

    fn visit_function_expr(&mut self, node: &'a ast::FunctionExpr) -> Self::Output {
        todo!()
    }

    fn visit_name_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        let name = node.ty().as_name();
        hir::Literal::new(
            match self.get_checker().stack.find_variable(&name, false) {
                VarSearchResult::Found(id) => {
                    self.get_checker().variables.get(id).unwrap().ty.clone()
                }
                VarSearchResult::NotFound => {
                    self.add_diagnostic(Diagnostic {
                        message: format!("Variable '{}' not found", name),
                        span: node.span(),
                    });
                    Type::Invalid
                }
            },
            self.alloc_node(node),
        )
        .into()
    }

    fn visit_number_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        hir::Literal::new(Type::Number, self.alloc_node(node)).into()
    }

    fn visit_nil_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        hir::Literal::new(Type::Nil, self.alloc_node(node)).into()
    }

    fn visit_str_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        todo!()
    }

    fn visit_table_ctor_expr(&mut self, node: &'a ast::TableCtor) -> Self::Output {
        todo!()
    }

    fn visit_varargs_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        todo!()
    }

    fn visit_binary_expr(&mut self, node: &'a ast::Binary) -> Self::Output {
        todo!()
    }

    fn visit_paren_expr(&mut self, node: &'a ast::Expr) -> Self::Output {
        self.visit_expr(node)
    }

    fn visit_suffixed_expr(&mut self, node: &'a ast::Suffixed) -> Self::Output {
        todo!()
    }

    fn visit_type_assertion_expr(&mut self, node: &'a ast::TypeAssertion) -> Self::Output {
        let base = self.visit_expr(node.base());
        let casted = self.visit_type_info(node.cast());

        // check if it can be casted
        self.report_if_not_match(base.ty(), casted.clone(), node.span());

        hir::TypeAssertion::new(Box::new(base), casted, self.alloc_node(node)).into()
    }

    fn visit_unary_expr(&mut self, node: &'a ast::Unary) -> Self::Output {
        todo!()
    }

    fn visit_suffix_kind_expr(&mut self, node: &'a ast::SuffixKind) -> Self::Output {
        todo!()
    }
}

impl<'a> ast::StmtVisitorLifetime<'a> for ScopeVisitor<'a> {
    type Output = hir::Stmt<'a>;

    fn visit_break_stmt(&mut self, node: &'a ast::Token) -> Self::Output {
        unimplemented!("break is not supported atm")
    }

    fn visit_call_stmt(&mut self, node: &'a ast::Expr) -> Self::Output {
        todo!()
    }

    fn visit_do_stmt(&mut self, node: &'a ast::DoStmt) -> Self::Output {
        todo!()
    }

    fn visit_function_assign_stmt(&mut self, node: &'a ast::FunctionAssign) -> Self::Output {
        todo!()
    }

    fn visit_generic_for_stmt(&mut self, node: &'a ast::GenericFor) -> Self::Output {
        todo!()
    }

    fn visit_if_stmt(&mut self, node: &'a ast::IfStmt) -> Self::Output {
        todo!()
    }

    fn visit_local_assign_stmt(&mut self, node: &'a ast::LocalAssign) -> Self::Output {
        // initialize variables before typechecking :)
        let variables = {
            let mut vars: Vec<hir::LocalAssignVariable<'a>> = Vec::new();
            let mut existing_names: HashSet<String> = HashSet::new();
            let exprlist = node.exprlist();
            // @TODO: tuple destructuring
            for (idx, name) in node.names().iter().enumerate() {
                let expr = exprlist.get(idx).map(|v| self.visit_expr(v));
                let (span, explicit) = name
                    .type_info()
                    .as_ref()
                    .map(|v| (Some(v.span()), Some(self.visit_type_info(v))))
                    .unwrap_or((None, None));

                let real_type = {
                    if explicit.is_none() && expr.is_none() {
                        Type::Nil
                    } else {
                        #[allow(clippy::or_fun_call)]
                        let (span, real_type) = expr
                            .as_ref()
                            .map(|v| (exprlist.get(idx).unwrap().span(), v.ty()))
                            .unwrap_or((name.type_info().as_ref().map(|v| v.span()).unwrap_or(name.span()), Type::Nil));

                        if let Some(explicit) = explicit.as_ref() {
                            self.report_if_not_match(real_type.clone(), explicit.clone(), span);
                        }

                        real_type
                    }
                };

                let real_name = name.name().ty().as_name();
                if existing_names.contains(&real_name) {
                    self.add_diagnostic(Diagnostic {
                        message: format!(
                            "Declaring {} within same statement is not allowed!",
                            &real_name
                        ),
                        span: name.span(),
                    });
                } else {
                    existing_names.insert(real_name.to_string());
                    self.get_checker_mut().declare_variable(
                        &real_name,
                        VariableKind::Variable,
                        Some(name.span()),
                        real_type,
                    );
                }

                vars.push(hir::LocalAssignVariable::new(
                    real_name,
                    name.span(),
                    explicit,
                    span,
                    expr,
                ));
            }
            let mut iter_exprs = exprlist.iter();
            iter_exprs.nth(node.names().len() - 1);
            for expr in iter_exprs {
                self.add_diagnostic(Diagnostic {
                    message: "Leftover expression".to_string(),
                    span: expr.span(),
                });
            }
            vars
        };
        hir::Stmt::LocalAssign(hir::LocalAssign::new(variables, self.alloc_node(node)))
    }

    fn visit_local_function_stmt(&mut self, node: &'a ast::LocalFunction) -> Self::Output {
        todo!()
    }

    fn visit_numeric_for_stmt(&mut self, node: &'a ast::NumericFor) -> Self::Output {
        todo!()
    }

    fn visit_repeat_stmt(&mut self, node: &'a ast::RepeatStmt) -> Self::Output {
        todo!()
    }

    fn visit_return_stmt(&mut self, _: &'a ast::ReturnStmt) -> Self::Output {
        unimplemented!("use 'visit_return_stmt_as_block' instead, required for StmtVisitor trait")
    }

    fn visit_while_stmt(&mut self, node: &'a ast::WhileStmt) -> Self::Output {
        todo!()
    }

    fn visit_var_assign_stmt(&mut self, node: &'a ast::VarAssign) -> Self::Output {
        todo!()
    }
}

impl<'a> ScopeVisitor<'a> {
    fn visit_return_stmt_as_block(&mut self, node: &'a ast::ReturnStmt) -> Type {
        let exprlist = node.exprlist();
        if exprlist.is_empty() {
            Type::Void
        } else if exprlist.len() > 1 {
            panic!("Multiple expressions returned is not supported (need to implement tuple type)");
        } else {
            self.visit_expr(exprlist.get(0).unwrap()).ty()
        }
    }
}

impl<'a> ast::AstVisitorLifetime<'a, 'a> for ScopeVisitor<'a> {
    type BlockOutput = hir::Block<'a>;
    type LastStmtOutput = Type;

    fn visit_block(&mut self, node: &'a ast::Block) -> Self::BlockOutput {
        self.get_checker_mut().stack.push_scope();
        let mut stmts = Vec::new();
        for stmt in node.stmts().iter() {
            stmts.push(self.visit_stmt(stmt));
        }
        self.get_checker_mut().stack.pop_scope();
        hir::Block::new(
            stmts,
            self.alloc_node(node),
            node.last_stmt()
                .as_ref()
                .map(|v| self.visit_last_stmt(v))
                .unwrap_or(Type::Void),
        )
    }

    fn visit_last_stmt(&mut self, node: &'a ast::Stmt) -> Self::LastStmtOutput {
        match node {
            ast::Stmt::Return(node) => self.visit_return_stmt_as_block(node),
            _ => Type::Void,
        }
    }
}
