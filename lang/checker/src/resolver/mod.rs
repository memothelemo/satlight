use std::borrow::Borrow;

use crate::{
    hir,
    types::{self, variants as type_variants, Type, TypeTrait},
    utils, EnvContext, ModuleContext, ModuleResult, Symbol,
};
use id_arena::Id;
use salite_common::{dictionary::Dictionary, memory::SafePtr};

mod errors;
mod nodes;

pub use errors::*;
pub use nodes::*;

pub type ResolveResult<T = ()> = Result<T, ResolveError>;

pub trait ResolveMut<'a, 'b> {
    type Output: 'b;

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output>;
}

impl<'a, 'b> ResolveMut<'a, 'b> for hir::File<'b> {
    type Output = ();

    fn resolve(&mut self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output> {
        self.block.resolve(resolver)
    }
}

pub trait Resolve<'a, 'b> {
    type Output: 'b;

    fn resolve(&self, resolver: &mut Resolver<'a, 'b>) -> ResolveResult<Self::Output>;
}

// Gets rid of unresolved types before typechecking
// and tries to prevent recursion types later on.
#[derive(Debug)]
pub struct Resolver<'a, 'b> {
    pub(crate) ctx: SafePtr<ModuleContext<'a, 'b>>,
    pub(crate) env_ctx: SafePtr<EnvContext<'a, 'b>>,
    pub(crate) type_vars: Dictionary<String, Type>,
    pub(crate) type_stack: Vec<Id<Symbol>>,
}

impl<'a, 'b> Resolver<'a, 'b> {
    pub fn resolve_table(
        &mut self,
        node: &type_variants::Table,
    ) -> Result<type_variants::Table, ResolveError> {
        let mut entries = Dictionary::new();
        for (key, value) in node.entries.iter() {
            entries.insert(key.clone(), self.resolve_type_inner(value)?);
        }
        let mut metatable = None;
        if let Some(met) = &node.metatable {
            metatable = Some(Box::new(self.resolve_table(met)?));
        }
        Ok(type_variants::Table {
            span: node.span,
            entries,
            is_metatable: node.is_metatable,
            metatable,
        })
    }

    pub fn resolve_type(&mut self, typ: &types::Type) -> ResolveResult<Type> {
        let res = self.resolve_type_inner(typ)?;
        self.type_stack.clear();
        Ok(res)
    }

    pub fn resolve_type_inner(&mut self, typ: &types::Type) -> Result<Type, ResolveError> {
        match typ {
            // TODO(memothelemo): Do something with other literal types
            Type::Literal(..) | Type::Any(..) | Type::Recursive(..) | Type::Unknown(..) => {
                Ok(typ.clone())
            }
            Type::Reference(r) => self.resolve_type_ref(r),
            Type::Tuple(node) => {
                let mut solved_members = Vec::new();
                for member in node.members.iter() {
                    solved_members.push(self.resolve_type_inner(member)?);
                }
                Ok(types::Type::Tuple(type_variants::Tuple {
                    span: node.span,
                    members: solved_members,
                }))
            }
            Type::Table(node) => Ok(Type::Table(self.resolve_table(node)?)),
            Type::Function(node) => {
                let mut parameters = Vec::new();
                for param in node.parameters.iter() {
                    parameters.push(type_variants::FunctionParameter {
                        optional: param.optional,
                        span: param.span,
                        name: param.name.clone(),
                        typ: self.resolve_type_inner(&param.typ)?,
                    });
                }
                let mut varidiac_param = None;
                if let Some(param) = &node.varidiac_param {
                    varidiac_param = Some(type_variants::VaridiacParameter {
                        span: param.span,
                        typ: Box::new(self.resolve_type_inner(&param.typ)?),
                    });
                }
                Ok(Type::Function(type_variants::Function {
                    span: node.span,
                    parameters,
                    varidiac_param,
                    return_type: Box::new(self.resolve_type_inner(&node.return_type)?),
                }))
            }
            Type::Unresolved(info) => self.resolve_type_inner(
                &self
                    .ctx
                    .symbols
                    .get(info.symbol)
                    .unwrap()
                    .get_type()
                    .expect("Expected type")
                    .clone(),
            ),
            // Type::CallProcrastinated(id, ..) => {
            //     match &self.binder.symbols.get(*id).unwrap().typ.as_ref().unwrap() {
            //         Type::Function(info) => self.solve_type(&info.return_type.clone()),
            //         Type::Table(tbl) if tbl.metatable.is_some() => {
            //             let metatable = tbl.metatable.as_ref().unwrap();

            //             // don't worry, it will ignore the span comparison.
            //             let value = match metatable.entries.get(&types::TableFieldKey::Name(
            //                 "__call".to_string(),
            //                 Span::invalid(),
            //             )) {
            //                 Some(value) => value,
            //                 None => unreachable!(),
            //             };

            //             // check if it is a function, meh!
            //             let value = self.solve_type(value)?;
            //             match value {
            //                 Type::Function(info) => self.solve_type(&info.return_type),
            //                 _ => unreachable!(),
            //             }
            //         }
            //         _ => unreachable!(),
            //     }
            // }
            Type::Intersection(node) => {
                // table mergies
                let mut members = Vec::new();
                let mut table_mergies = Vec::new();

                for member in node.members.iter() {
                    let member = self.resolve_type_inner(member)?;
                    if let Type::Table(tbl) = member {
                        table_mergies.push(tbl);
                    } else {
                        members.push(member);
                    }
                }

                // combine both together to form merged table
                let table_length = table_mergies.len();
                match table_length.cmp(&1) {
                    std::cmp::Ordering::Equal => {
                        let mut table_mergies = table_mergies.drain(..);
                        members.push(Type::Table(table_mergies.next().unwrap()));
                    }
                    std::cmp::Ordering::Greater => {
                        let mut table_mergies = table_mergies.drain(..);
                        let mut base_table = table_mergies.next().unwrap();

                        for tbl in table_mergies {
                            base_table.combine(&tbl, node.span);
                        }

                        members.push(Type::Table(base_table));
                    }
                    _ => {}
                };

                if table_length == node.members.len() {
                    Ok(members.last().unwrap().clone())
                } else {
                    Ok(Type::Intersection(type_variants::Intersection {
                        span: node.span,
                        members,
                    }))
                }
            }
            Type::Union(node) => {
                let mut members = Vec::new();
                for member in node.members.iter() {
                    members.push(self.resolve_type_inner(member)?)
                }
                Ok(Type::Union(type_variants::Union {
                    span: node.span,
                    members,
                }))
            }
        }
    }

    pub fn resolve_type_ref(&mut self, typ: &type_variants::Reference) -> ResolveResult<Type> {
        // type parameters...
        if let Some(typ) = self.type_vars.get(&typ.name) {
            // TOOD(memothelemo): make sure it doesn't have parameters?
            return Ok(typ.clone());
        }

        // get the type parameters
        let symbol = self.ctx.symbols.get(typ.symbol).unwrap();
        self.type_stack.push(typ.symbol);

        let parameters = match &symbol.kind {
            crate::SymbolKind::TypeAlias(typ) => {
                if typ.parameters.is_none() {
                    self.type_stack.pop();
                    return Ok(typ.typ.clone());
                }
                typ.parameters.as_ref().unwrap()
            }
            _ => {
                //eprintln!("Invalid type reference symbol! {:#?}", symbol);
                self.type_stack.pop();
                return Ok(types::makers::any(typ.span()));
            }
        };

        if typ.arguments.is_none() && !parameters.is_empty() {
            self.type_stack.pop();
            return Err(ResolveError::NoArguments {
                span: typ.span(),
                base: typ.name.to_string(),
            });
        }

        let arguments = typ.arguments.as_ref().unwrap();

        #[allow(clippy::or_fun_call)]
        for (idx, param) in parameters.iter().enumerate() {
            let arg = arguments.get(idx).or(param.default.as_ref());
            let explicit_param = param
                .explicit
                .clone()
                .unwrap_or(types::makers::any(param.span));

            if let Some(arg) = arg {
                self.type_vars.insert(param.name.clone(), arg.clone());
            } else {
                return Err(ResolveError::MissingTypeArgument {
                    span: typ.span(),
                    idx: idx + 1,
                    expected_type: utils::type_description(&self.ctx, &explicit_param),
                });
            }
        }

        self.type_stack.pop();

        let typ = symbol.get_type().expect("Expected type").clone();
        self.resolve_type_inner(&typ)
    }
}

impl<'a, 'b> Resolver<'a, 'b> {
    pub fn from_result(
        result: &mut ModuleResult<'a, 'b>,
        env_ctx: SafePtr<EnvContext<'a, 'b>>,
    ) -> ResolveResult {
        let mut resolver = unsafe {
            Self::from_ctx_ptr(
                SafePtr::from_ptr((result.ctx.borrow() as *const ModuleContext<'a, 'b>).as_mut()),
                env_ctx,
            )
        };
        result.file.resolve(&mut resolver)
    }

    // It is unsafe to use a null pointer when I try
    // to implement `Default` trait.

    /// # Safety
    /// Make sure your pointer object is not dropped when
    /// you create a Resolver object.
    ///
    /// Otherwise you'll face `segmentation fault` errors, which
    /// you don't want in Rust.
    pub unsafe fn from_ctx_ptr(
        ctx: SafePtr<ModuleContext<'a, 'b>>,
        env_ctx: SafePtr<EnvContext<'a, 'b>>,
    ) -> Self {
        Self {
            ctx,
            env_ctx,
            type_stack: Vec::new(),
            type_vars: Dictionary::new(),
        }
    }
}
