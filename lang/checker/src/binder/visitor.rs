use super::*;
use crate::types::Type;
use crate::{hir, types};
use lunar_ast as ast;
use lunar_ast::{AstVisitor, ExprVisitor, LastStmtVisitor, Node, StmtVisitor, TypeVisitor};
use std::borrow::Borrow;

impl TypeVisitor<'_> for Binder {
    type Output = Type;

    fn visit_type_callback(&mut self, node: &lunar_ast::TypeCallback) -> Self::Output {
        todo!()
    }

    fn visit_type_reference(&mut self, node: &lunar_ast::TypeReference) -> Self::Output {
        let scope = self.current_scope();
        let real_name = node.name().ty().as_name();

        let symbol = scope.find_symbol_type(self, &real_name);
        match symbol {
            Some(symbol_id) => {
                let symbol = self.symbols.get(symbol_id).unwrap();
                Type::Ref(types::RefType {
                    span: node.span(),
                    name: real_name,
                    symbol: symbol_id,
                    arguments: None,
                })
            }
            None => {
                println!("Unknown type variable: {}", real_name);
                types::makers::any(node.span())
            }
        }
    }
}

impl ExprVisitor<'_> for Binder {
    type Output = hir::Expr;

    fn visit_bool_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        hir::Expr::Literal(hir::Literal {
            typ: types::makers::bool(node.span()),
            span: node.span(),
            symbol: None,
        })
    }

    fn visit_function_expr(&mut self, node: &lunar_ast::FunctionExpr) -> Self::Output {
        todo!()
    }

    fn visit_name_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        let scope = self.current_scope();
        let real_name = node.ty().as_name();

        let symbol = scope.find_symbol_var(self, &real_name);
        match symbol {
            Some(symbol_id) => {
                let symbol = self.symbols.get(symbol_id).unwrap();
                hir::Expr::Literal(hir::Literal {
                    // meh?
                    typ: symbol.typ.clone().unwrap(),
                    symbol: Some(symbol_id),
                    span: node.span(),
                })
            }
            None => {
                println!("Unknown variable: {}", real_name);
                hir::Expr::Literal(hir::Literal {
                    typ: types::makers::any(node.span()),
                    symbol: None,
                    span: node.span(),
                })
            }
        }
    }

    fn visit_number_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        hir::Expr::Literal(hir::Literal {
            typ: types::makers::number(node.span()),
            span: node.span(),
            symbol: None,
        })
    }

    fn visit_nil_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        hir::Expr::Literal(hir::Literal {
            typ: types::makers::nil(node.span()),
            span: node.span(),
            symbol: None,
        })
    }

    fn visit_str_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        hir::Expr::Literal(hir::Literal {
            typ: types::makers::string(node.span()),
            span: node.span(),
            symbol: None,
        })
    }

    fn visit_table_ctor_expr(&mut self, node: &lunar_ast::TableCtor) -> Self::Output {
        todo!()
    }

    fn visit_varargs_expr(&mut self, node: &lunar_ast::Token) -> Self::Output {
        todo!()
    }

    fn visit_binary_expr(&mut self, node: &lunar_ast::Binary) -> Self::Output {
        todo!()
    }

    fn visit_paren_expr(&mut self, node: &lunar_ast::Expr) -> Self::Output {
        self.visit_expr(node)
    }

    fn visit_suffixed_expr(&mut self, node: &lunar_ast::Suffixed) -> Self::Output {
        todo!()
    }

    fn visit_type_assertion_expr(&mut self, node: &lunar_ast::TypeAssertion) -> Self::Output {
        hir::Expr::TypeAssertion(hir::TypeAssertion {
            base: Box::new(self.visit_expr(node.base().borrow())),
            cast: self.visit_type_info(node.cast()),
            symbol: None,
            span: node.span(),
        })
    }

    fn visit_unary_expr(&mut self, node: &lunar_ast::Unary) -> Self::Output {
        todo!()
    }

    fn visit_suffix_kind_expr(&mut self, node: &lunar_ast::SuffixKind) -> Self::Output {
        todo!()
    }
}

impl StmtVisitor<'_> for Binder {
    type Output = hir::Stmt;

    fn visit_call_stmt(&mut self, node: &lunar_ast::Expr) -> Self::Output {
        self.visit_expr(node);
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
        let exprs = {
            let mut exprs = Vec::new();
            for expr in node.exprlist().iter() {
                let expr = self.visit_expr(expr);
                let span = expr.span();
                let types = expr.typ().clone().deref_tuples();
                for typ in types {
                    exprs.push((span, typ));
                }
            }
            exprs
        };
        let mut variables = Vec::new();
        for (id, name) in node.names().iter().enumerate() {
            let real_name = name.name().ty().as_name();
            let (expr_source, expr) = exprs
                .get(id)
                .cloned()
                .map(|v| (Some(v.0), Some(v.1)))
                .unwrap_or((None, None));

            let symbol_id = self.register_symbol(
                SymbolFlags::BlockVariable,
                vec![name.span()],
                expr.clone().or(Some(types::makers::any(name.span()))),
            );

            let explicit_type = name.type_info().as_ref().map(|v| self.visit_type_info(v));
            variables.push(hir::LocalAssignVar {
                name: real_name,
                name_symbol: symbol_id,
                name_span: name.span(),
                explicit_type,
                expr_source,
                expr,
            });
        }

        hir::Stmt::LocalAssign(hir::LocalAssign {
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

    fn visit_while_stmt(&mut self, node: &lunar_ast::WhileStmt) -> Self::Output {
        todo!()
    }

    fn visit_var_assign_stmt(&mut self, node: &lunar_ast::VarAssign) -> Self::Output {
        todo!()
    }

    fn visit_type_declaration_stmt(&mut self, node: &lunar_ast::TypeDeclaration) -> Self::Output {
        // get all the type parameters first...
        let parameters = if let Some(real_params) = node.params() {
            let mut params: Vec<hir::TypeParameter> = Vec::new();
            for param in real_params.iter() {
                let real_name = param.name().ty().as_name();
                let explicit_type = param.typ().as_ref().map(|v| self.visit_type_info(v));
                let default_type = param.default().as_ref().map(|v| self.visit_type_info(v));

                params.push(hir::TypeParameter {
                    name: real_name,
                    name_span: param.name().span(),
                    explicit: explicit_type,
                    default: default_type,
                    span: param.span(),
                });
            }
            Some(params)
        } else {
            None
        };

        // recursive types are allowed :)
        // unless it has to be explicit...
        let real_name = node.name().ty().as_name();
        let symbol_id = self.declare_type_var(
            &real_name,
            SymbolFlags::TypeAlias,
            Some(node.name().span()),
            types::makers::any(node.name().span()),
        );

        self.push_scope(ScopeKind::TypeAliasValue);

        if let Some(ref parameters) = parameters {
            for param in parameters.iter() {
                // assume declare type variable?
                self.declare_type_var(
                    &param.name,
                    SymbolFlags::TypeParameter,
                    Some(param.name_span),
                    param
                        .explicit
                        .clone()
                        .or(param.default.clone())
                        .unwrap_or(types::makers::any(param.name_span)),
                );
            }
        }

        let value = self.visit_type_info(node.typ());
        self.pop_scope();

        let mut symbol = self.symbols.get_mut(symbol_id).unwrap();
        symbol.typ = Some(value.clone());

        hir::Stmt::TypeDeclaration(hir::TypeDeclaration {
            name: real_name,
            parameters,
            value,
        })
    }
}
impl AstVisitor<'_> for Binder {
    type BlockOutput = hir::Block;

    fn visit_block(&mut self, node: &lunar_ast::Block) -> Self::BlockOutput {
        let stmts = node
            .stmts()
            .iter()
            .map(|v| self.visit_stmt(v))
            .collect::<Vec<hir::Stmt>>();

        let last_stmt = node
            .last_stmt()
            .as_ref()
            .map(|v| match v.borrow() {
                ast::Stmt::Return(node) => {
                    let exprs = node
                        .exprlist()
                        .iter()
                        .map(|v| self.visit_expr(v))
                        .collect::<Vec<hir::Expr>>();

                    hir::LastStmt::Return(hir::Return {
                        exprs,
                        span: node.span(),
                    })
                }
                ast::Stmt::Break(node) => hir::LastStmt::Break(node.span()),
                _ => unreachable!(),
            })
            .unwrap_or(hir::LastStmt::None);

        hir::Block {
            stmts,
            last_stmt,
            span: node.span(),
        }
    }
}
