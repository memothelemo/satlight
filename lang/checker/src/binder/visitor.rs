use super::*;
use crate::types::Type;
use crate::{hir, types};
use ast::AstVisitorWithLifetime;
use lunar_ast as ast;
use lunar_ast::{
    AstVisitor, ExprVisitorWithLifetime, LastStmtVisitorWithLifetime, SpannedNode,
    StmtVisitorWithLifetime, TypeVisitor,
};
use lunar_common::dictionary::Dictionary;
use std::borrow::Borrow;

impl<'a> TypeVisitor<'_> for Binder<'a> {
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
                let arguments = if let Some(arguments) = node.arguments() {
                    let mut list = Vec::new();
                    for arg in arguments.iter() {
                        list.push(self.visit_type_info(arg));
                    }
                    Some(list)
                } else {
                    None
                };
                Type::Ref(types::RefType {
                    span: node.span(),
                    name: real_name,
                    symbol: symbol_id,
                    arguments,
                })
            }
            None => {
                // ERROR: Unknown type variable: {}", real_name
                types::makers::any(node.span())
            }
        }
    }

    fn visit_type_table(&mut self, node: &ast::TypeTable) -> Self::Output {
        todo!()
    }
}

impl<'a> ExprVisitorWithLifetime<'a> for Binder<'a> {
    type Output = hir::Expr<'a>;

    fn visit_bool_expr(&mut self, node: &'a lunar_ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_function_expr(&mut self, node: &'a lunar_ast::FunctionExpr) -> Self::Output {
        todo!()
    }

    fn visit_name_expr(&mut self, node: &'a lunar_ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_number_expr(&mut self, node: &'a lunar_ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_nil_expr(&mut self, node: &'a lunar_ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_str_expr(&mut self, node: &'a lunar_ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_table_ctor_expr(&mut self, node: &'a lunar_ast::TableCtor) -> Self::Output {
        let mut fields = Vec::new();
        let mut entries = Dictionary::new();
        for field in node.fields().iter() {
            let field = match field {
                ast::TableField::Array(expr) => {
                    let expr = self.visit_expr(expr);
                    entries.insert(types::TableFieldKey::None, expr.typ().clone());
                    (hir::TableFieldKey::None, expr)
                }
                ast::TableField::Expr { span, index, value } => {
                    let index = self.visit_expr(index);
                    let value = self.visit_expr(value);
                    entries.insert(
                        types::TableFieldKey::Computed(index.typ().clone()),
                        value.typ().clone(),
                    );
                    (hir::TableFieldKey::Computed(index), value)
                }
                ast::TableField::Named { span, name, value } => {
                    let value = self.visit_expr(value);
                    entries.insert(
                        types::TableFieldKey::Name(name.ty().as_name(), name.span()),
                        value.typ().clone(),
                    );
                    (
                        hir::TableFieldKey::Name(name.ty().as_name(), name.span()),
                        value,
                    )
                }
            };
            fields.push(field);
        }
        let typ = types::Type::Table(types::Table {
            span: node.span(),
            entries,
            metatable: None,
        });
        hir::Expr::Table(hir::Table {
            span: node.span(),
            typ,
            node_id: self.nodes.alloc(node),
            fields,
            symbol: None,
        })
    }

    fn visit_varargs_expr(&mut self, node: &'a lunar_ast::Token) -> Self::Output {
        todo!()
    }

    fn visit_binary_expr(&mut self, node: &'a lunar_ast::Binary) -> Self::Output {
        todo!()
    }

    fn visit_paren_expr(&mut self, node: &'a lunar_ast::Expr) -> Self::Output {
        self.visit_expr(node)
    }

    fn visit_suffixed_expr(&mut self, node: &'a lunar_ast::Suffixed) -> Self::Output {
        todo!()
    }

    fn visit_type_assertion_expr(&mut self, node: &'a lunar_ast::TypeAssertion) -> Self::Output {
        hir::Expr::TypeAssertion(hir::TypeAssertion {
            base: Box::new(self.visit_expr(node.base().borrow())),
            cast: self.visit_type_info(node.cast()),
            symbol: None,
            span: node.span(),
            node_id: self.nodes.alloc(node),
        })
    }

    fn visit_unary_expr(&mut self, node: &'a lunar_ast::Unary) -> Self::Output {
        todo!()
    }

    fn visit_suffix_kind_expr(&mut self, node: &'a lunar_ast::SuffixKind) -> Self::Output {
        todo!()
    }

    fn visit_literal_expr(&mut self, base: &'a ast::Literal) -> Self::Output {
        match base {
            ast::Literal::Bool(node) => hir::Expr::Literal(hir::Literal {
                typ: types::makers::bool(node.span()),
                span: node.span(),
                symbol: None,
                node_id: self.nodes.alloc(base),
            }),
            ast::Literal::Function(node) => self.visit_function_expr(node),
            ast::Literal::Name(node) => {
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
                            node_id: self.nodes.alloc(base),
                        })
                    }
                    None => {
                        // ERROR: "Unknown variable: {}", real_name
                        hir::Expr::Literal(hir::Literal {
                            typ: types::makers::any(node.span()),
                            symbol: None,
                            span: node.span(),
                            node_id: self.nodes.alloc(base),
                        })
                    }
                }
            }
            ast::Literal::Number(node) => hir::Expr::Literal(hir::Literal {
                typ: types::makers::number(node.span()),
                span: node.span(),
                symbol: None,
                node_id: self.nodes.alloc(base),
            }),
            ast::Literal::Nil(node) => hir::Expr::Literal(hir::Literal {
                typ: types::makers::nil(node.span()),
                span: node.span(),
                symbol: None,
                node_id: self.nodes.alloc(base),
            }),
            ast::Literal::Str(node) => hir::Expr::Literal(hir::Literal {
                typ: types::makers::string(node.span()),
                span: node.span(),
                symbol: None,
                node_id: self.nodes.alloc(base),
            }),
            ast::Literal::Table(node) => self.visit_table_ctor_expr(node),
            ast::Literal::Varargs(node) => self.visit_varargs_expr(node),
        }
    }
}

impl<'a> StmtVisitorWithLifetime<'a> for Binder<'a> {
    type Output = hir::Stmt<'a>;

    fn visit_call_stmt(&mut self, node: &'a lunar_ast::Expr) -> Self::Output {
        self.visit_expr(node);
        todo!()
    }

    fn visit_do_stmt(&mut self, node: &'a lunar_ast::DoStmt) -> Self::Output {
        todo!()
    }

    fn visit_function_assign_stmt(&mut self, node: &'a lunar_ast::FunctionAssign) -> Self::Output {
        todo!()
    }

    fn visit_generic_for_stmt(&mut self, node: &'a lunar_ast::GenericFor) -> Self::Output {
        todo!()
    }

    fn visit_if_stmt(&mut self, node: &'a lunar_ast::IfStmt) -> Self::Output {
        todo!()
    }

    fn visit_local_assign_stmt(&mut self, node: &'a lunar_ast::LocalAssign) -> Self::Output {
        let exprs = {
            let mut exprs = Vec::new();
            for expr in node.exprlist().iter() {
                let expr_value = self.visit_expr(expr);
                let span = expr_value.span();
                let types = expr_value.typ().clone().deref_tuples();
                for typ in types {
                    exprs.push((span, typ, expr_value.clone()));
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
                None,
            );

            let explicit_type = name.type_info().as_ref().map(|v| self.visit_type_info(v));
            variables.push(hir::LocalAssignVar {
                name: real_name,
                name_symbol: symbol_id,
                name_span: name.span(),
                explicit_type,
                expr_source,
                expr_id: id,
                expr,
            });
        }

        hir::Stmt::LocalAssign(hir::LocalAssign {
            variables,
            span: node.span(),
            node_id: self.nodes.alloc(node),
            exprs: exprs.iter().map(|v| v.2.clone()).collect(),
        })
    }

    fn visit_local_function_stmt(&mut self, node: &'a lunar_ast::LocalFunction) -> Self::Output {
        todo!()
    }

    fn visit_numeric_for_stmt(&mut self, node: &'a lunar_ast::NumericFor) -> Self::Output {
        todo!()
    }

    fn visit_repeat_stmt(&mut self, node: &'a lunar_ast::RepeatStmt) -> Self::Output {
        todo!()
    }

    fn visit_while_stmt(&mut self, node: &'a lunar_ast::WhileStmt) -> Self::Output {
        todo!()
    }

    fn visit_var_assign_stmt(&mut self, node: &'a lunar_ast::VarAssign) -> Self::Output {
        todo!()
    }

    fn visit_type_declaration_stmt(
        &mut self,
        node: &'a lunar_ast::TypeDeclaration,
    ) -> Self::Output {
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
            parameters.clone(),
        );

        self.push_scope(ScopeKind::TypeAliasValue);

        // declare all of the parameters in an
        // isolated type declaration scope
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
                    None,
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
            node_id: self.nodes.alloc(node),
        })
    }
}

impl<'a> AstVisitorWithLifetime<'a> for Binder<'a> {
    type BlockOutput = hir::Block<'a>;

    fn visit_block(&mut self, node: &'a lunar_ast::Block) -> Self::BlockOutput {
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
                        node_id: self.nodes.alloc(node),
                    })
                }
                c @ ast::Stmt::Break(n) => hir::LastStmt::Break(n.span(), self.nodes.alloc(c)),
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
