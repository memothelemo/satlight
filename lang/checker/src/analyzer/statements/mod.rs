use super::*;

mod assign;
mod last;
mod typ;

pub use assign::*;
pub use last::*;
pub use typ::*;

impl<'a> Validate<'a> for hir::Stmt<'a> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Stmt::LocalAssign(node) => node.validate(analyzer),
            hir::Stmt::TypeDeclaration(node) => node.validate(analyzer),
            hir::Stmt::Call(node) => node.validate(analyzer),
            hir::Stmt::Library(node) => node.validate(analyzer),
        }
    }
}
