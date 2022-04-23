use crate::to_expr_typ;

use super::*;
use id_arena::Id;
use lunar_macros::PropertyGetter;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PropertyGetter)]
pub struct TypCallback {
    pub(crate) return_typ: Box<Typ>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Typ {
    Any,
    Bool,
    Callback(TypCallback),
    Error,
    Number,
    Nil,
    String,
    Tuple(Vec<Typ>),
    Unknown,
    Variable(Id<Symbol>),
    Void,
}

pub(crate) fn from_vec_exprs(exprs: Vec<bind_ast::Expr>, symbols: &SymbolStorage) -> Typ {
    // type optimization
    if exprs.is_empty() {
        Typ::Nil
    } else if exprs.len() == 1 {
        to_expr_typ(exprs.get(0).unwrap(), symbols)
    } else {
        let mut components = Vec::new();
        for expr in exprs {
            components.push(to_expr_typ(&expr, symbols));
        }
        Typ::Tuple(components)
    }
}

pub(crate) fn from_vec_ref_exprs(exprs: &Vec<bind_ast::Expr>, symbols: &SymbolStorage) -> Typ {
    // type optimization
    if exprs.is_empty() {
        Typ::Nil
    } else if exprs.len() == 1 {
        to_expr_typ(exprs.get(0).unwrap(), symbols)
    } else {
        let mut components = Vec::new();
        for expr in exprs {
            components.push(to_expr_typ(expr, symbols));
        }
        Typ::Tuple(components)
    }
}
