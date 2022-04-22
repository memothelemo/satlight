use super::*;
use bind_ast::ExprKind;
use lunar_ast::{AstVisitor, Binary, ExprVisitor, Node, StmtVisitor, TypeVisitor};

impl TypeVisitor<'_> for Binder {
    type Output = Typ;

    fn visit_type_reference(&mut self, node: &lunar_ast::TypeReference) -> Self::Output {
        let real_name = node.name().ty().as_name();
        if let Some((_, id)) = self
            .stack
            .current()
            .lookup_typ(&real_name, &mut self.storage)
        {
            Typ::Variable(id)
        } else {
            self.diagnostics.push(Diagnostic::new(
                format!("Cannot find type `{}` in this scope", real_name),
                node.span(),
            ));
            Typ::Error
        }
    }
}

impl StmtVisitor<'_> for Binder {
    type Output = bind_ast::Stmt;

    fn visit_break_stmt(&mut self, node: &lunar_ast::Token) -> Self::Output {
        unimplemented!("Eh?")
    }

    fn visit_call_stmt(&mut self, node: &lunar_ast::Expr) -> Self::Output {
        todo!()
    }

    fn visit_do_stmt(&mut self, node: &lunar_ast::DoStmt) -> Self::Output {
        todo!()
    }

    fn visit_function_assign_stmt(&mut self, node: &lunar_ast::FunctionAssign) -> Self::Output {
        todo!()
    }

    fn visit_generic_for_stmt(&mut self, node: &lunar_ast::GenericFor) -> Self::Output {
        todo!()
    }

    fn visit_if_stmt(&mut self, node: &lunar_ast::IfStmt) -> Self::Output {
        todo!()
    }

    fn visit_local_assign_stmt(&mut self, node: &lunar_ast::LocalAssign) -> Self::Output {
        let mut variables = Vec::new();
        for (name, expr) in node.into_segments() {
            variables.push(bind_ast::LocalAssignVar {
                name: name.name().ty().as_name(),
                name_span: name.name().span(),
                explicit_type: name.type_info().as_ref().map(|v| self.visit_type_info(v)),
                expr: expr.map(|v| self.visit_expr(v)),
            });
        }
        bind_ast::Stmt::LocalAssign(bind_ast::LocalAssign {
            variables,
            span: node.span(),
        })
    }

    fn visit_local_function_stmt(&mut self, node: &lunar_ast::LocalFunction) -> Self::Output {
        todo!()
    }

    fn visit_numeric_for_stmt(&mut self, node: &lunar_ast::NumericFor) -> Self::Output {
        todo!()
    }

    fn visit_repeat_stmt(&mut self, node: &lunar_ast::RepeatStmt) -> Self::Output {
        todo!()
    }

    fn visit_return_stmt(&mut self, node: &lunar_ast::ReturnStmt) -> Self::Output {
        todo!()
    }

    fn visit_while_stmt(&mut self, node: &lunar_ast::WhileStmt) -> Self::Output {
        todo!()
    }

    fn visit_var_assign_stmt(&mut self, node: &lunar_ast::VarAssign) -> Self::Output {
        todo!()
    }

    fn visit_type_declaration_stmt(&mut self, node: &lunar_ast::TypeDeclaration) -> Self::Output {
        todo!()
    }
}

impl ExprVisitor<'_> for Binder {
    type Output = bind_ast::Expr;

    fn visit_bool_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        bind_ast::Expr::literal(Typ::Bool, node.span())
    }

    fn visit_function_expr(&mut self, node: &lunar_ast::FunctionExpr) -> Self::Output {
        todo!()
    }

    fn visit_name_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        let real_name = node.ty().as_name();
        if let Some((_, id)) = self
            .stack
            .current()
            .lookup_var(&real_name, &mut self.storage)
        {
            bind_ast::Expr::new(ExprKind::Name(Typ::Variable(id)), node.span())
        } else {
            self.diagnostics.push(Diagnostic::new(
                format!("Cannot find `{}` in this scope", real_name),
                node.span(),
            ));
            bind_ast::Expr::err(node.span())
        }
    }

    fn visit_number_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        bind_ast::Expr::literal(Typ::Number, node.span())
    }

    fn visit_nil_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        bind_ast::Expr::literal(Typ::Nil, node.span())
    }

    fn visit_str_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        bind_ast::Expr::literal(Typ::String, node.span())
    }

    fn visit_table_ctor_expr(&mut self, node: &lunar_ast::TableCtor) -> Self::Output {
        todo!()
    }

    fn visit_varargs_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        todo!()
    }

    fn visit_binary_expr(&mut self, node: &Binary) -> Self::Output {
        todo!()
    }

    fn visit_paren_expr(&mut self, node: &lunar_ast::Expr) -> Self::Output {
        self.visit_expr(node)
    }

    fn visit_suffixed_expr(&mut self, node: &lunar_ast::Suffixed) -> Self::Output {
        todo!()
    }

    fn visit_type_assertion_expr(&mut self, node: &lunar_ast::TypeAssertion) -> Self::Output {
        bind_ast::Expr::new(
            ExprKind::Assertion(
                Box::new(self.visit_expr(node.base())),
                self.visit_type_info(node.cast()),
            ),
            node.span(),
        )
    }

    fn visit_unary_expr(&mut self, node: &lunar_ast::Unary) -> Self::Output {
        todo!()
    }

    fn visit_suffix_kind_expr(&mut self, node: &lunar_ast::SuffixKind) -> Self::Output {
        todo!()
    }
}

impl AstVisitor<'_, '_> for Binder {
    type BlockOutput = bind_ast::Block;
    type LastStmtOutput = bind_ast::LastStmt;

    fn visit_block(&mut self, node: &lunar_ast::Block) -> Self::BlockOutput {
        let mut stmts = Vec::new();
        for stmt in node.stmts().iter() {
            stmts.push(self.visit_stmt(stmt));
        }
        let last_stmt = node
            .last_stmt()
            .as_ref()
            .map(|last_stmt| self.visit_last_stmt(last_stmt));
        bind_ast::Block {
            stmts,
            span: node.span(),
            last_stmt,
        }
    }

    fn visit_last_stmt(&mut self, node: &lunar_ast::Stmt) -> Self::LastStmtOutput {
        use lunar_ast::Stmt;
        match node {
            Stmt::Return(_) => todo!(),
            Stmt::Break(node) => {
                if !self.stack.current().is_looping {
                    self.diagnostics.push(Diagnostic::new(
                        "Cannot use `break` without a loop inside".to_string(),
                        node.span(),
                    ));
                    bind_ast::LastStmt::Error(node.span())
                } else {
                    bind_ast::LastStmt::Break
                }
            }
            _ => unreachable!(),
        }
    }
}
