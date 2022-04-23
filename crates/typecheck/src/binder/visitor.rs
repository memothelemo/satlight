use crate::to_expr_typ;

use super::*;
use bind_ast::ExprKind;
use either::Either;
use lunar_ast::{AstVisitor, Binary, ExprVisitor, Node, StmtVisitor, TypeVisitor};

impl Binder {
    #[allow(clippy::type_complexity)]
    pub fn spread_exprlist(
        &mut self,
        list: &lunar_ast::ExprList,
    ) -> Either<(Vec<bind_ast::Expr>, Vec<(usize, Typ)>), lunar_ast::Span> {
        // destruct tuples
        let expr_sources = list
            .iter()
            .map(|v| self.visit_expr(v))
            .collect::<Vec<bind_ast::Expr>>();
        let expr_typs = {
            let mut typs = Vec::new();
            for (id, expr) in expr_sources.iter().enumerate() {
                if expr.is_error() {
                    return Either::Right(expr.span());
                }
                typs.append(
                    &mut self
                        .spread_tuples(to_expr_typ(expr, &self.storage))
                        .drain(..)
                        .map(|v| (id, v))
                        .collect::<Vec<(usize, Typ)>>(),
                );
            }
            typs
        };
        Either::Left((expr_sources, expr_typs))
    }

    pub fn spread_tuples(&self, typ: Typ) -> Vec<Typ> {
        match &typ {
            Typ::Tuple(typs) => {
                let mut result = Vec::new();
                for child in typs.iter() {
                    let mut result_typ = self.spread_tuples(child.clone());
                    result.append(&mut result_typ);
                }
                result
            }
            _ => vec![typ],
        }
    }

    #[inline]
    pub fn spread_tuples_multiple(&self, typs: Vec<Typ>) -> Vec<Typ> {
        let mut result = Vec::new();
        for typ in typs.iter() {
            result.append(&mut self.spread_tuples(typ.clone()));
        }
        result
    }

    pub fn real_visit_block(&mut self, node: &lunar_ast::Block) -> bind_ast::Block {
        let mut stmts = Vec::new();
        for stmt in node.stmts().iter() {
            stmts.push(self.visit_stmt(stmt));
        }

        let last_stmt = node
            .last_stmt()
            .as_ref()
            .map(|last_stmt| self.visit_last_stmt(last_stmt));

        bind_ast::Block {
            expected_type: self.stack.current().expected_type.clone(),
            stmts,
            scope: self.stack.current_id(),
            span: node.span(),
            last_stmt,
        }
    }
}

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

    fn visit_type_callback(&mut self, node: &lunar_ast::TypeCallback) -> Self::Output {
        Typ::Callback(TypCallback {
            return_typ: Box::new(self.visit_type_info(node.return_type())),
        })
    }
}

impl StmtVisitor<'_> for Binder {
    type Output = bind_ast::Stmt;

    fn visit_break_stmt(&mut self, node: &lunar_ast::Token) -> Self::Output {
        unimplemented!("Eh?")
    }

    fn visit_call_stmt(&mut self, node: &lunar_ast::Expr) -> Self::Output {
        let expr = self.visit_expr(node);
        bind_ast::Stmt::Call(expr)
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
        // spread tuples
        let (source, typs) = match self.spread_exprlist(node.exprlist()) {
            Either::Left(v) => v,
            Either::Right(err) => return bind_ast::Stmt::Error(err),
        };

        // local function cool() -> (number, number, number)
        //     return 1, 2, 3
        // end
        //       |<-|<-|<----------- number from cool's return type
        //       |        |<-------- number from a constant (3)
        // local a, b, c, d = cool(), 3;
        let mut segments = {
            let mut segments = Vec::new();
            for (id, name) in node.names().iter().enumerate() {
                segments.push((name, typs.get(id).cloned()));
            }
            segments
        };
        drop(typs);

        let mut variables = Vec::new();
        for (name, expr) in segments.drain(..) {
            let real_name = name.name().ty().as_name();
            let explicit_type = name.type_info().as_ref().map(|v| self.visit_type_info(v));
            variables.push(bind_ast::LocalAssignVar {
                name: real_name.to_string(),
                name_span: name.name().span(),
                explicit_type: explicit_type.clone(),
                expr: expr.clone().map(|(source_id, typ)| {
                    let source = source.get(source_id).unwrap().to_owned();
                    (source, typ, source_id)
                }),
            });
            self.stack.current_mut().try_declare(
                &real_name,
                #[allow(clippy::or_fun_call)]
                SymbolTyp::Variable(explicit_type.or(expr.as_ref().map(|v| v.1.clone())).unwrap_or(Typ::Nil)),
                &mut self.storage,
            );
        }
        drop(source);

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
        // we're going to deal with parameters later on. :)
        let body = node.body();
        let return_typ = body.return_type().as_ref().map(|v| self.visit_type_info(v));

        let mut scope = Scope::new(Some(self.stack.current_id()), return_typ.clone());
        for param in body.params().iter() {
            match param {
                lunar_ast::Param::Varargs(..) => {
                    unimplemented!("Varidiac arguments are not supported yet!")
                }
                lunar_ast::Param::Name(name) => {
                    // we do not support shadowing atm
                    if scope
                        .try_declare(
                            &name.ty().as_name(),
                            SymbolTyp::Variable(Typ::Any),
                            &mut self.storage,
                        )
                        .occupied()
                    {
                        self.diagnostics.push(Diagnostic::new(
                            "Shadowing variables is not supported!".to_string(),
                            param.span(),
                        ));
                        return bind_ast::Expr::err(node.span());
                    }
                }
            }
        }

        // visit there maybe?
        self.stack.push(scope);
        let block_result = self.real_visit_block(body.block());
        self.stack.pop();

        bind_ast::Expr::new(
            bind_ast::ExprKind::Callback(
                block_result,
                return_typ.unwrap_or(Typ::Nil),
            ),
            node.span(),
        )
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
        // @TODO: Lua metatable __call function support
        match node.suffix() {
            lunar_ast::SuffixKind::Call(args) => {
                let exprs = match args {
                    lunar_ast::Args::ExprList(list) => list
                        .iter()
                        .map(|v| self.visit_expr(v))
                        .collect::<Vec<bind_ast::Expr>>(),
                    lunar_ast::Args::Table(_) => todo!(),
                    lunar_ast::Args::Str(_) => todo!(),
                };
                bind_ast::Expr::new(
                    bind_ast::ExprKind::Call(Box::new(self.visit_expr(node.base())), exprs),
                    node.span(),
                )
            }
            lunar_ast::SuffixKind::Computed(_) => todo!(),
            lunar_ast::SuffixKind::Method(_) => todo!(),
            lunar_ast::SuffixKind::Name(_) => todo!(),
        }
    }

    fn visit_type_assertion_expr(&mut self, node: &lunar_ast::TypeAssertion) -> Self::Output {
        let expr = self.visit_expr(node.base());
        if expr.is_error() {
            return bind_ast::Expr::err(expr.span());
        }

        bind_ast::Expr::new(
            ExprKind::Assertion(Box::new(expr), self.visit_type_info(node.cast())),
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

    fn visit_block(&mut self, _: &lunar_ast::Block) -> Self::BlockOutput {
        unimplemented!("Use real_visit_block")
    }

    fn visit_last_stmt(&mut self, node: &lunar_ast::Stmt) -> Self::LastStmtOutput {
        use lunar_ast::Stmt;
        match node {
            Stmt::Return(node) => {
                let mut exprs = Vec::new();
                for expr in node.exprlist().iter() {
                    let result = self.visit_expr(expr);
                    if result.is_error() {
                        return bind_ast::LastStmt::Error(expr.span());
                    }
                    exprs.push(result);
                }

                // real types if there's no expected return type there
                let has_expected_type = {
                    let mut current = Some(self.stack.current_id());
                    'condition: while let Some(real_id) = current {
                        let scope = self.scopes.get(real_id).unwrap();
                        if scope.expected_type.is_some() {
                            break 'condition;
                        }
                        current = scope.parent;
                    }
                    false
                };

                if !has_expected_type {
                    // automatically set it then :)
                    self.stack.current_mut().real_type = Some(from_vec_exprs(exprs.to_vec(), &self.storage));
                }

                bind_ast::LastStmt::Return(exprs, node.span())
            }
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
