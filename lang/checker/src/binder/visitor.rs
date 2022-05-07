use super::*;
use crate::types;
use ast::SpannedNode;
use salite_ast as ast;
use salite_common::dictionary::Dictionary;
use std::borrow::Borrow;

macro_rules! literal_macro {
    ($self:expr, $typ:expr, $node:expr, $base:expr, $symbol:expr) => {
        hir::Expr::Literal(hir::Literal {
            span: $node.span(),
            symbol: $symbol,
            typ: $typ,
            node_id: $self.nodes.alloc($base),
        })
    };
}

macro_rules! invalid_lib_use {
    ($self:expr, $span:expr, $lib:expr) => {
        $self.diagnostics.push(Diagnostic::InvalidLibraryUse {
            lib: $lib.to_string(),
            span: $span,
        });
    };
}

impl<'a> Binder<'a> {
    pub(crate) fn visit_call_expr_inner(
        &mut self,
        node: &'a ast::Suffixed,
        args: &'a ast::Args,
    ) -> hir::Expr<'a> {
        let mut arguments = Vec::new();
        let base = self.visit_expr(node.base().borrow());

        match args {
            ast::Args::ExprList(list) => {
                for expr in list.iter() {
                    arguments.push(self.visit_expr(expr));
                }
            }
            ast::Args::Table(arg) => {
                arguments.push(self.visit_table_ctor_expr(arg));
            }
            ast::Args::Str(arg) => arguments.push(literal_macro!(
                self,
                types::makers::string(node.span()),
                arg,
                node,
                None
            )),
        };

        let procrast_type = if let Type::Procrastinated(symbol_id, span) = &base.typ() {
            Some(Type::CallProcrastinated(*symbol_id, *span))
        } else {
            Some(types::makers::any(node.span()))
        };

        hir::Expr::Call(hir::Call {
            span: node.span(),
            procrast_type,
            base: Box::new(base),
            arguments,
        })
    }

    pub(crate) fn eval_lua_call_globals(
        &mut self,
        name: &str,
        node: &'a ast::Suffixed,
        args: &'a ast::Args,
    ) -> hir::Expr<'a> {
        match name {
            "setmetatable" => {
                let list = match args {
                    ast::Args::ExprList(list) => list,
                    _ => {
                        invalid_lib_use!(self, node.span(), "setmetatable");
                        return self.visit_call_expr_inner(node, args);
                    }
                };

                // procrastinate the setmetatable check and set its symbol
                let base = if let Some(arg) = list.get(0) {
                    self.visit_expr(arg)
                } else {
                    invalid_lib_use!(self, node.span(), "setmetatable");
                    return self.visit_call_expr_inner(node, args);
                };

                let mut metatable = if let Some(arg) = list.get(1) {
                    self.visit_expr(arg)
                } else {
                    invalid_lib_use!(self, node.span(), "setmetatable");
                    return self.visit_call_expr_inner(node, args);
                };

                let symbol_id =
                    self.register_symbol(vec![node.span()], true, SymbolFlags::Value, None, None);

                // making a procrastinated type
                let typ = types::makers::procrastinated(symbol_id, node.span());
                let scope = self.current_scope_mut();

                if let Some(base_symbol) = base.symbol() {
                    scope.facts.vars.insert(base_symbol, symbol_id);
                }

                let symbol = self.symbols.get_mut(symbol_id).unwrap();
                symbol.typ = Some(typ.clone());

                hir::Expr::Library(hir::LibraryExpr::SetMetatable(hir::SetMetatable {
                    return_type: typ,
                    span: node.span(),
                    base_symbol: base.symbol(),
                    base_table: Box::new(base),
                    metatable: Box::new(metatable),
                }))
            }
            _ => panic!("Unknown lua global: {}", name),
        }
    }

    pub(crate) fn visit_type_table_inner(
        &mut self,
        node: &'a ast::TypeTable,
        is_metatable: bool,
    ) -> Type {
        let mut entries = Dictionary::new();
        let mut metatable = None;
        let mut array_member_count = 0;
        for field in node.fields().iter() {
            match field {
                ast::TypeTableField::Computed { span, key, value } => {
                    let key = self.visit_type_info(key);
                    let value = self.visit_type_info(value);
                    entries.insert(types::TableFieldKey::Computed(key), value);
                }
                ast::TypeTableField::Named { span, name, value } => {
                    let real_name = name.ty().as_name();
                    let value = self.visit_type_info(value);
                    if &real_name == "@metatable" {
                        if metatable.is_none() {
                            let table = types::Table {
                                span: value.span(),
                                entries: Dictionary::new(),
                                metatable: None,
                                is_metatable,
                            };
                            metatable = Some(Box::new(table));
                        } else {
                            self.diagnostics
                                .push(Diagnostic::DuplicatedMetatable { span: *span });
                        }
                    } else {
                        entries.insert(types::TableFieldKey::Name(real_name, *span), value);
                    }
                }
                ast::TypeTableField::Array(value) => {
                    array_member_count += 1;
                    let value = self.visit_type_info(value);
                    entries.insert(types::TableFieldKey::None(array_member_count), value);
                }
            }
        }
        types::Type::Table(types::Table {
            span: node.span(),
            entries,
            is_metatable,
            metatable,
        })
    }

    pub(crate) fn combine_return_types(&mut self, typ: Type) {
        // We need to find a scope that is a:
        // - Module scope
        // - Function scope
        // - Has no expected type which it will be processed to the analyzer
        let mut parent = Some(self.current_scope_id());
        while let Some(real_parent) = parent {
            let scope = self.scopes.get_mut(real_parent).unwrap();
            parent = scope.parent;

            // check if it is a returnable scope and combine return types
            if scope.is_returnable() && scope.expected_type.is_none() {
                scope.actual_type = if let Some(actual_type) = &scope.actual_type {
                    if actual_type == &typ {
                        break;
                    }
                    // TODO(memothelemo): Add union and intersection types
                    todo!("Union and intersection are on the way!")
                } else {
                    Some(typ)
                };
                break;
            }
        }
    }

    pub(crate) fn visit_function_body(
        &mut self,
        body: &'a ast::FunctionBody,
        allocated_id: Id<&'a dyn ast::Node>,
        span: Span,
    ) -> hir::Function<'a> {
        let mut parameters = Vec::new();
        let mut defaults = Vec::new();

        self.push_scope(ScopeKind::Function);

        let expected_type = body
            .return_type()
            .as_ref()
            .map(|return_type| self.visit_type_info(return_type));

        let mut scope = self.current_scope_mut();
        scope.expected_type = expected_type.clone();

        for param in body.params().iter() {
            let name = param.name.ty().as_name();
            let typ = param
                .explicit_type
                .as_ref()
                .map(|v| self.visit_type_info(v))
                .unwrap_or(types::makers::any(param.span));

            defaults.push(param.default.as_ref().map(|v| self.visit_expr(v)));

            self.insert_variable(
                true,
                &name,
                SymbolFlags::FunctionParameter,
                Some(param.span),
                typ.clone(),
            );

            parameters.push(types::FunctionParameter {
                optional: param.optional,
                span: param.span,
                name: Some(name),
                typ,
            });
        }

        let mut varidiac_param = None;

        // TODO(memothelemo): Add support for varidiac parameters
        if let Some(varidiac) = body.varidiac() {
            varidiac_param = Some(types::VaridiacParameter {
                span: varidiac.span,
                typ: Box::new(
                    varidiac
                        .typ
                        .as_ref()
                        .map(|v| self.visit_type_info(v))
                        .unwrap_or(types::makers::any(varidiac.span)),
                ),
            });
        }

        let block = self.visit_block(body.block());
        let expr = hir::Function {
            span,
            defaults,
            typ: Type::Function(types::FunctionType {
                span,
                parameters,
                varidiac_param,
                return_type: Box::new(expected_type.unwrap_or(block.actual_type.clone())),
            }),
            block,
            node_id: allocated_id,
        };

        self.pop_scope();

        expr
    }

    pub(crate) fn visit_file(&mut self, file: &'a ast::File) -> hir::Block<'a> {
        self.visit_block(file.block())
    }

    pub(crate) fn revisit_function_type(
        &mut self,
        mut base: types::FunctionType,
        assertion: types::FunctionType,
    ) -> (types::Type, types::Type) {
        for (idx, base_param) in base.parameters.iter_mut().enumerate() {
            if let Some(assertion_param) = assertion.parameters.get(idx) {
                if matches!(base_param.typ.get_lit_type(), Some(types::LiteralKind::Any)) {
                    // overiding the parameter guess enough?
                    base_param.typ = assertion_param.typ.clone();
                    *base_param.typ.span_mut() = base_param.span;
                }
            }
        }
        (Type::Function(base), Type::Function(assertion))
    }
}

use salite_ast::{AstVisitor, ExprVisitor, LastStmtVisitor, StmtVisitor, TypeVisitor};

/// Attempts to unwrap the symbol's type and warns
/// depending on the compiler flag.
#[inline]
pub fn unwrap_symbol_type(symbol: &Symbol) -> &Type {
    #[cfg(debug_assertions)]
    {
        if symbol.typ.is_none() {
            panic!("Expected there's type in a symbol: {:#?}", symbol);
        }
        symbol.typ.as_ref().unwrap()
    }
    #[cfg(not(debug_assertions))]
    {
        symbol.typ.as_ref().expect("Invalid symbol")
    }
}

impl<'a> TypeVisitor<'a> for Binder<'a> {
    type Output = Type;

    fn visit_type_callback(&mut self, node: &'a ast::TypeCallback) -> Self::Output {
        let mut parameters = Vec::new();
        for type_param in node.parameters().iter() {
            match type_param.name().ty() {
                ast::TokenType::Identifier(str) => {
                    let typ = self.visit_type_info(type_param.type_info());
                    parameters.push(types::FunctionParameter {
                        optional: false,
                        span: type_param.span(),
                        name: Some(str.to_string()),
                        typ,
                    });
                }
                _ => unreachable!(),
            }
        }
        Type::Function(types::FunctionType {
            span: node.span(),
            parameters,
            return_type: Box::new(self.visit_type_info(node.return_type().borrow())),
            // TODO(memothelemo): Add varidiac type parameter?
            varidiac_param: None,
        })
    }

    fn visit_type_reference(&mut self, node: &'a ast::TypeReference) -> Self::Output {
        let scope = self.current_scope();
        let name = node.name().ty().as_name();

        let symbol = scope.symbol_from_type_alias(self, &name);
        if let Some(symbol_id) = symbol {
            let symbol = self.symbols.get(symbol_id).unwrap();
            let arguments = node.arguments().as_ref().map(|arguments| {
                let mut list = Vec::new();
                for arg in arguments.iter() {
                    list.push(self.visit_type_info(arg));
                }
                list
            });
            Type::Ref(types::RefType {
                span: node.span(),
                name,
                symbol: symbol_id,
                arguments,
            })
        } else {
            self.diagnostics.push(Diagnostic::UnknownTypeAlias {
                name,
                span: node.span(),
            });
            types::makers::any(node.span())
        }
    }

    fn visit_type_table(&mut self, node: &'a ast::TypeTable) -> Self::Output {
        self.visit_type_table_inner(node, false)
    }

    fn visit_type_metatable(&mut self, node: &'a ast::TypeMetatable) -> Self::Output {
        self.visit_type_table_inner(node.table(), true)
    }

    fn visit_type_tuple(&mut self, node: &'a ast::TypeTuple) -> Self::Output {
        let mut members = Vec::new();
        for member in node.members().iter() {
            members.push(self.visit_type_info(member));
        }
        Type::Tuple(types::TupleType {
            span: node.span(),
            members,
        })
    }
}

impl<'a> ExprVisitor<'a> for Binder<'a> {
    type Output = hir::Expr<'a>;

    fn visit_bool_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_function_expr(&mut self, node: &'a ast::FunctionExpr) -> Self::Output {
        let id = self.nodes.alloc(node);
        let function = self.visit_function_body(node.body(), id, node.span());
        hir::Expr::Function(function)
    }

    fn visit_name_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_number_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_nil_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_str_expr(&mut self, node: &'a ast::Token) -> Self::Output {
        unimplemented!()
    }

    fn visit_table_ctor_expr(&mut self, node: &'a ast::TableCtor) -> Self::Output {
        let mut fields = Vec::new();
        let mut entries = Dictionary::new();
        let mut array_member_count = 0;
        for field in node.fields().iter() {
            let field = match field {
                ast::TableField::Array(expr) => {
                    array_member_count += 1;
                    let expr = self.visit_expr(expr);
                    entries.insert(
                        types::TableFieldKey::None(array_member_count),
                        expr.typ().clone(),
                    );
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
            is_metatable: false,
            metatable: None,
        });
        hir::Expr::Table(hir::Table {
            span: node.span(),
            node_id: self.nodes.alloc(node),
            fields,
            typ,
        })
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
        match node.suffix() {
            ast::SuffixKind::Call(args) => {
                // check for lua globals?
                if let ast::Expr::Literal(ast::Literal::Name(tok)) = node.base().borrow() {
                    let name = tok.ty().as_name();
                    for (id, _) in self.var_globals.iter() {
                        if id == &name {
                            return self.eval_lua_call_globals(&name, node, args);
                        }
                    }
                }
                self.visit_call_expr_inner(node, args)
            }
            ast::SuffixKind::Computed(_) => todo!(),
            ast::SuffixKind::Method(_) => todo!(),
            ast::SuffixKind::Name(_) => todo!(),
        }
    }

    fn visit_type_assertion_expr(&mut self, node: &'a ast::TypeAssertion) -> Self::Output {
        let typ = self.visit_type_info(node.cast());
        hir::Expr::TypeAssertion(hir::TypeAssertion {
            base: Box::new(self.visit_expr(node.base().borrow())),
            cast: typ,
            span: node.span(),
            node_id: self.nodes.alloc(node),
        })
    }

    fn visit_unary_expr(&mut self, node: &'a ast::Unary) -> Self::Output {
        todo!()
    }

    fn visit_suffix_kind_expr(&mut self, node: &'a ast::SuffixKind) -> Self::Output {
        todo!()
    }

    fn visit_literal_expr(&mut self, base: &'a ast::Literal) -> Self::Output {
        match base {
            ast::Literal::Bool(node) => {
                literal_macro!(self, types::makers::bool(node.span()), node, base, None)
            }
            ast::Literal::Function(node) => self.visit_function_expr(node),
            ast::Literal::Name(node) => {
                let scope = self.current_scope();
                let real_name = node.ty().as_name();

                let symbol = scope.symbol_from_variable(self, &real_name);
                if let Some(symbol_id) = symbol {
                    let symbol = self.symbols.get(symbol_id).unwrap();
                    hir::Expr::Literal(hir::Literal {
                        // meh?
                        typ: unwrap_symbol_type(symbol).clone(),
                        span: node.span(),
                        symbol: Some(symbol_id),
                        node_id: self.nodes.alloc(base),
                    })
                } else {
                    self.diagnostics.push(Diagnostic::UnknownVariable {
                        name: real_name,
                        span: node.span(),
                    });
                    hir::Expr::Literal(hir::Literal {
                        typ: types::makers::any(node.span()),
                        span: node.span(),
                        symbol: Some(self.register_symbol(
                            vec![node.span()],
                            false,
                            SymbolFlags::UnknownVariable,
                            Some(types::makers::any(node.span())),
                            None,
                        )),
                        node_id: self.nodes.alloc(base),
                    })
                }
            }
            ast::Literal::Number(node) => {
                literal_macro!(self, types::makers::number(node.span()), node, base, None)
            }
            ast::Literal::Nil(node) => {
                literal_macro!(self, types::makers::nil(node.span()), node, base, None)
            }
            ast::Literal::Str(node) => {
                literal_macro!(self, types::makers::string(node.span()), node, base, None)
            }
            ast::Literal::Table(node) => self.visit_table_ctor_expr(node),
            ast::Literal::Varargs(node) => self.visit_varargs_expr(node),
        }
    }
}

impl<'a> StmtVisitor<'a> for Binder<'a> {
    type Output = hir::Stmt<'a>;

    fn visit_call_stmt(&mut self, node: &'a ast::Expr) -> Self::Output {
        match self.visit_expr(node) {
            hir::Expr::Call(n) => hir::Stmt::Call(n),
            hir::Expr::Library(n) => hir::Stmt::Library(n),
            _ => unreachable!(),
        }
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
            let (expr_source, mut expr) = exprs
                .get(id)
                .cloned()
                .map(|v| (Some(v.0), Some(v.1)))
                .unwrap_or((None, None));

            let symbol_id = self.insert_variable(
                name.type_info().is_some(),
                &real_name,
                SymbolFlags::BlockVariable,
                Some(name.span()),
                expr.clone().unwrap_or(types::makers::any(name.span())),
            );

            let explicit_type = name.type_info().as_ref().map(|v| self.visit_type_info(v));
            let (explicit_type, expr) = match (explicit_type, expr) {
                (Some(Type::Function(mut assertion)), Some(Type::Function(mut expr))) => {
                    let (v0, v1) = self.revisit_function_type(expr, assertion);
                    (Some(v0), Some(v1))
                }
                a => (a.0, a.1),
            };

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

    fn visit_local_function_stmt(&mut self, node: &'a ast::LocalFunction) -> Self::Output {
        let name = node.name().ty().as_name();
        let node_id = self.nodes.alloc(node);
        let body = self.visit_function_body(node.body(), node_id, node.span());

        let symbol_id = self.insert_variable(
            false,
            &name,
            SymbolFlags::BlockVariable,
            Some(node.span()),
            body.typ.clone(),
        );

        let stmt = hir::Stmt::LocalAssign(hir::LocalAssign {
            variables: vec![hir::LocalAssignVar {
                name,
                name_span: node.name().span(),
                name_symbol: symbol_id,
                explicit_type: None,
                expr_id: 0,
                expr_source: Some(body.span),
                expr: Some(body.typ.clone()),
            }],
            span: node.span(),
            node_id: body.node_id,
            exprs: vec![hir::Expr::Function(body)],
        });

        stmt
    }

    fn visit_numeric_for_stmt(&mut self, node: &'a ast::NumericFor) -> Self::Output {
        todo!()
    }

    fn visit_repeat_stmt(&mut self, node: &'a ast::RepeatStmt) -> Self::Output {
        todo!()
    }

    fn visit_while_stmt(&mut self, node: &'a ast::WhileStmt) -> Self::Output {
        todo!()
    }

    fn visit_var_assign_stmt(&mut self, node: &'a ast::VarAssign) -> Self::Output {
        todo!()
    }

    fn visit_type_declaration_stmt(&mut self, node: &'a ast::TypeDeclaration) -> Self::Output {
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
        let symbol_id = self.insert_type_alias(
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
                self.insert_type_alias(
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

impl<'a> AstVisitor<'a> for Binder<'a> {
    type BlockOutput = hir::Block<'a>;

    fn visit_block(&mut self, node: &'a ast::Block) -> Self::BlockOutput {
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

                    // create assumable return type
                    let return_type = if exprs.is_empty() {
                        types::makers::void(node.span())
                    } else if exprs.len() == 1 {
                        exprs.get(0).unwrap().typ().clone()
                    } else {
                        let mut members = Vec::new();
                        for expr in exprs.iter() {
                            members.push(expr.typ().clone());
                        }
                        Type::Tuple(types::TupleType {
                            span: node.span(),
                            members,
                        })
                    };
                    self.combine_return_types(return_type.clone());

                    hir::LastStmt::Return(hir::Return {
                        concluding_typ: return_type,
                        exprs,
                        span: node.span(),
                        node_id: self.nodes.alloc(node),
                    })
                }
                c @ ast::Stmt::Break(n) => hir::LastStmt::Break(n.span(), self.nodes.alloc(c)),
                _ => unreachable!(),
            })
            .unwrap_or(hir::LastStmt::None);

        let actual_type = self
            .current_scope()
            .actual_type
            .clone()
            .unwrap_or(types::makers::void(node.span()));

        let expected_type = self.current_scope().expected_type.clone();

        hir::Block {
            actual_type,
            expected_type,
            stmts,
            last_stmt,
            span: node.span(),
        }
    }
}
