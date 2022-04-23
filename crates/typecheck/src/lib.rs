mod binder;
mod builtin;
mod checker;
mod diagnostic;
mod storage;

use std::borrow::Borrow;

pub use binder::*;
pub use builtin::*;
pub use checker::*;
pub use diagnostic::*;
pub use storage::*;

#[inline]
fn skip_downwards(typ: Typ, symbols: &SymbolStorage) -> Typ {
    // instead of recursion, we use while loops
    let mut result = typ;
    while let Typ::Variable(id) = result {
        result = match symbols.get_symbol(id).unwrap().typ {
            SymbolTyp::Variable(v) => v,
            SymbolTyp::Type(v) => v,
        };
    }
    result
}

#[inline]
fn to_expr_typ(expr: &bind_ast::Expr, symbols: &SymbolStorage) -> Typ {
    use bind_ast::ExprKind;
    match &expr.kind {
        ExprKind::Assertion(_, ty) => ty.clone(),
        ExprKind::Error => {
            unimplemented!("'Error' expression kind is not allowed, evaluate first.")
        }
        ExprKind::Literal(ty) => ty.clone(),
        ExprKind::Name(ty) => ty.clone(),
        ExprKind::Callback(_, ty) => Typ::Callback(TypCallback {
            return_typ: Box::new(ty.clone()),
        }),
        ExprKind::Call(expr, _) => {
            let real_type = skip_downwards(to_expr_typ(expr, symbols), symbols);
            match real_type {
                Typ::Callback(call) => {
                    let typ: &Typ = call.return_typ.borrow();
                    typ.clone()
                },
                _ => Typ::Error,
            }
        },
    }
}
