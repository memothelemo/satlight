#![allow(unused)]
mod errors;

use super::*;
use crate::binder::Binder;
use crate::types::Type;
use crate::{hir, types as ctypes};
pub use errors::*;
use salite_ast::Span;
use std::borrow::Borrow;
use std::collections::HashMap;

pub trait Validate<'a> {
    type Output;

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError>;
}

#[derive(Debug)]
pub struct Analyzer<'a> {
    binder: &'a Binder<'a>,
    config: &'a salite_common::Config,

    /// contemporary storage for type parameters
    type_vars: HashMap<String, Type>,
}

impl<'a> Analyzer<'a> {
    pub fn analyze(
        binder: &'a Binder,
        config: &'a salite_common::Config,
        file: &'a hir::File,
    ) -> Result<(), AnalyzeError> {
        todo!()
    }
}
