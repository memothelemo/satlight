use ast::SpannedNode;

use super::*;

mod local_assign;

impl<'a, 'b> Transform<'a, 'b> for ast::Stmt {
    type Output = hir::Stmt<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self {
            ast::Stmt::Call(_) => todo!(),
            ast::Stmt::Do(_) => todo!(),
            ast::Stmt::FunctionAssign(_) => todo!(),
            ast::Stmt::GenericFor(_) => todo!(),
            ast::Stmt::If(_) => todo!(),
            ast::Stmt::LocalAssign(node) => node.transform(tfmr),
            ast::Stmt::LocalFunction(_) => todo!(),
            ast::Stmt::NumericFor(_) => todo!(),
            ast::Stmt::Repeat(_) => todo!(),
            ast::Stmt::While(_) => todo!(),
            ast::Stmt::TypeDeclaration(_) => todo!(),
            ast::Stmt::VarAssign(_) => todo!(),
            _ => unreachable!(),
        }
    }
}

impl<'a, 'b> Transform<'a, 'b> for ast::Block {
    type Output = hir::Block<'b>;

    fn transform(&'b self, trmf: &mut Transformer<'a, 'b>) -> Self::Output {
        let mut stmts = Vec::new();
        for stmt in self.stmts().iter() {
            stmts.push(stmt.transform(trmf));
        }
        let last_stmt = if let Some(..) = self.last_stmt() {
            todo!()
        } else {
            hir::LastStmt::None
        };
        hir::Block {
            span: self.span(),
            stmts,
            last_stmt,
            actual_type: types::makers::any(self.span()),
            expected_type: None,
        }
    }
}
