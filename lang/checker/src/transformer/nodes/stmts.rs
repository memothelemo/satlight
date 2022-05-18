mod local_assign;
mod type_declare;

use super::*;
use crate::types::Type;
use ast::SpannedNode;

pub use local_assign::*;
pub use type_declare::*;

impl<'a, 'b> Transform<'a, 'b> for ast::Stmt {
    type Output = hir::Stmt<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        match self {
            ast::Stmt::Call(node) => match node.transform(tfmr) {
                hir::Expr::Suffixed(node) => hir::Stmt::Call(node),
                hir::Expr::Library(node) => hir::Stmt::Library(node),
                _ => unreachable!(),
            },
            ast::Stmt::Do(_) => todo!(),
            ast::Stmt::FunctionAssign(_) => todo!(),
            ast::Stmt::GenericFor(_) => todo!(),
            ast::Stmt::If(_) => todo!(),
            ast::Stmt::LocalAssign(node) => node.transform(tfmr),
            ast::Stmt::LocalFunction(node) => node.transform(tfmr),
            ast::Stmt::NumericFor(_) => todo!(),
            ast::Stmt::Repeat(_) => todo!(),
            ast::Stmt::While(_) => todo!(),
            ast::Stmt::TypeDeclaration(node) => node.transform(tfmr),
            ast::Stmt::VarAssign(_) => todo!(),
            _ => unreachable!(),
        }
    }
}

fn visit_last_stmt<'a, 'b>(
    node: &'b ast::Stmt,
    tfmr: &mut Transformer<'a, 'b>,
) -> hir::LastStmt<'b> {
    match node {
        ast::Stmt::Return(node) => {
            let exprs = node
                .exprlist()
                .iter()
                .map(|v| v.transform(tfmr))
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
                Type::Tuple(variants::Tuple {
                    span: node.span(),
                    members,
                })
            };
            tfmr.combine_return_types(return_type.clone());

            hir::LastStmt::Return(hir::Return {
                concluding_typ: return_type,
                exprs,
                span: node.span(),
                node_id: tfmr.ctx.nodes.alloc(node),
            })
        }
        ast::Stmt::Break(n) => hir::LastStmt::Break(n.span(), tfmr.ctx.nodes.alloc(node)),
        _ => unreachable!(),
    }
}

impl<'a, 'b> Transform<'a, 'b> for ast::Block {
    type Output = hir::Block<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let mut stmts = Vec::new();
        for stmt in self.stmts().iter() {
            stmts.push(stmt.transform(tfmr));
        }
        let last_stmt = if let Some(stmt) = self.last_stmt() {
            visit_last_stmt(stmt, tfmr)
        } else {
            hir::LastStmt::None
        };
        hir::Block {
            span: self.span(),
            stmts,
            last_stmt,
            actual_type: tfmr
                .current_scope()
                .actual_type
                .clone()
                .unwrap_or(types::makers::void(self.span())),
            expected_type: None,
        }
    }
}
