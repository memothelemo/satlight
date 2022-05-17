use super::*;
use crate::types::Type;

pub(crate) fn transform_function_body<'a, 'b>(
    tfmr: &mut Transformer<'a, 'b>,
    body: &'b ast::FunctionBody,
    span: Span,
    allocated_id: Id<&'b dyn ast::Node>,
) -> hir::Function<'b> {
    let mut parameters = Vec::new();
    let mut defaults = Vec::new();

    tfmr.push_scope(ScopeKind::Function);

    let expected_type = body
        .return_type()
        .as_ref()
        .map(|return_type| return_type.transform(tfmr));

    let mut scope = tfmr.current_scope_mut();
    scope.expected_type = expected_type.clone();

    for param in body.params().iter() {
        let name = param.name.ty().as_name();
        let typ = param
            .explicit_type
            .as_ref()
            .map(|v| v.transform(tfmr))
            .unwrap_or(types::makers::any(param.span));

        defaults.push(param.default.as_ref().map(|v| v.transform(tfmr)));

        tfmr.insert_variable(
            &name,
            SymbolKind::FunctionParameter(name.to_string(), typ.clone(), param.optional),
            Some(param.span),
        );

        parameters.push(variants::FunctionParameter {
            optional: param.optional,
            span: param.span,
            name,
            typ,
        });
    }

    let mut varidiac_param = None;

    // TODO(memothelemo): Add support for varidiac parameters
    if let Some(varidiac) = body.varidiac() {
        varidiac_param = Some(variants::VaridiacParameter {
            span: varidiac.span,
            typ: Box::new(
                varidiac
                    .typ
                    .as_ref()
                    .map(|v| v.transform(tfmr))
                    .unwrap_or(types::makers::any(varidiac.span)),
            ),
        });
    }

    let block = body.block().transform(tfmr);
    let expr = hir::Function {
        span,
        defaults,
        typ: Type::Function(variants::Function {
            span,
            parameters,
            varidiac_param,
            return_type: Box::new(expected_type.unwrap_or(block.actual_type.clone())),
        }),
        block,
        node_id: allocated_id,
    };

    tfmr.pop_scope();

    expr
}

impl<'a, 'b> Transform<'a, 'b> for ast::FunctionExpr {
    type Output = hir::Expr<'b>;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let id = tfmr.ctx.nodes.alloc(self);
        let function = transform_function_body(tfmr, self.body(), self.span(), id);
        hir::Expr::Function(function)
    }
}
