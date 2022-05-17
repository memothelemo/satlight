use std::sync::Arc;

use crate::{
    hir,
    types::{variants, Type, TypeTrait},
    utils, ModuleContext,
};
use salite_ast::Span;

mod checker;
mod errors;
mod expressions;
mod statements;
mod typess;

pub use checker::*;
pub use errors::*;

pub trait Validate<'a, 'b> {
    type Output;

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError>;
}

impl<'a, 'b> Validate<'a, 'b> for hir::Block<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
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

pub type AnalyzeResult<T = ()> = Result<T, AnalyzeError>;

#[derive(Debug)]
pub struct Analyzer<'a, 'b> {
    pub ctx: Arc<ModuleContext<'a, 'b>>,
    pub expected_type: Option<Type>,
}

impl<'a, 'b> Analyzer<'a, 'b> {
    pub fn analyze(ctx: Arc<ModuleContext<'a, 'b>>, file: &hir::File<'b>) -> AnalyzeResult {
        let mut analyzer = Self {
            ctx,
            expected_type: None,
        };
        file.block.validate(&mut analyzer)
    }
}
