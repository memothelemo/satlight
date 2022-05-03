use super::*;

mod assign;
mod last;
mod typ;

pub use assign::*;
pub use last::*;
pub use typ::*;

impl Validate for hir::Stmt {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Stmt::LocalAssign(node) => node.validate(analyzer),
            hir::Stmt::TypeDeclaration(node) => node.validate(analyzer),
        }
    }
}
