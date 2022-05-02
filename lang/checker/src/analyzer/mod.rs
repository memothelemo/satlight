#![allow(unused)]
use crate::binder::Binder;
use crate::types::Type;
use crate::{hir, types as ctypes};

use super::*;

mod errors;
mod exprs;
mod stmts;
mod types;

pub use errors::*;
pub use exprs::*;
use lunar_ast::Span;
pub use stmts::*;
pub use types::*;

pub trait Validate {
    type Output;

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError>;
}

impl Validate for hir::Block {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for stmt in self.stmts.iter() {
            stmt.validate(analyzer)?;
        }
        self.last_stmt.validate(analyzer)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Analyzer<'a> {
    binder: &'a Binder,
    config: &'a lunar_common::Config,
}

impl<'a> Analyzer<'a> {
    pub fn analyze(
        binder: &'a Binder,
        config: &'a lunar_common::Config,
        block: &hir::Block,
    ) -> Result<(), AnalyzeError> {
        let mut analyzer = Analyzer { binder, config };
        block.validate(&mut analyzer)?;
        Ok(())
    }
}

impl<'a> Analyzer<'a> {
    pub fn type_description(&self, typ: &Type) -> String {
        match typ {
            Type::Ref(info) => info.name.to_string(),
            Type::Tuple(info) => {
                let mut result = Vec::new();
                for typ in info.members.iter() {
                    result.push(self.type_description(typ));
                }
                format!("({})", result.join(","))
            }
            Type::Literal(info) => match info.kind {
                ctypes::LiteralKind::Any => "any",
                ctypes::LiteralKind::Bool => "bool",
                ctypes::LiteralKind::Number => "number",
                ctypes::LiteralKind::Nil => "nil",
                ctypes::LiteralKind::String => "string",
                ctypes::LiteralKind::Unknown => "unknown",
                ctypes::LiteralKind::Void => "void",
            }
            .to_string(),
        }
    }

    pub fn skip_downwards(&self, mut typ: Type) -> Type {
        #[allow(clippy::or_fun_call)]
        while let Type::Ref(ref node) = typ {
            typ = self
                .binder
                .symbols
                .get(node.symbol)
                .unwrap()
                .typ
                .clone()
                .unwrap_or(ctypes::makers::any(typ.span()));
        }
        typ
    }

    pub fn resolve_type(&self, left: &Type, right: &Type, span: Span) -> Result<(), AnalyzeError> {
        let left = self.skip_downwards(left.clone());
        let right = self.skip_downwards(right.clone());
        match (&left, &right) {
            (_, Type::Tuple(tupl)) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.resolve_type(&left, member_type, span)
            }
            (Type::Tuple(tupl), _) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.resolve_type(member_type, &right, span)
            }
            _ if left == right => Ok(()),
            _ => Err(AnalyzeError::NotExtendable {
                left: self.type_description(&left),
                right: self.type_description(&right),
                span,
            }),
        }
    }
}
