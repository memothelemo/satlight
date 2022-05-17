use super::*;

mod assign;
mod last;
mod typ;

pub use assign::*;
pub use last::*;
pub use typ::*;

impl<'a, 'b> Validate<'a, 'b> for hir::Stmt<'b> {
    type Output = ();

    fn validate(&self, analyzer: &mut Analyzer<'a, 'b>) -> Result<Self::Output, AnalyzeError> {
        match self {
            hir::Stmt::LocalAssign(node) => node.validate(analyzer),
            hir::Stmt::TypeDeclaration(node) => node.validate(analyzer),
            hir::Stmt::Call(node) => node.validate(analyzer),
            hir::Stmt::Library(..) => todo!(),
        }
    }
}
