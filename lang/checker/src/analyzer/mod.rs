#![allow(unused)]
use std::collections::HashMap;

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

pub trait Validate<'a> {
    type Output;

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError>;
}

impl<'a> Validate<'a> for hir::Block<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for stmt in self.stmts.iter() {
            stmt.validate(analyzer)?;
        }
        self.last_stmt.validate(analyzer)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Analyzer<'a> {
    binder: &'a Binder<'a>,
    config: &'a lunar_common::Config,

    /// contemporary storage for type parameters
    type_vars: HashMap<String, Type>,
}

impl<'a> Analyzer<'a> {
    pub fn analyze(
        binder: &'a Binder,
        config: &'a lunar_common::Config,
        file: &'a hir::File,
    ) -> Result<(), AnalyzeError> {
        let mut analyzer = Analyzer {
            binder,
            config,
            type_vars: HashMap::default(),
        };
        file.block.validate(&mut analyzer)?;
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
            Type::Table(_) => todo!(),
            //Type::Alias(node) => node.name.to_string(),
        }
    }

    pub fn skip_downwards(&self, mut typ: Type) -> Type {
        #[allow(clippy::or_fun_call)]
        while let Type::Ref(ref node) = typ {
            // unless it has type arguments such we need to evaluate further
            if node.arguments.is_some() {
                break;
            }

            // even further evaluation, it's slow but it's worth it actually
            let symbol = self.binder.symbols.get(node.symbol).unwrap();
            if symbol.parameters.is_some() {
                break;
            }

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

    pub fn solve_type_recursive(&mut self, typ: &ctypes::Type) -> Result<Type, AnalyzeError> {
        match typ {
            // TODO(memothelemo): Do something with other literal types
            Type::Literal(node) => Ok(typ.clone()),
            Type::Ref(_) => self.solve_type_ref(typ),
            Type::Tuple(node) => {
                let mut solved_members = Vec::new();
                for member in node.members.iter() {
                    solved_members.push(self.solve_type_recursive(member)?);
                }
                Ok(ctypes::Type::Tuple(ctypes::TupleType {
                    span: node.span,
                    members: solved_members,
                }))
            }
            Type::Table(_) => todo!(),
        }
    }

    pub fn solve_type_ref(&mut self, typ: &ctypes::Type) -> Result<Type, AnalyzeError> {
        let sample = match typ {
            ctypes::Type::Ref(refer) => refer,
            _ => panic!("Expected type reference"),
        };

        // straight forward thing to do
        if let Some(typ) = self.type_vars.get(&sample.name) {
            return Ok(typ.clone());
        }

        // get the type parameters
        let symbol = self.binder.symbols.get(sample.symbol).unwrap();

        #[allow(clippy::or_fun_call)]
        if symbol.parameters.is_none() {
            return Ok(symbol
                .typ
                .clone()
                .unwrap_or(ctypes::makers::any(typ.span())));
        }

        let parameters = symbol.parameters.as_ref().unwrap();
        if sample.arguments.is_none() && !parameters.is_empty() {
            return Err(AnalyzeError::NoArguments {
                span: typ.span(),
                base: sample.name.to_string(),
            });
        }

        let arguments = sample.arguments.as_ref().unwrap();

        #[allow(clippy::or_fun_call)]
        for (idx, param) in parameters.iter().enumerate() {
            let arg = arguments.get(idx).or(param.default.as_ref());
            let explicit_param = param
                .explicit
                .clone()
                .unwrap_or(ctypes::makers::any(param.span));

            if let Some(arg) = arg {
                self.resolve_type(arg, &explicit_param, arg.span())?;
                self.type_vars.insert(param.name.clone(), arg.clone());
            } else {
                return Err(AnalyzeError::MissingArgument {
                    span: typ.span(),
                    idx,
                    base: sample.name.to_string(),
                    expected_type: self.type_description(&explicit_param),
                });
            }
        }

        self.solve_type_recursive(symbol.typ.as_ref().unwrap())
    }

    pub fn solve_type(&mut self, value: &Type) -> Result<Type, AnalyzeError> {
        let result = self.solve_type_recursive(value)?;
        self.type_vars.clear();
        Ok(result)
    }

    pub fn resolve_type(
        &mut self,
        value: &Type,
        assertion: &Type,
        span: Span,
    ) -> Result<(), AnalyzeError> {
        let value_des = self.type_description(value);
        let assert_des = self.type_description(assertion);
        let value = self.skip_downwards(value.clone());
        let assertion = self.skip_downwards(assertion.clone());
        use ctypes::LiteralKind;
        match (&value, &assertion) {
            (_, Type::Literal(l)) if matches!(l.kind, LiteralKind::Any | LiteralKind::Unknown) => {
                Ok(())
            }
            (Type::Literal(l), _) if matches!(l.kind, LiteralKind::Any) => Ok(()),
            (Type::Literal(a), Type::Literal(b))
                if matches!(
                    (&a.kind, &b.kind),
                    (
                        LiteralKind::Nil | LiteralKind::Void,
                        LiteralKind::Void | LiteralKind::Nil
                    )
                ) =>
            {
                Ok(())
            }

            (_, Type::Ref(_)) => {
                let real_type = self.solve_type_ref(&assertion)?;
                self.resolve_type(&value, &real_type, span)
            }
            (Type::Ref(_), _) => {
                let real_type = self.solve_type_ref(&value)?;
                self.resolve_type(&real_type, &assertion, span)
            }

            (_, Type::Tuple(tupl)) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.resolve_type(&value, member_type, span)
            }
            (Type::Tuple(tupl), _) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.resolve_type(member_type, &assertion, span)
            }

            _ if value == assertion => Ok(()),
            _ => Err(AnalyzeError::NotExtendable {
                value: value_des,
                assertion: assert_des,
                span,
            }),
        }
    }
}
