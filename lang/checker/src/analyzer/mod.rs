#![allow(unused)]
mod errors;
mod expressions;
mod normalizer;
mod statements;
mod typess;

use expressions::*;
use salite_common::dictionary::Dictionary;
use statements::*;
use typess::*;

use super::*;
use crate::binder::Binder;
use crate::types::Type;
use crate::{hir, types};
pub use errors::*;
use salite_ast::Span;
use std::borrow::Borrow;
use std::collections::HashMap;

pub trait Validate<'a> {
    type Output;

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError>;
}

impl<'a> Validate<'a> for hir::Block<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        let last_expected_type = analyzer.expected_type.clone();
        analyzer.expected_type = self.expected_type.clone();
        for stmt in self.stmts.iter() {
            stmt.validate(analyzer)?;
        }
        self.last_stmt.validate(analyzer)?;
        analyzer.expected_type = last_expected_type;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Analyzer<'a> {
    binder: &'a Binder<'a>,
    config: &'a salite_common::Config,

    /// contemporary storage for type parameters
    type_vars: HashMap<String, Type>,
    expected_type: Option<Type>,
}

impl<'a> Analyzer<'a> {
    pub fn analyze(
        binder: &'a Binder,
        config: &'a salite_common::Config,
        file: &'a hir::File,
    ) -> Result<(), AnalyzeError> {
        let mut analyzer = Analyzer {
            binder,
            config,
            type_vars: HashMap::new(),
            expected_type: None,
        };
        file.block.validate(&mut analyzer)?;
        Ok(())
    }
}

impl<'a> Analyzer<'a> {
    pub fn table_key_description(&self, key: &types::TableFieldKey) -> String {
        match key {
            types::TableFieldKey::Name(str, _) => str.to_string(),
            types::TableFieldKey::Computed(typ) => self.type_description(typ),
            types::TableFieldKey::None(id) => format!("[array {}]", id),
        }
    }

    pub fn table_description(&self, tbl: &types::Table) -> String {
        // that's very long, but the maximum of table entries is about 5?
        let mut entry_result = Vec::new();
        let limited_entries = tbl.entries.pick_limit(5);

        if limited_entries.is_empty() {
            return String::from("{{}}");
        }

        for (key, value) in limited_entries {
            entry_result.push(format!(
                "{}{}",
                {
                    let result = self.table_key_description(key);
                    if result.is_empty() {
                        String::new()
                    } else {
                        format!("{}: ", result)
                    }
                },
                self.type_description(value)
            ));
        }

        if let Some(ref meta) = tbl.metatable {
            entry_result.push(self.table_description(meta));
        }

        if tbl.entries.len() > 5 {
            entry_result.push("..".to_string());
        }

        format!("{{ {} }}", entry_result.join(", "))
    }

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
                types::LiteralKind::Any => "any",
                types::LiteralKind::Bool => "bool",
                types::LiteralKind::Number => "number",
                types::LiteralKind::Nil => "nil",
                types::LiteralKind::String => "string",
                types::LiteralKind::Unknown => "unknown",
                types::LiteralKind::Void => "void",
            }
            .to_string(),
            Type::Table(tbl) => self.table_description(tbl),
            Type::Function(info) => {
                let mut params = Vec::new();
                for param in info.parameters.iter() {
                    let name = param
                        .name
                        .clone()
                        .map(|v| format!("{}: ", v))
                        .unwrap_or(String::new());

                    let typ = self.type_description(&param.typ);
                    params.push(format!("{}{}", name, typ));
                }
                format!(
                    "({}) -> {}",
                    params.join(","),
                    self.type_description(&info.return_type)
                )
            }
            Type::Procrastinated(..) | Type::CallProcrastinated(..) => {
                panic!("This type is procrastinating!")
            }
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
            if symbol.type_parameters.is_some() {
                break;
            }

            typ = self
                .binder
                .symbols
                .get(node.symbol)
                .unwrap()
                .typ
                .clone()
                .unwrap_or(types::makers::any(typ.span()));
        }
        typ
    }

    pub fn solve_table_recursive(
        &mut self,
        node: &types::Table,
    ) -> Result<types::Table, AnalyzeError> {
        let mut entries = Dictionary::new();
        for (key, value) in node.entries.iter() {
            entries.insert(key.clone(), self.solve_type_recursive(value)?);
        }
        let mut metatable = None;
        if let Some(met) = &node.metatable {
            metatable = Some(Box::new(self.solve_table_recursive(met)?));
        }
        Ok(types::Table {
            span: node.span,
            entries,
            is_metatable: node.is_metatable,
            metatable,
        })
    }

    pub fn solve_type_recursive(&mut self, typ: &types::Type) -> Result<Type, AnalyzeError> {
        match typ {
            // TODO(memothelemo): Do something with other literal types
            Type::Literal(node) => Ok(typ.clone()),
            Type::Ref(_) => self.solve_type_ref(typ),
            Type::Tuple(node) => {
                let mut solved_members = Vec::new();
                for member in node.members.iter() {
                    solved_members.push(self.solve_type_recursive(member)?);
                }
                Ok(types::Type::Tuple(types::TupleType {
                    span: node.span,
                    members: solved_members,
                }))
            }
            Type::Table(node) => Ok(Type::Table(self.solve_table_recursive(node)?)),
            Type::Function(node) => {
                let mut parameters = Vec::new();
                for param in node.parameters.iter() {
                    parameters.push(types::FunctionParameter {
                        optional: param.optional,
                        span: param.span,
                        name: param.name.clone(),
                        typ: self.solve_type_recursive(&param.typ)?,
                    });
                }
                let mut varidiac_param = None;
                if let Some(param) = &node.varidiac_param {
                    varidiac_param = Some(types::VaridiacParameter {
                        span: param.span,
                        typ: Box::new(self.solve_type_recursive(&param.typ)?),
                    });
                }
                Ok(Type::Function(types::FunctionType {
                    span: node.span,
                    parameters,
                    varidiac_param,
                    return_type: Box::new(self.solve_type_recursive(&node.return_type)?),
                }))
            }
            Type::Procrastinated(id, ..) => {
                self.solve_type(&self.binder.symbols.get(*id).unwrap().typ.clone().unwrap())
            }
            Type::CallProcrastinated(id, ..) => {
                match &self.binder.symbols.get(*id).unwrap().typ.as_ref().unwrap() {
                    Type::Function(info) => self.solve_type(&info.return_type.clone()),
                    Type::Table(tbl) if tbl.metatable.is_some() => {
                        let metatable = tbl.metatable.as_ref().unwrap();

                        // don't worry, it will ignore the span comparison.
                        let value = match metatable.entries.get(&types::TableFieldKey::Name(
                            "__call".to_string(),
                            Span::invalid(),
                        )) {
                            Some(value) => value,
                            None => unreachable!(),
                        };

                        // check if it is a function, meh!
                        let value = self.solve_type(value)?;
                        match value {
                            Type::Function(info) => self.solve_type(&info.return_type),
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn solve_type_ref(&mut self, typ: &types::Type) -> Result<Type, AnalyzeError> {
        let sample = match typ {
            types::Type::Ref(refer) => refer,
            _ => panic!("Expected type reference"),
        };

        // straight forward thing to do
        if let Some(typ) = self.type_vars.get(&sample.name) {
            return Ok(typ.clone());
        }

        // get the type parameters
        let symbol = self.binder.symbols.get(sample.symbol).unwrap();

        #[allow(clippy::or_fun_call)]
        if symbol.type_parameters.is_none() {
            return Ok(symbol.typ.clone().unwrap_or(types::makers::any(typ.span())));
        }

        let parameters = symbol.type_parameters.as_ref().unwrap();
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
                .unwrap_or(types::makers::any(param.span));

            if let Some(arg) = arg {
                self.check_lr_types(arg, &explicit_param, arg.span())?;
                self.type_vars.insert(param.name.clone(), arg.clone());
            } else {
                return Err(AnalyzeError::MissingTypeArgument {
                    span: typ.span(),
                    idx: idx + 1,
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
}
