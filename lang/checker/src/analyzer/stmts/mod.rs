use super::*;

mod assign;
mod last;

pub use assign::*;
pub use last::*;

impl Validate for hir::Stmt {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Stmt::LocalAssign(node) => node.validate(analyzer),
        }
    }
}
