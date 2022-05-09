use super::*;

pub trait ExprVisitor<'a> {
    type Output: 'a;

    fn visit_bool_expr(&mut self, node: &'a Token) -> Self::Output;
    fn visit_function_expr(&mut self, node: &'a FunctionExpr) -> Self::Output;
    fn visit_name_expr(&mut self, node: &'a Token) -> Self::Output;
    fn visit_number_expr(&mut self, node: &'a Token) -> Self::Output;
    fn visit_nil_expr(&mut self, node: &'a Token) -> Self::Output;
    fn visit_str_expr(&mut self, node: &'a Token) -> Self::Output;
    fn visit_table_ctor_expr(&mut self, node: &'a TableCtor) -> Self::Output;
    fn visit_varargs_expr(&mut self, node: &'a Token) -> Self::Output;

    fn visit_binary_expr(&mut self, node: &'a Binary) -> Self::Output;
    fn visit_paren_expr(&mut self, node: &'a Expr) -> Self::Output;
    fn visit_suffixed_expr(&mut self, node: &'a Suffixed) -> Self::Output;
    fn visit_type_assertion_expr(&mut self, node: &'a TypeAssertion) -> Self::Output;
    fn visit_unary_expr(&mut self, node: &'a Unary) -> Self::Output;

    fn visit_suffix_kind_expr(&mut self, node: &'a SuffixKind) -> Self::Output;

    fn visit_literal_expr(&mut self, node: &'a Literal) -> Self::Output {
        match node {
            Literal::Bool(node) => self.visit_bool_expr(node),
            Literal::Function(node) => self.visit_function_expr(node),
            Literal::Name(node) => self.visit_name_expr(node),
            Literal::Number(node) => self.visit_number_expr(node),
            Literal::Nil(node) => self.visit_nil_expr(node),
            Literal::Str(node) => self.visit_str_expr(node),
            Literal::Table(node) => self.visit_table_ctor_expr(node),
            Literal::Varargs(node) => self.visit_varargs_expr(node),
        }
    }

    fn visit_expr(&mut self, node: &'a Expr) -> Self::Output {
        match node {
            Expr::Binary(node) => self.visit_binary_expr(node),
            Expr::Literal(node) => self.visit_literal_expr(node),
            Expr::Paren(node) => self.visit_paren_expr(node),
            Expr::Suffixed(node) => self.visit_suffixed_expr(node),
            Expr::TypeAssertion(node) => self.visit_type_assertion_expr(node),
            Expr::Unary(node) => self.visit_unary_expr(node),
        }
    }
}

pub trait TypeVisitor<'a> {
    type Output: 'a;

    fn visit_type_callback(&mut self, node: &'a TypeCallback) -> Self::Output;
    fn visit_type_reference(&mut self, node: &'a TypeReference) -> Self::Output;
    fn visit_type_table(&mut self, node: &'a TypeTable) -> Self::Output;
    fn visit_type_metatable(&mut self, node: &'a TypeMetatable) -> Self::Output;
    fn visit_type_tuple(&mut self, node: &'a TypeTuple) -> Self::Output;
    fn visit_type_intersection(&mut self, node: &'a TypeIntersection) -> Self::Output;
    fn visit_type_union(&mut self, node: &'a TypeUnion) -> Self::Output;

    fn visit_type_info(&mut self, node: &'a TypeInfo) -> Self::Output {
        match node {
            TypeInfo::Callback(node) => self.visit_type_callback(node),
            TypeInfo::Reference(node) => self.visit_type_reference(node),
            TypeInfo::Table(node) => self.visit_type_table(node),
            TypeInfo::Metatable(node) => self.visit_type_metatable(node),
            TypeInfo::Tuple(node) => self.visit_type_tuple(node),
            TypeInfo::Intersection(node) => self.visit_type_intersection(node),
            TypeInfo::Union(node) => self.visit_type_union(node),
        }
    }
}

pub trait StmtVisitor<'a> {
    type Output: 'a;

    fn visit_call_stmt(&mut self, node: &'a Expr) -> Self::Output;
    fn visit_do_stmt(&mut self, node: &'a DoStmt) -> Self::Output;
    fn visit_function_assign_stmt(&mut self, node: &'a FunctionAssign) -> Self::Output;
    fn visit_generic_for_stmt(&mut self, node: &'a GenericFor) -> Self::Output;
    fn visit_if_stmt(&mut self, node: &'a IfStmt) -> Self::Output;
    fn visit_local_assign_stmt(&mut self, node: &'a LocalAssign) -> Self::Output;
    fn visit_local_function_stmt(&mut self, node: &'a LocalFunction) -> Self::Output;
    fn visit_numeric_for_stmt(&mut self, node: &'a NumericFor) -> Self::Output;
    fn visit_repeat_stmt(&mut self, node: &'a RepeatStmt) -> Self::Output;
    fn visit_while_stmt(&mut self, node: &'a WhileStmt) -> Self::Output;
    fn visit_var_assign_stmt(&mut self, node: &'a VarAssign) -> Self::Output;
    fn visit_type_declaration_stmt(&mut self, node: &'a TypeDeclaration) -> Self::Output;

    fn visit_stmt(&mut self, node: &'a Stmt) -> Self::Output {
        match node {
            Stmt::Call(node) => self.visit_call_stmt(node),
            Stmt::Do(node) => self.visit_do_stmt(node),
            Stmt::FunctionAssign(node) => self.visit_function_assign_stmt(node),
            Stmt::GenericFor(node) => self.visit_generic_for_stmt(node),
            Stmt::If(node) => self.visit_if_stmt(node),
            Stmt::LocalAssign(node) => self.visit_local_assign_stmt(node),
            Stmt::LocalFunction(node) => self.visit_local_function_stmt(node),
            Stmt::NumericFor(node) => self.visit_numeric_for_stmt(node),
            Stmt::Repeat(node) => self.visit_repeat_stmt(node),
            Stmt::While(node) => self.visit_while_stmt(node),
            Stmt::VarAssign(node) => self.visit_var_assign_stmt(node),
            Stmt::TypeDeclaration(node) => self.visit_type_declaration_stmt(node),
            _ => unreachable!(),
        }
    }
}

pub trait LastStmtVisitor<'a> {
    type Output: 'a;

    fn visit_break_stmt(&mut self, node: &'a Token) -> Self::Output;
    fn visit_return_stmt(&mut self, node: &'a ReturnStmt) -> Self::Output;

    fn visit_last_stmt(&mut self, node: &'a Stmt) -> Self::Output {
        match node {
            Stmt::Break(node) => self.visit_break_stmt(node),
            Stmt::Return(node) => self.visit_return_stmt(node),
            _ => unreachable!(),
        }
    }
}

pub trait AstVisitor<'a> {
    type BlockOutput: 'a;

    fn visit_block(&mut self, node: &'a Block) -> Self::BlockOutput;
}
