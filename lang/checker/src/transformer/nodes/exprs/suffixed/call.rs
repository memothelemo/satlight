use super::*;
use crate::literal;

#[macro_export]
macro_rules! invalid_lib_use {
    ($self:expr, $span:expr, $lib:expr) => {
        $self.ctx.diagnostics.push(Diagnostic::InvalidLibraryUse {
            lib: $lib.to_string(),
            span: $span,
        });
    };
}

pub(crate) fn visit_call_expr_inner<'a, 'b>(
    tfmr: &mut Transformer<'a, 'b>,
    node: &'b ast::Suffixed,
    args: &'b ast::Args,
) -> hir::Expr<'b> {
    let mut arguments = Vec::new();
    let base = node.base().transform(tfmr);

    match args {
        ast::Args::ExprList(list) => {
            for expr in list.iter() {
                arguments.push(expr.transform(tfmr));
            }
        }
        ast::Args::Table(arg) => {
            arguments.push(arg.transform(tfmr));
        }
        ast::Args::Str(arg) => arguments.push(literal!(arg, tfmr, node, string)),
    };

    hir::Expr::Suffixed(hir::Suffixed {
        span: node.span(),
        base: Box::new(base),
        kind: hir::SuffixKind::Call(arguments),
    })
}
